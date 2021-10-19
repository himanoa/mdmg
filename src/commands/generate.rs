use crate::template_repository::{FSTemplateRepository, TemplateRepository};
use crate::template::{render, MdmgCtx};
use crate::markdown::parse;
use crate::Result;
use crate::scaffold_executor::{DryRunScaffoldExecutor, ScaffoldExecutor};

use std::env::current_dir;
use std::sync::Arc;

pub struct GenerateCommandImpl;

impl GenerateCommandImpl {
    pub fn new()->Self {
        GenerateCommandImpl
    }
}

trait Dependencies {
    fn template_repository(&self) -> Arc<dyn TemplateRepository>;
}

impl Dependencies for GenerateCommandImpl {
    fn template_repository(&self) -> Arc<dyn TemplateRepository> {
        let current_dir = current_dir().expect("failed fetch current dir");
        Arc::new(FSTemplateRepository::new(current_dir))
    }
}

pub trait GenerateCommand {
    fn run(&self, plan_name: String, component_name: String) -> Result<()>; 
}

impl GenerateCommand for GenerateCommandImpl {
    fn run(&self, plan_name: String, component_name: String) -> Result<()> {
        let template = self.template_repository().resolve(plan_name)?;
        let render_ctx = MdmgCtx::new(component_name);
        if let Ok(scaffolds) = parse(render(template, &render_ctx)?) {
            for scaffold in scaffolds.iter() {
                DryRunScaffoldExecutor::new().execute(scaffold)?;
            }
        };
        Ok(())
    }
}
