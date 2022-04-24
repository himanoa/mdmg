use crate::scaffold::Scaffold;
use crate::Result;
use crate::{error::MdmgError, logger::Logger};

use derive_more::{Constructor, Display, Into};
use inflector::Inflector;
use std::fs::{remove_file, rename as rename_file, write};
use std::path::Path;
use std::sync::Arc;

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
}

#[derive(Debug, Clone, Constructor, PartialEq, Eq, Default, Into)]
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
        let renamed_file_name = rename(file_name, before_identify, after_identify);
        let replaced_file_body = rename(file_body, before_identify, after_identify);

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

trait ReplacementOperationInterpreter {
    fn none(&self, id: &str);
    fn rename(&self, from_name: &str, to_name: &str) -> Result<()>;
    fn replace(&self, id: &str, replaced_body: &str) -> Result<()>;
    fn rename_and_replace(&self, parameter: &ReplacementParameter) -> Result<()>;
}

fn run(
    parameter: &ReplacementParameter,
    interpreter: &dyn ReplacementOperationInterpreter,
) -> Result<()> {
    let operation: ReplacementOperation = ReplacementOperation::from(parameter);
    match operation {
        ReplacementOperation::None => {
            interpreter.none(parameter.id.as_str());
            Ok(())
        }
        ReplacementOperation::Rename => {
            interpreter.rename(parameter.id.as_str(), parameter.renamed_name.as_str())
        }
        ReplacementOperation::Replace => {
            interpreter.replace(parameter.id.as_str(), parameter.replaced_body.as_str())
        }
        ReplacementOperation::RenameAndReplace => interpreter.rename_and_replace(parameter),
    }
}

#[derive(Constructor)]
struct FSReplacementOperationInterpreter {
    logger_instance: Arc<dyn Logger>,
}

impl ReplacementOperationInterpreter for FSReplacementOperationInterpreter {
    fn none(&self, id: &str) {
        self.logger_instance
            .info(format!("{} is no changed", id).as_str())
    }
    fn rename(&self, from_name: &str, to_name: &str) -> Result<()> {
        self.logger_instance
            .info(format!("{} rename started.(to: {})", &from_name, &to_name).as_str());

        rename_file(from_name, to_name)?;

        self.logger_instance
            .info(format!("{} renamed", &from_name).as_str());

        Ok(())
    }
    fn replace(&self, id: &str, replaced_body: &str) -> Result<()> {
        self.logger_instance
            .info(format!("{} replace file body started.", &id).as_str());

        write(id, replaced_body)?;

        self.logger_instance
            .info(format!("{} replaced file body.", &id).as_str());

        Ok(())
    }
    fn rename_and_replace(&self, parameter: &ReplacementParameter) -> Result<()> {
        self.logger_instance.info(
            format!(
                "{} replace name and body started.(to: {})",
                &parameter.id, &parameter.renamed_name
            )
            .as_str(),
        );

        write(
            parameter.renamed_name.as_str(),
            parameter.replaced_body.as_str(),
        )?;
        remove_file(Path::new(parameter.id.as_str()))?;

        self.logger_instance.info(
            format!(
                "{} replaced name and body.(to: {})",
                &parameter.id, &parameter.renamed_name
            )
            .as_str(),
        );
        Ok(())
    }
}

impl From<&ReplacementParameter> for ReplacementOperation {
    fn from(params: &ReplacementParameter) -> Self {
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
        scaffolds: &[Scaffold],
        before_identify: &str,
        after_identify: &str,
    ) -> Result<()>;
}

#[derive(Constructor)]
struct DefaultRenameExecutor {
    interpreter: Arc<dyn ReplacementOperationInterpreter>,
}

impl RenameExecutor for DefaultRenameExecutor {
    fn execute(
        self,
        scaffolds: &[Scaffold],
        before_identify: &str,
        after_identify: &str,
    ) -> Result<()> {
        for scaffold in scaffolds.iter() {
            let parameter =
                ReplacementParameter::from_scaffold(scaffold, before_identify, after_identify)?;
            run(&parameter, self.interpreter.as_ref())?;
        }
        Ok(())
    }
}

#[cfg(not(tarpaulin_include))]
#[cfg(test)]
mod tests {
    use crate::logger::Logger;
    use crate::rename_executor::{
        DefaultRenameExecutor, FSReplacementOperationInterpreter, RenameExecutor,
        ReplacementOperationInterpreter,
    };
    use crate::scaffold::Scaffold;

    use super::{rename, run, ReplacementOperation, ReplacementParameter};
    use derive_more::{Deref, DerefMut};
    use std::cell::Cell;
    use std::fs::{create_dir, read_to_string, remove_dir, remove_file, write};
    use std::path::Path;
    use std::sync::{Arc, Mutex};

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
            ReplacementOperation::from(&ReplacementParameter::default()),
            ReplacementOperation::None,
            "before == after"
        );
        assert_eq!(
            ReplacementOperation::from(&ReplacementParameter {
                id: "foo".to_string(),
                renamed_name: "bar".to_string(),
                ..ReplacementParameter::default()
            }),
            ReplacementOperation::Rename,
            "id != renamed_name"
        );
        assert_eq!(
            ReplacementOperation::from(&ReplacementParameter {
                before_replace_body: "foo".to_string(),
                replaced_body: "bar".to_string(),
                ..ReplacementParameter::default()
            }),
            ReplacementOperation::Replace,
            "before_replace_body != replaced_body"
        );
        assert_eq!(
            ReplacementOperation::from(&ReplacementParameter {
                before_replace_body: "foo".to_string(),
                replaced_body: "bar".to_string(),
                id: "foo".to_string(),
                renamed_name: "bar".to_string()
            }),
            ReplacementOperation::RenameAndReplace,
            "all_changed"
        );
    }

    #[test]
    fn test_replacement_operation_interpreter_when_operation_is_none() {
        #[derive(Deref)]
        struct Dummy(pub Cell<bool>);

        impl ReplacementOperationInterpreter for Dummy {
            fn none(&self, _id: &str) {
                self.0.replace(true);
            }
            fn rename(&self, _from_name: &str, _to_name: &str) -> crate::Result<()> {
                unreachable!()
            }
            fn replace(&self, _id: &str, _replaced_body: &str) -> crate::Result<()> {
                unreachable!()
            }
            fn rename_and_replace(&self, _parameter: &ReplacementParameter) -> crate::Result<()> {
                unreachable!()
            }
        }

        let interpreter = Dummy(Cell::new(false));
        assert!(run(
            &ReplacementParameter::new(
                "xxx".to_string(),
                "xxx".to_string(),
                "xxx".to_string(),
                "xxx".to_string(),
            ),
            &interpreter
        )
        .is_ok());
        assert!(interpreter.get());
    }

    #[test]
    fn test_replacement_operation_interpreter_when_operation_is_rename() {
        #[derive(Deref)]
        struct Dummy(pub Cell<bool>);

        impl ReplacementOperationInterpreter for Dummy {
            fn none(&self, _id: &str) {
                unreachable!()
            }
            fn rename(&self, _from_name: &str, _to_name: &str) -> crate::Result<()> {
                self.0.replace(true);
                Ok(())
            }
            fn replace(&self, _id: &str, _replaced_body: &str) -> crate::Result<()> {
                unreachable!()
            }
            fn rename_and_replace(&self, _parameter: &ReplacementParameter) -> crate::Result<()> {
                unreachable!()
            }
        }

        let interpreter = Dummy(Cell::new(false));
        assert!(run(
            &ReplacementParameter::new(
                "xxx".to_string(),
                "xxxy".to_string(),
                "xxx".to_string(),
                "xxx".to_string(),
            ),
            &interpreter
        )
        .is_ok());
        assert!(interpreter.get());
    }

    #[test]
    fn test_replacement_operation_interpreter_when_operation_is_replace() {
        #[derive(Deref)]
        struct Dummy(pub Cell<bool>);

        impl ReplacementOperationInterpreter for Dummy {
            fn none(&self, _id: &str) {
                unreachable!()
            }
            fn rename(&self, _from_name: &str, _to_name: &str) -> crate::Result<()> {
                unreachable!()
            }
            fn replace(&self, _id: &str, _replaced_body: &str) -> crate::Result<()> {
                self.0.replace(true);
                Ok(())
            }
            fn rename_and_replace(&self, _parameter: &ReplacementParameter) -> crate::Result<()> {
                unreachable!()
            }
        }

        let interpreter = Dummy(Cell::new(false));
        assert!(run(
            &ReplacementParameter::new(
                "xxx".to_string(),
                "xxx".to_string(),
                "xxx".to_string(),
                "xxxy".to_string(),
            ),
            &interpreter
        )
        .is_ok());
        assert!(interpreter.get());
    }

    #[test]
    fn test_replacement_operation_interpreter_when_operation_is_rename_and_replace() {
        #[derive(Deref)]
        struct Dummy(pub Cell<bool>);

        impl ReplacementOperationInterpreter for Dummy {
            fn none(&self, _id: &str) {
                unreachable!()
            }
            fn rename(&self, _from_name: &str, _to_name: &str) -> crate::Result<()> {
                unreachable!()
            }
            fn replace(&self, _id: &str, _replaced_body: &str) -> crate::Result<()> {
                unreachable!()
            }
            fn rename_and_replace(&self, _parameter: &ReplacementParameter) -> crate::Result<()> {
                self.0.replace(true);
                Ok(())
            }
        }

        let interpreter = Dummy(Cell::new(false));
        assert!(run(
            &ReplacementParameter::new(
                "xxx".to_string(),
                "xxxy".to_string(),
                "xxx".to_string(),
                "xxxy".to_string(),
            ),
            &interpreter
        )
        .is_ok());
        assert!(interpreter.get());
    }

    #[test]
    fn test_from_scafold() {
        let scaffold = Scaffold::Complete {
            file_name: "sooo".to_string(),
            file_body: "".to_string(),
        };
        let parameter =
            ReplacementParameter::from_scaffold(&scaffold, "before_identify", "after_identify")
                .unwrap();
        assert!(!parameter.all_changed());
        assert!(!parameter.name_changed());
        assert!(!parameter.body_changed());

        let scaffold = Scaffold::Complete {
            file_name: "sooo".to_string(),
            file_body: "".to_string(),
        };
        let parameter = ReplacementParameter::from_scaffold(&scaffold, "so", "af").unwrap();
        assert!(parameter.name_changed());
        assert!(!parameter.all_changed());
        assert!(!parameter.body_changed());

        let scaffold = Scaffold::Complete {
            file_name: "ooo".to_string(),
            file_body: "so".to_string(),
        };
        let parameter = ReplacementParameter::from_scaffold(&scaffold, "so", "af").unwrap();
        assert!(parameter.body_changed());
        assert!(!parameter.name_changed());
        assert!(!parameter.all_changed());

        let scaffold = Scaffold::Complete {
            file_name: "so".to_string(),
            file_body: "so".to_string(),
        };
        let parameter = ReplacementParameter::from_scaffold(&scaffold, "so", "af").unwrap();
        assert!(parameter.all_changed());

        let scaffold = Scaffold::Pending {
            file_name: "xxx".to_string(),
        };
        assert!(ReplacementParameter::from_scaffold(&scaffold, "so", "af").is_err())
    }

    struct DummyLogger(Cell<bool>);
    impl Logger for DummyLogger {
        fn info(&self, _info: &str) {
            self.0.replace(true);
        }
        fn debug(&self, _log: &str) {}
    }

    #[test]
    #[cfg_attr(not(feature = "fs-test"), ignore)]
    pub fn test_fs_replacement_operation_interpreter_none() {
        let logger = Arc::new(DummyLogger(Cell::new(false)));
        let interpreter = FSReplacementOperationInterpreter::new(logger.clone());
        interpreter.none("foo");
        assert!(logger.0.get());
    }

    #[test]
    #[cfg_attr(not(feature = "fs-test"), ignore)]
    pub fn test_fs_replacement_operation_interpreter_rename() {
        let path =
            Path::new("./support/fs_rename_executor_fs_replacement_operation_interpreter_rename/");
        let file_path = path.join("dummy.txt");
        let logger = Arc::new(DummyLogger(Cell::new(false)));
        let interpreter = FSReplacementOperationInterpreter::new(logger.clone());
        let dist_path =
            "./support/fs_rename_executor_fs_replacement_operation_interpreter_rename/bar.txt";

        assert!(create_dir(path).is_ok());
        assert!(write(&file_path, "dummy").is_ok());
        assert!(interpreter
            .rename(file_path.to_string_lossy().as_ref(), dist_path)
            .is_ok());
        assert!(Path::new(&dist_path).exists());
        assert!(logger.0.get());
        assert!(remove_file(dist_path).is_ok());
        assert!(remove_dir(path).is_ok());
    }

    #[test]
    #[cfg_attr(not(feature = "fs-test"), ignore)]
    pub fn test_fs_replacement_operation_interpreter_replace() {
        let path =
            Path::new("./support/fs_rename_executor_fs_replacement_operation_interpreter_replace/");
        let file_path = path.join("dummy.txt");
        let logger = Arc::new(DummyLogger(Cell::new(false)));
        let interpreter = FSReplacementOperationInterpreter::new(logger.clone());

        assert!(create_dir(path).is_ok());
        assert!(write(&file_path, "dummy").is_ok());
        assert!(interpreter
            .replace(file_path.to_string_lossy().as_ref(), "replaced")
            .is_ok());
        assert_eq!(read_to_string(&file_path).unwrap().as_str(), "replaced");
        assert!(logger.0.get());
        assert!(remove_file(file_path).is_ok());
        assert!(remove_dir(path).is_ok());
    }

    #[test]
    #[cfg_attr(not(feature = "fs-test"), ignore)]
    pub fn test_fs_replacement_operation_interpreter_rename_and_replace() {
        let path = Path::new(
            "./support/fs_rename_executor_fs_replacement_operation_interpreter_rename_and_replace/",
        );
        let file_path = path.join("dummy");
        let logger = Arc::new(DummyLogger(Cell::new(false)));
        let interpreter = FSReplacementOperationInterpreter::new(logger.clone());
        let dist_path = Path::new("./support/fs_rename_executor_fs_replacement_operation_interpreter_rename_and_replace/dummy1");

        assert!(create_dir(path).is_ok());
        assert!(write(&file_path, "dummy").is_ok());
        assert!(interpreter
            .rename_and_replace(&ReplacementParameter::new(
                file_path.to_string_lossy().to_string(),
                dist_path.to_string_lossy().to_string(),
                "dummy".to_string(),
                "dummy1".to_string()
            ))
            .is_ok());
        assert!(Path::new(&dist_path).exists());
        assert_eq!(read_to_string(&dist_path).unwrap().as_str(), "dummy1");
        assert!(logger.0.get());
        assert!(remove_file(dist_path).is_ok());
        assert!(remove_dir(path).is_ok());
    }

    #[test]
    pub fn rename_executor_execute() {
        #[derive(DerefMut, Deref)]
        struct DummyInterpreter(pub Mutex<Vec<String>>);

        impl ReplacementOperationInterpreter for DummyInterpreter {
            fn none(&self, _id: &str) {
                self.lock().unwrap().push("none".to_string());
            }
            fn rename(&self, _from_name: &str, _to_name: &str) -> crate::Result<()> {
                self.lock().unwrap().push("rename".to_string());
                Ok(())
            }
            fn replace(&self, _id: &str, _replaced_body: &str) -> crate::Result<()> {
                self.lock().unwrap().push("replace".to_string());
                Ok(())
            }
            fn rename_and_replace(&self, _parameter: &ReplacementParameter) -> crate::Result<()> {
                self.lock().unwrap().push("rename_and_replace".to_string());
                Ok(())
            }
        }

        let interpreter = Arc::new(DummyInterpreter(Mutex::new(vec![])));
        let executor = DefaultRenameExecutor::new(interpreter.clone());
        let scaffolds = vec![
            Scaffold::Complete {
                file_name: "xxx".to_string(),
                file_body: "xxx".to_string(),
            },
            Scaffold::Complete {
                file_name: "replace_target".to_string(),
                file_body: "xxx".to_string(),
            },
            Scaffold::Complete {
                file_name: "xxx".to_string(),
                file_body: "replace_target".to_string(),
            },
            Scaffold::Complete {
                file_name: "replace_target".to_string(),
                file_body: "replace_target".to_string(),
            },
        ];
        let expected_plans = vec![
            "none".to_string(),
            "rename".to_string(),
            "replace".to_string(),
            "rename_and_replace".to_string(),
        ];
        assert!(executor
            .execute(&scaffolds, "replace_target", "replaced")
            .is_ok());
        assert_eq!(interpreter.lock().unwrap().clone(), expected_plans);
    }
}
