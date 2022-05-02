use derive_more::Constructor;
use std::path::{Path, PathBuf};

use crate::error::MdmgError;
use crate::Result;
use std::fs::read_to_string;

pub trait GeneratedFileRepository {
    fn resolve(&self, file_name: &Path) -> Result<String>;
}

#[derive(Debug, Constructor)]
pub struct FSGeneratedFileRepository {
    base: PathBuf,
}

impl GeneratedFileRepository for FSGeneratedFileRepository {
    fn resolve(&self, path: &Path) -> Result<String> {
        let file_path = self.base.join(path);
        read_to_string(self.base.join(path))
            .map_err(|_| {
                MdmgError::GeneratedFileIsNotFound(file_path.to_string_lossy().to_string())
            })
    }
}

#[cfg(not(tarpaulin_include))]
#[cfg(test)]
mod tests {
    use super::{FSGeneratedFileRepository, GeneratedFileRepository};
    use std::{path::PathBuf, str::FromStr};

    #[test]
    #[cfg_attr(not(feature = "fs-test"), ignore)]
    fn test_fs_generated_file_repository_is_not_found() {
        let repository = FSGeneratedFileRepository::new(PathBuf::from_str(".").unwrap());
        assert!(repository
            .resolve(&PathBuf::from_str("support/foobar").unwrap())
            .is_err())
    }

    #[test]
    #[cfg_attr(not(feature = "fs-test"), ignore)]
    fn test_fs_generated_file_repository_is_ok() {
        let repository = FSGeneratedFileRepository::new(PathBuf::from_str(".").unwrap());
        assert!(repository
            .resolve(&PathBuf::from_str("./src/main.rs").unwrap())
            .is_ok())
    }
}
