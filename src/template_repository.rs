use crate::Result;
use crate::error::MdmbError;
use std::fs::read_dir;
use std::path::PathBuf;

#[derive(Debug, Eq, PartialEq, Clone, PartialOrd, Ord)]
pub struct FileName(pub String);

impl FileName {
    pub fn new<T: Into<String>>(filename: T) -> Self{
        FileName(filename.into())
    }
}

pub trait TemplateRepository {
    fn list(&self) -> Result<Vec<FileName>>;
}

pub struct FSTemplateRepository {
    path: PathBuf 
}

impl FSTemplateRepository {
    pub fn new<T: Into<PathBuf>>(path: T) -> FSTemplateRepository {
        FSTemplateRepository { path: path.into() }
    }
}

impl TemplateRepository for FSTemplateRepository {
    fn list(&self) -> Result<Vec<FileName>> {
        let dir = read_dir(&self.path)?.flatten();
        let file_vec_result = dir.map(|entry| {
            let filename_result  = entry.file_name().into_string();
            filename_result.map(|filename| {
                return FileName::new(filename);
            }).map_err(|os_string| {
                MdmbError::FileNameConvertError(os_string)
            })
        }).collect::<Result<Vec<_>>>();
        file_vec_result.map(|files| {
            let mut sorted_files = files;
            sorted_files.sort();
            sorted_files
        })
    }
}

#[cfg(feature = "integration_test")]
mod tests {
    use super::{FSTemplateRepository, TemplateRepository, FileName};

    #[test]
    pub fn test_FSTemplateRepository_list_return_to_files() {
        let repository = FSTemplateRepository::new("./support");
        let result = repository.list().expect("result is error");
        assert_eq!(
            result,
            vec![FileName::new("file1"), FileName::new("file2"), FileName::new("file3")]
       )
    }
}

