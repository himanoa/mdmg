use crate::error::MdmgError;
use crate::template::Template;
use crate::Result;
use crate::file::FileName;

use std::fs::{read_dir, read_to_string};
use std::path::{Path, PathBuf};


pub trait TemplateRepository {
    fn list(&self) -> Result<Vec<FileName>>;
    fn resolve(&self, template_name: String) -> Result<Template>;
}

pub struct FSTemplateRepository {
    path: PathBuf,
}

impl FSTemplateRepository {
    pub fn new<T: Into<PathBuf>>(path: T) -> FSTemplateRepository {
        FSTemplateRepository { path: path.into() }
    }
}

impl TemplateRepository for FSTemplateRepository {
    fn list(&self) -> Result<Vec<FileName>> {
        let dir = read_dir(&self.path)?.flatten();
        let file_vec_result = dir
            .map(|entry| {
                let filename_result = entry.file_name().into_string();
                filename_result
                    .map(|filename| {
                        let path = Path::new(&filename);
                        match path.file_stem() {
                            Some(name) => FileName::new(name.to_string_lossy()),
                            None => FileName::new(filename),
                        }
                    })
                    .map_err(MdmgError::FileNameConvertError)
            })
            .collect::<Result<Vec<_>>>();
        file_vec_result.map(|files| {
            let mut sorted_files = files;
            sorted_files.sort();
            sorted_files
        })
    }
    fn resolve(&self, template_name: String) -> Result<Template> {
        let templates_path = PathBuf::from(&self.path).join(format!("{}.md", template_name));
        let body = read_to_string(templates_path)
            .map_err(|_| MdmgError::TemplateIsNotFound(template_name))?;
        Ok(Template::new(body.trim()))
    }
}

#[cfg(test)]
mod tests {
    use super::{FSTemplateRepository, FileName, TemplateRepository};

    use crate::template::Template;

    #[test]
    #[cfg_attr(not(feature = "fs-test"), ignore)]
    pub fn test_fstemplate_repository_list_return_to_files() {
        let repository = FSTemplateRepository::new("./support/fs_template_repository_list_test");
        let result = repository.list().expect("result is error");
        assert_eq!(
            result,
            vec![
                FileName::new("file1"),
                FileName::new("file2"),
                FileName::new("file3")
            ]
        )
    }

    #[test]
    #[cfg_attr(not(feature = "fs-test"), ignore)]
    pub fn test_fstemplate_repository_resolve_return_to_template_not_found() {
        let repository = FSTemplateRepository::new("./support/fs_template_repository_resolve_test");
        let err = repository.resolve("not_found".to_string()).is_err();
        assert!(err)
    }

    #[test]
    #[cfg_attr(not(feature = "fs-test"), ignore)]
    pub fn test_fstemplate_repository_resolve_return_to_template() {
        let repository = FSTemplateRepository::new("./support/fs_template_repository_resolve_test");
        let template = repository
            .resolve("foobar".to_string())
            .expect("template foobar is not found");
        assert_eq!(template, Template::new("testing"));
    }
}
