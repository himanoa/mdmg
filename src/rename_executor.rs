use crate::scaffold::Scaffold;
use crate::Result;
use crate::{error::MdmgError, logger::Logger};

use derive_more::{Constructor, Display};
use inflector::Inflector;

fn rename(rename_target: &str, before_identify: &str, after_identify: &str) -> String {
    rename_target
        .replace(
            &before_identify.to_pascal_case(),
            &after_identify.to_pascal_case(),
        )
        .replace(
            &before_identify.to_camel_case(),
            &after_identify.to_camel_case(),
        )
        .replace(
            &before_identify.to_kebab_case(),
            &after_identify.to_kebab_case(),
        )
        .replace(
            &before_identify.to_snake_case(),
            &after_identify.to_snake_case(),
        )
        .to_string()
}

#[derive(Debug, Clone, Constructor, PartialEq, Eq, Default)]
pub struct ReplacementParameter {
    id: String,
    renamed_name: String,
    before_replace_body: String,
    replaced_body: String,
}

impl ReplacementParameter {
    pub fn from_scaffold(
        scaffold: &Scaffold,
        before_identify: &str,
        after_identify: &str,
    ) -> Result<ReplacementParameter> {
        let (file_name, file_body) = match scaffold {
            Scaffold::Pending { file_name } => {
                return Err(MdmgError::ReadPendingScaffoldError {
                    file_name: file_name.clone(),
                })
            }
            Scaffold::Complete {
                file_name,
                file_body,
            } => (file_name, file_body),
        };
        let renamed_file_name = rename(&file_name, before_identify, after_identify);
        let replaced_file_body = rename(&file_body, before_identify, after_identify);

        Ok(ReplacementParameter::new(
            file_name.clone(),
            renamed_file_name,
            file_body.clone(),
            replaced_file_body,
        ))
    }

    pub fn name_changed(&self) -> bool {
        self.id != self.renamed_name
    }

    pub fn body_changed(&self) -> bool {
        self.before_replace_body != self.replaced_body
    }

    pub fn all_changed(&self) -> bool {
        self.name_changed() && self.body_changed()
    }
}

#[derive(Debug, Clone, Copy, Eq, PartialEq, Display)]
pub enum ReplacementOperation {
    None,
    Rename,
    Replace,
    RenameAndReplace,
}

impl From<ReplacementParameter> for ReplacementOperation {
    fn from(params: ReplacementParameter) -> Self {
        if params.all_changed() {
            Self::RenameAndReplace
        } else if params.body_changed() {
            Self::Replace
        } else if params.name_changed() {
            Self::Rename
        } else {
            Self::None
        }
    }
}

pub trait RenameExecutor {
    fn execute(
        self,
        scaffold: &Scaffold,
        before_identify: &str,
        after_identify: &str,
    ) -> Result<()>;
}

#[cfg(not(tarpaulin_include))]
#[cfg(test)]
mod tests {
    use super::{rename, ReplacementOperation, ReplacementParameter};

    #[test]
    fn test_rename() {
        assert_eq!(
            rename("ExampleService", "Example", "Himanoa"),
            "HimanoaService".to_string(),
            "Pascal case test"
        );
        assert_eq!(
            rename("exampleService", "example", "himanoa"),
            "himanoaService".to_string(),
            "Camel case test"
        );
        assert_eq!(
            rename("example-service", "example", "himanoa"),
            "himanoa-service".to_string(),
            "Kebab case test"
        );
        assert_eq!(
            rename("example_service", "example", "himanoa"),
            "himanoa_service".to_string(),
            "Snake case test"
        );
        assert_eq!(
            rename("example_service", "adfadf", "himanoa"),
            "example_service".to_string(),
            "No replace"
        );
    }

    #[test]
    fn test_replacement_parameter_name_changed() {
        assert!(
            !ReplacementParameter::default().name_changed(),
            "id equal renamed_name"
        );
        assert!(
            ReplacementParameter {
                id: "foo".to_string(),
                renamed_name: "bar".to_string(),
                ..ReplacementParameter::default()
            }
            .name_changed(),
            "id equal renamed_name"
        )
    }

    #[test]
    fn test_replacement_parameter_body_changed() {
        assert!(
            !ReplacementParameter::default().body_changed(),
            "before_replace_body equal replaced_body"
        );
        assert!(
            ReplacementParameter {
                before_replace_body: "foo".to_string(),
                replaced_body: "bar".to_string(),
                ..ReplacementParameter::default()
            }
            .body_changed(),
            "before_replace_body not equal replaced_body"
        )
    }

    #[test]
    fn test_replacement_parameter_all_changed() {
        assert!(
            !ReplacementParameter::default().all_changed(),
            "id equal renamed_name"
        );

        assert!(
            !ReplacementParameter {
                id: "foo".to_string(),
                renamed_name: "bar".to_string(),
                ..ReplacementParameter::default()
            }
            .all_changed(),
            "id equal renamed_name"
        );
        assert!(
            !ReplacementParameter {
                before_replace_body: "foo".to_string(),
                replaced_body: "bar".to_string(),
                ..ReplacementParameter::default()
            }
            .all_changed(),
            "before_replace_body not equal replaced_body"
        );
        assert!(
            ReplacementParameter {
                before_replace_body: "foo".to_string(),
                replaced_body: "bar".to_string(),
                id: "foo".to_string(),
                renamed_name: "bar".to_string()
            }
            .all_changed(),
            "before_replace_body not equal replaced_body"
        )
    }

    #[test]
    fn test_replacement_operator_from() {
        assert_eq!(
            ReplacementOperation::from(ReplacementParameter::default()),
            ReplacementOperation::None,
            "before == after"
        );
        assert_eq!(
            ReplacementOperation::from(ReplacementParameter {
                id: "foo".to_string(),
                renamed_name: "bar".to_string(),
                ..ReplacementParameter::default()
            }),
            ReplacementOperation::Rename,
            "id != renamed_name"
        );
        assert_eq!(
            ReplacementOperation::from(ReplacementParameter {
                before_replace_body: "foo".to_string(),
                replaced_body: "bar".to_string(),
                ..ReplacementParameter::default()
            }),
            ReplacementOperation::Replace,
            "before_replace_body != replaced_body"
        );
        assert_eq!(
            ReplacementOperation::from(ReplacementParameter {
                before_replace_body: "foo".to_string(),
                replaced_body: "bar".to_string(),
                id: "foo".to_string(),
                renamed_name: "bar".to_string()
            }),
            ReplacementOperation::RenameAndReplace,
            "all_changed"
        );
    }
}
