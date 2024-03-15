use itertools::{concat, Itertools};

use crate::error::MdmgError;
use crate::file::FileName;
use crate::template::Template;
use crate::Result;

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
        let local_dir = read_dir(&self.path)?.flatten();
        let xdg_dir = xdg::BaseDirectories::with_prefix("mdmg")
            .map(|x| x.list_data_files(""))
            .unwrap_or(vec![]);

        let local_dir_file_names = local_dir
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
            .collect::<Result<Vec<_>>>()
            .unwrap_or(vec![]);

        let xdg_dir_file_names: Vec<FileName> = xdg_dir.iter().fold(vec![], |acc, path| {
            let file_name_opt: Option<FileName> = path
                .file_stem()
                .and_then(|name| name.to_str())
                .and_then(|name| Some(FileName::new(name)));

            match file_name_opt {
                Some(file_name) => acc
                    .into_iter()
                    .chain(vec![file_name])
                    .collect::<Vec<FileName>>(),
                None => acc,
            }
        });
        let file_names = concat(vec![local_dir_file_names, xdg_dir_file_names]);

        Ok(file_names.into_iter().sorted().collect::<Vec<FileName>>())
    }
    fn resolve(&self, template_name: String) -> Result<Template> {
        let template_file_name = format!("{}.md", template_name);
        let local_template_path_buf = PathBuf::from(&self.path).join(&template_file_name);
        let local_template_path = if local_template_path_buf.exists() {
            Some(local_template_path_buf)
        } else {
            None
        };
        let xdg_data_dir_template_path = xdg::BaseDirectories::with_prefix("mdmg")
            .map(|x| x.find_data_file(template_file_name))
            .unwrap_or(None);
        let template_body = [local_template_path, xdg_data_dir_template_path]
            .into_iter()
            .find_map(|s| s.map(|p| read_to_string(p).ok()))
            .flatten();

        let body = template_body.ok_or(MdmgError::TemplateIsNotFound(template_name))?;

        Ok(Template::new(body.trim()))
    }
}

#[cfg(test)]
mod tests {
    use super::{FSTemplateRepository, FileName, TemplateRepository};
    use crate::template::Template;
    use std::env::{current_dir, set_var, var};

    fn with_xdg_data_path<O: FnOnce() -> ()>(closure: O) {
        let xdg_data_dir = var("XDG_DATA_HOME").unwrap_or("".to_string());
        let current = current_dir().unwrap();
        set_var("XDG_DATA_HOME", current.join("support/xdg_data_dir"));
        closure();
        set_var("XDG_DATA_HOME", xdg_data_dir);
    }

    #[test]
    #[cfg_attr(not(feature = "fs-test"), ignore)]
    pub fn test_fstemplate_repository_list_return_to_files() {
        with_xdg_data_path(|| {
            let repository =
                FSTemplateRepository::new("./support/fs_template_repository_list_test");
            let result = repository.list().expect("result is error");
            assert_eq!(
                result,
                vec![
                    FileName::new("file1"),
                    FileName::new("file2"),
                    FileName::new("file3"),
                    FileName::new("file4")
                ]
            )
        })
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

    #[test]
    #[cfg_attr(not(feature = "fs-test"), ignore)]
    pub fn test_fstemplate_repository_resolve_return_to_template_when_selected_xdg_data_dir_templates(
    ) {
        with_xdg_data_path(|| {
            let repository =
                FSTemplateRepository::new("./support/fs_template_repository_resolve_test");
            let template = repository
                .resolve("file4".to_string())
                .expect("template foobar is not found");
            assert_eq!(template, Template::new("xdg data dir"));
        })
    }
}
