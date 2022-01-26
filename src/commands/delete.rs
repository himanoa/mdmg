use crate::markdown::parse;
use crate::template::{render, MdmgCtx};
use crate::template_repository::{FSTemplateRepository, TemplateRepository};
use crate::delete_executor::{FSDeleteExecutor, DeleteExecutor, FSDeleteExecutorDeps};
use crate::Result;

use std::env::current_dir;
use std::sync::Arc;

pub trait DeleteCommand {
    fn run(&self, plan_name: String, component_name: String) -> Result<()>;
}

pub struct DeleteCommandImpl {
    template_repository_ref: Arc<FSTemplateRepository>,
    delete_executor_ref: Arc<dyn DeleteExecutor>
}

trait Dependencies {
    fn template_repository(&self) -> Arc<dyn TemplateRepository>;
    fn delete_executor(&self) -> Arc<dyn DeleteExecutor>;
}

impl Dependencies for DeleteCommandImpl {
    fn template_repository(&self) -> Arc<dyn TemplateRepository> {
        self.template_repository_ref.clone()
    }

    fn delete_executor(&self) -> Arc<dyn DeleteExecutor> {
        self.delete_executor_ref.clone()
    }
}

impl DeleteCommandImpl {
    pub fn new() -> Self {
        let current_dir = current_dir().expect("failed fetch current dir");
        let delete_executor_deps = Arc::new(FSDeleteExecutorDeps::new());
        DeleteCommandImpl { 
            template_repository_ref: Arc::new(FSTemplateRepository::new(current_dir.join(".mdmg"))),
            delete_executor_ref: Arc::new(FSDeleteExecutor::new(delete_executor_deps))
        }
    }
}

impl DeleteCommand for DeleteCommandImpl {
    fn run(&self, plan_name: String, component_name: String) -> Result<()> {
        let template = self.template_repository().resolve(plan_name)?;
        let render_ctx = MdmgCtx::new(component_name);
        let scaffolds = parse(render(template, &render_ctx)?)?;

        for scaffold in scaffolds.into_iter() {
            match self.delete_executor().execute(&scaffold) {
                Ok(_) => {},
                Err(e) => eprintln!("{}", e.to_string())
            }
        }

        Ok(())
    }
}
