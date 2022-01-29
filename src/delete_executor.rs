use crate::scaffold::Scaffold;
use crate::MdmgError;
use crate::Result;
use yansi::Paint;

use std::fs::{read_dir, remove_dir, remove_file};
use std::path::Path;
use std::sync::Arc;

pub trait DeleteExecutorDeps {
    fn delete_file(&self, path: &Path) -> Result<()>;
    fn delete_directory(&self, path: &Path) -> Result<()>;
    fn is_empty_directory(&self, directory_path: &Path) -> bool;
}

pub trait DeleteExecutor {
    fn execute(&self, scaffold: &Scaffold) -> Result<()>;
}

#[derive(Clone, Copy)]
pub struct FSDeleteExecutorDeps {}

impl FSDeleteExecutorDeps {
    pub fn new() -> Self {
        FSDeleteExecutorDeps {}
    }
}

impl DeleteExecutorDeps for FSDeleteExecutorDeps {
    fn delete_file(&self, path: &Path) -> Result<()> {
        remove_file(path)
            .map_err(|_| MdmgError::FailedDeleteFile(path.to_str().unwrap().to_string()))
    }
    fn delete_directory(&self, path: &Path) -> Result<()> {
        remove_dir(path)
            .map_err(|_| MdmgError::FailedRemoveParentDirectory(path.to_str().unwrap().to_string()))
    }
    fn is_empty_directory(&self, directory_path: &Path) -> bool {
        read_dir(directory_path).map_or(false, |read_dir| read_dir.into_iter().count() == 0)
    }
}

#[derive(Debug, Clone)]
pub struct FSDeleteExecutor<T: DeleteExecutorDeps> {
    deps: Arc<T>,
}

impl<T: DeleteExecutorDeps> FSDeleteExecutor<T> {
    pub fn new(deps: Arc<T>) -> Self {
        FSDeleteExecutor { deps }
    }
}

impl<T: DeleteExecutorDeps> DeleteExecutor for FSDeleteExecutor<T> {
    fn execute(&self, scaffold: &Scaffold) -> Result<()> {
        let file_name = match scaffold {
            Scaffold::Complete {
                file_name,
                file_body: _,
            } => file_name,
            Scaffold::Pending { file_name } => file_name,
        };
        let path = Path::new(file_name);

        self.deps.delete_file(path)?;
        println!("{} {}", Paint::green("Deleted"), file_name);

        let parent_path = &path
            .parent()
            .ok_or_else(|| MdmgError::ParentDirectoryIsNotFound(file_name.clone()))?;

        if self.deps.is_empty_directory(parent_path) {
            self.deps.delete_directory(parent_path)?;
            println!(
                "{} {}",
                Paint::green("Deleted empty directory"),
                parent_path.to_string_lossy()
            );
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::{DeleteExecutor, DeleteExecutorDeps, FSDeleteExecutor};

    use crate::error::MdmgError;
    use crate::scaffold::Scaffold;

    use std::cell::Cell;
    use std::path::Path;
    use std::sync::Arc;

    #[test]
    pub fn delete_only_file() {
        #[derive(Default)]
        struct StubDeleteExecutorDeps {
            pub deleted_file_path: Cell<Option<String>>,
            pub deleted_directory_path: Cell<Option<String>>,
        }
        impl DeleteExecutorDeps for StubDeleteExecutorDeps {
            fn delete_file(&self, path: &std::path::Path) -> crate::Result<()> {
                self.deleted_file_path
                    .replace(path.to_str().map(|s| s.to_string()));
                Ok(())
            }
            fn delete_directory(&self, path: &Path) -> crate::Result<()> {
                self.deleted_directory_path
                    .replace(path.to_str().map(|s| s.to_string()));
                Ok(())
            }
            fn is_empty_directory(&self, _directory_path: &Path) -> bool {
                false
            }
        }

        let stub_deps = Arc::new(StubDeleteExecutorDeps::default());
        let executor = FSDeleteExecutor::new(stub_deps.clone());
        let actual = executor.execute(&Scaffold::Complete {
            file_name: "foo/bar.md".to_string(),
            file_body: String::default(),
        });

        assert!(actual.is_ok());
        assert_eq!(
            stub_deps.deleted_file_path.take(),
            Some("foo/bar.md".to_string())
        );
        assert_eq!(stub_deps.deleted_directory_path.take(), None)
    }

    #[test]
    pub fn delete_file_and_parent_directory() {
        #[derive(Default)]
        struct StubDeleteExecutorDeps {
            pub deleted_file_path: Cell<Option<String>>,
            pub deleted_directory_path: Cell<Option<String>>,
        }
        impl DeleteExecutorDeps for StubDeleteExecutorDeps {
            fn delete_file(&self, path: &std::path::Path) -> crate::Result<()> {
                self.deleted_file_path
                    .replace(path.to_str().map(|s| s.to_string()));
                Ok(())
            }
            fn delete_directory(&self, path: &Path) -> crate::Result<()> {
                self.deleted_directory_path
                    .replace(path.to_str().map(|s| s.to_string()));
                Ok(())
            }
            fn is_empty_directory(&self, _directory_path: &Path) -> bool {
                true
            }
        }

        let stub_deps = Arc::new(StubDeleteExecutorDeps::default());
        let executor = FSDeleteExecutor::new(stub_deps.clone());
        let actual = executor.execute(&Scaffold::Complete {
            file_name: "foo/bar.md".to_string(),
            file_body: String::default(),
        });

        assert!(actual.is_ok());
        assert_eq!(
            stub_deps.deleted_file_path.take(),
            Some("foo/bar.md".to_string())
        );
        assert_eq!(
            stub_deps.deleted_directory_path.take(),
            Some("foo".to_string())
        )
    }

    #[test]
    pub fn when_failed_delete_file() {
        #[derive(Default)]
        struct StubDeleteExecutorDeps {
            pub deleted_file_path: Cell<Option<String>>,
            pub deleted_directory_path: Cell<Option<String>>,
        }
        impl DeleteExecutorDeps for StubDeleteExecutorDeps {
            fn delete_file(&self, path: &std::path::Path) -> crate::Result<()> {
                Err(MdmgError::FailedDeleteFile(
                    path.to_string_lossy().to_string(),
                ))
            }
            fn delete_directory(&self, path: &Path) -> crate::Result<()> {
                self.deleted_directory_path
                    .replace(path.to_str().map(|s| s.to_string()));
                Ok(())
            }
            fn is_empty_directory(&self, _directory_path: &Path) -> bool {
                false
            }
        }

        let stub_deps = Arc::new(StubDeleteExecutorDeps::default());
        let executor = FSDeleteExecutor::new(stub_deps.clone());
        let actual = executor.execute(&Scaffold::Complete {
            file_name: "foo/bar.md".to_string(),
            file_body: String::default(),
        });

        assert!(actual.is_err());
        assert_eq!(stub_deps.deleted_file_path.take(), None);
        assert_eq!(stub_deps.deleted_directory_path.take(), None);
    }

    #[test]
    pub fn when_failed_delete_parent_delete_directory() {
        #[derive(Default)]
        struct StubDeleteExecutorDeps {
            pub deleted_file_path: Cell<Option<String>>,
            pub deleted_directory_path: Cell<Option<String>>,
        }
        impl DeleteExecutorDeps for StubDeleteExecutorDeps {
            fn delete_file(&self, path: &std::path::Path) -> crate::Result<()> {
                self.deleted_file_path
                    .replace(Some(path.to_string_lossy().to_string()));
                Ok(())
            }
            fn delete_directory(&self, path: &Path) -> crate::Result<()> {
                Err(MdmgError::FailedRemoveParentDirectory(
                    path.to_string_lossy().to_string(),
                ))
            }
            fn is_empty_directory(&self, _directory_path: &Path) -> bool {
                true
            }
        }

        let stub_deps = Arc::new(StubDeleteExecutorDeps::default());
        let executor = FSDeleteExecutor::new(stub_deps.clone());
        let actual = executor.execute(&Scaffold::Complete {
            file_name: "foo/bar.md".to_string(),
            file_body: String::default(),
        });

        assert!(actual.is_err());
        assert_eq!(
            stub_deps.deleted_file_path.take(),
            Some("foo/bar.md".to_string())
        );
        assert_eq!(stub_deps.deleted_directory_path.take(), None);
    }
}
