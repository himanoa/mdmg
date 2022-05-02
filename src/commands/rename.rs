use crate::generated_file_repository::FSGeneratedFileRepository;
use crate::logger::{Logger, StdoutLogger};
use crate::markdown::parse;
use crate::rename_executor::{
    DefaultRenameExecutor, FSReplacementOperationInterpreter, RenameExecutor,
};
use crate::template::{render, MdmgCtx};
use crate::template_repository::{FSTemplateRepository, TemplateRepository};
use crate::Result;

use std::env::current_dir;
use std::sync::Arc;

pub trait RenameCommand {
    fn run(&self, plan_name: &str, identify: &str, replaced_identify: &str) -> Result<()>;
}

pub struct RenameCommandImpl {
    template_repository_instance: Arc<dyn TemplateRepository>,
    logger_instance: Arc<dyn Logger>,
    rename_executor_instance: Arc<dyn RenameExecutor>,
}

impl RenameCommandImpl {
    pub fn new() -> Self {
        let current_dir = current_dir().expect("failed fetch current dir");
        let logger = Arc::new(StdoutLogger::new());
        let replacement_operation_interpreter_instance: Arc<FSReplacementOperationInterpreter> =
            Arc::new(FSReplacementOperationInterpreter::new(logger.clone()));
        let generated_file_repository: Arc<FSGeneratedFileRepository> =
            Arc::new(FSGeneratedFileRepository::new(current_dir.clone()));

        RenameCommandImpl {
            template_repository_instance: Arc::new(FSTemplateRepository::new(
                current_dir.join(".mdmg"),
            )),
            logger_instance: logger,
            rename_executor_instance: Arc::new(DefaultRenameExecutor::new(
                replacement_operation_interpreter_instance,
                generated_file_repository,
            )),
        }
    }
}

trait Dependencies {
    fn template_repository(&self) -> Arc<dyn TemplateRepository>;
    fn logger(&self) -> Arc<dyn Logger>;
    fn rename_executor(&self) -> Arc<dyn RenameExecutor>;
}

impl Dependencies for RenameCommandImpl {
    fn template_repository(&self) -> Arc<dyn TemplateRepository> {
        self.template_repository_instance.clone()
    }
    fn logger(&self) -> Arc<dyn Logger> {
        self.logger_instance.clone()
    }
    fn rename_executor(&self) -> Arc<dyn RenameExecutor> {
        self.rename_executor_instance.clone()
    }
}

impl RenameCommand for RenameCommandImpl {
    fn run(&self, plan_name: &str, identify: &str, replaced_identify: &str) -> Result<()> {
        let template = self.template_repository().resolve(plan_name.to_string())?;
        let render_ctx = MdmgCtx::new(identify);
        let scaffolds = match parse(render(template, &render_ctx)?) {
            Ok(scaffolds) => scaffolds,
            Err(_) => return Ok(()),
        };
        self.rename_executor()
            .execute(&scaffolds, identify, replaced_identify)
    }
}

#[cfg(not(tarpaulin_include))]
#[cfg(test)]
mod tests {
    use crate::commands::rename::{RenameCommand, RenameCommandImpl};
    use crate::error::MdmgError;
    use crate::logger::Logger;
    use crate::rename_executor::RenameExecutor;
    use crate::template::Template;
    use crate::template_repository::TemplateRepository;
    use derive_more::Constructor;

    use std::sync::Arc;

    #[test]
    fn test_rename_command_run_template_is_not_found() {
        #[derive(Constructor, Debug)]
        struct DummyTemplateRepository;

        impl TemplateRepository for DummyTemplateRepository {
            fn resolve(&self, _template_name: String) -> crate::Result<crate::template::Template> {
                Err(MdmgError::TemplateIsNotFound("dummy".to_string()))
            }
            fn list(&self) -> crate::Result<Vec<crate::file::FileName>> {
                Ok(vec![])
            }
        }

        #[derive(Constructor, Debug)]
        struct DummyLogger;

        impl Logger for DummyLogger {
            fn info(&self, _log: &str) {}
            fn debug(&self, _log: &str) {}
        }

        #[derive(Debug, Constructor)]
        struct DummyRenameExecutor;

        impl RenameExecutor for DummyRenameExecutor {
            fn execute(
                &self,
                _scaffolds: &[crate::scaffold::Scaffold],
                _before_identify: &str,
                _after_identify: &str,
            ) -> crate::Result<()> {
                Ok(())
            }
        }

        let command = RenameCommandImpl {
            template_repository_instance: Arc::new(DummyTemplateRepository::new()),
            logger_instance: Arc::new(DummyLogger::new()),
            rename_executor_instance: Arc::new(DummyRenameExecutor::new()),
        };

        let result = command.run("dummy", "dummy", "dummy");
        assert!(result.is_err())
    }

    #[test]
    fn test_rename_command_run_template_is_ok() {
        #[derive(Constructor, Debug)]
        struct DummyTemplateRepository;

        impl TemplateRepository for DummyTemplateRepository {
            fn resolve(&self, _template_name: String) -> crate::Result<crate::template::Template> {
                Ok(Template::new("aaa"))
            }
            fn list(&self) -> crate::Result<Vec<crate::file::FileName>> {
                Ok(vec![])
            }
        }

        #[derive(Constructor, Debug)]
        struct DummyLogger;

        impl Logger for DummyLogger {
            fn info(&self, _log: &str) {}
            fn debug(&self, _log: &str) {}
        }

        #[derive(Debug, Constructor)]
        struct DummyRenameExecutor;

        impl RenameExecutor for DummyRenameExecutor {
            fn execute(
                &self,
                _scaffolds: &[crate::scaffold::Scaffold],
                _before_identify: &str,
                _after_identify: &str,
            ) -> crate::Result<()> {
                Ok(())
            }
        }

        let command = RenameCommandImpl {
            template_repository_instance: Arc::new(DummyTemplateRepository::new()),
            logger_instance: Arc::new(DummyLogger::new()),
            rename_executor_instance: Arc::new(DummyRenameExecutor::new()),
        };

        let result = command.run("dummy", "dummy", "dummy");
        assert!(result.is_ok())
    }

    #[test]
    #[cfg_attr(not(feature = "fs-test"), ignore)]
    fn test_rename_command_impl_new_test() {
        RenameCommandImpl::new();
    }
}
