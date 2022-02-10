use crate::scaffold::Scaffold;
use crate::MdmgError;
use crate::Result;

use std::fs::{read_dir, remove_dir, remove_file};
use std::path::Path;
use std::sync::Arc;

use derive_more::Constructor;
use yansi::Paint;

pub trait DeleteExecutorDeps {
    fn delete_file(&self, path: &Path) -> Result<()>;
    fn delete_directory(&self, path: &Path) -> Result<()>;
    fn is_empty_directory(&self, directory_path: &Path) -> bool;
}

pub trait DeleteExecutor {
    fn execute(&self, scaffold: &Scaffold) -> Result<()>;
}

#[derive(Clone, Copy, Constructor)]
pub struct FSDeleteExecutorDeps {}

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

#[cfg(not(tarpaulin_include))]
#[cfg(test)]
mod tests {
    use super::{DeleteExecutor, DeleteExecutorDeps, FSDeleteExecutor, FSDeleteExecutorDeps};

    use crate::error::MdmgError;
    use crate::scaffold::Scaffold;

    use std::cell::Cell;
    use std::fs::{create_dir, write, remove_dir, remove_file};
    use std::path::Path;
    use std::sync::Arc;

    #[test]
    pub fn delete_only_file_when_pending_scaffold_only() {
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
        let actual = executor.execute(&Scaffold::Pending {
            file_name: "foo/bar.md".to_string(),
        });

        assert!(actual.is_ok());
        assert_eq!(
            stub_deps.deleted_file_path.take(),
            Some("foo/bar.md".to_string())
        );
        assert_eq!(stub_deps.deleted_directory_path.take(), None)
    }

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

    #[test]
    #[cfg_attr(not(feature="fs-test"), ignore)]
    pub fn fs_delete_executor_deps_delete_file_can_delete_file() {
        let file_path = Path::new("./support/fs_delete_executor_deps_delete_file_can_delete_file/dummy.txt");
        assert!(create_dir(&file_path.parent().unwrap()).is_ok());
        assert!(write(file_path, "dummy").is_ok());

        let deps = FSDeleteExecutorDeps::new();
        assert!(deps.delete_file(file_path).is_ok());
        assert!(!file_path.exists());
        assert!(remove_dir(file_path.parent().unwrap()).is_ok());
    }

    #[test]
    #[cfg_attr(not(feature="fs-test"), ignore)]
    pub fn fs_delete_executor_deps_delete_file_failed_when_not_exist_file() {
        let file_path = Path::new("./support/fs_delete_executor_deps_delete_file_failed_when_not_exist_file/dummy.txt");

        let deps = FSDeleteExecutorDeps::new();
        let actual = deps.delete_file(file_path);
        assert!(actual.is_err());
        assert!(!file_path.exists());
    }


    #[test]
    #[cfg_attr(not(feature="fs-test"), ignore)]
    pub fn fs_delete_executor_deps_delete_directory_can_delete_directory() {
        let path = Path::new("./support/fs_delete_executor_deps_delete_directory_can_delete_directory");
        assert!(create_dir(&path).is_ok());

        let deps = FSDeleteExecutorDeps::new();
        assert!(deps.delete_directory(path).is_ok());
        assert!(!path.exists());
    }

    #[test]
    #[cfg_attr(not(feature="fs-test"), ignore)]
    pub fn fs_delete_executor_deps_delete_directory_failed_when_not_exist_file() {
        let file_path = Path::new("./support/fs_delete_executor_deps_delete_directory_failed_when_not_exist_file/");

        let deps = FSDeleteExecutorDeps::new();
        let actual = deps.delete_directory(file_path);
        assert!(actual.is_err());
        assert!(!file_path.exists());
    }

    #[test]
    #[cfg_attr(not(feature="fs-test"), ignore)]
    pub fn fs_delete_executor_deps_is_empty_directory_return_to_true() {
        let path = Path::new("./support/fs_delete_executor_deps_is_empty_directory_return_to_true/");
        assert!(create_dir(path).is_ok());

        let deps = FSDeleteExecutorDeps::new();
        assert!(deps.is_empty_directory(path));
        assert!(remove_dir(path).is_ok());
    }

    #[test]
    #[cfg_attr(not(feature="fs-test"), ignore)]
    pub fn fs_delete_executor_deps_is_empty_directory_return_to_false() {
        let path = Path::new("./support/fs_delete_executor_deps_is_empty_directory_return_to_false/");
        let file_path = path.join("dummy.txt");
        assert!(create_dir(path).is_ok());
        assert!(write(&file_path, "dummy").is_ok());

        let deps = FSDeleteExecutorDeps::new();
        assert!(!deps.is_empty_directory(path));
        assert!(remove_file(file_path).is_ok());
        assert!(remove_dir(path).is_ok());
    }
}
