use crate::Result;
use crate::markdown::parse;
use crate::logger::{Logger, StdoutLogger};
use crate::rename_executor::{RenameExecutor, DefaultRenameExecutor, ReplacementOperationInterpreter, FSReplacementOperationInterpreter};
use crate::template::{render, MdmgCtx};
use crate::template_repository::{FSTemplateRepository, TemplateRepository};

use std::sync::Arc;
use std::env::current_dir;

pub trait RenameCommand {
    fn run(&self, plan_name: &str, identify: &str, replaced_identify: &str) -> Result<()>;
}

pub struct RenameCommandImpl {
    template_repository_instance: Arc<dyn TemplateRepository>,
    logger_instance: Arc<dyn Logger>,
    rename_executor_instance: Arc<dyn RenameExecutor>,
}

impl RenameCommandImpl {
    fn new() -> Self {
        let current_dir = current_dir().expect("failed fetch current dir");
        let logger = Arc::new(StdoutLogger::new());
        let replacement_operation_interpreter_instance: Arc<FSReplacementOperationInterpreter> = Arc::new(FSReplacementOperationInterpreter::new(logger.clone()));

        RenameCommandImpl {
            template_repository_instance: Arc::new(FSTemplateRepository::new(current_dir)),
            logger_instance: logger.clone(),
            rename_executor_instance: Arc::new(DefaultRenameExecutor::new(replacement_operation_interpreter_instance))
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
            Ok(scaffolds) => { scaffolds },
            Err(_) => { return Ok(()) }
        };
        &self.rename_executor().execute(&scaffolds, identify, replaced_identify);
        Ok(())
    }
}
