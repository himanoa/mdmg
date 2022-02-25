use crate::markdown::parse;
use crate::scaffold_executor::{DryRunScaffoldExecutor, FSScaffoldExecutor, ScaffoldExecutor};
use crate::template::{render, MdmgCtx};
use crate::template_repository::{FSTemplateRepository, TemplateRepository};
use crate::Result;

use std::env::current_dir;
use std::sync::Arc;

pub struct GenerateCommandImpl;

impl GenerateCommandImpl {
    pub fn new() -> Self {
        GenerateCommandImpl
    }
}

trait Dependencies {
    fn template_repository(&self) -> Arc<dyn TemplateRepository>;
}

impl Dependencies for GenerateCommandImpl {
    fn template_repository(&self) -> Arc<dyn TemplateRepository> {
        let current_dir = current_dir().expect("failed fetch current dir");
        Arc::new(FSTemplateRepository::new(current_dir.join(".mdmg")))
    }
}

pub trait GenerateCommand {
    fn run(&self, plan_name: String, component_name: String, dry_run: bool) -> Result<()>;
}

impl GenerateCommand for GenerateCommandImpl {
    fn run(&self, plan_name: String, component_name: String, dry_run: bool) -> Result<()> {
        let template = self.template_repository().resolve(plan_name)?;
        let render_ctx = MdmgCtx::new(component_name);
        if let Ok(scaffolds) = parse(render(template, &render_ctx)?) {
            for scaffold in scaffolds.iter() {
                match dry_run {
                    true => DryRunScaffoldExecutor::new().execute(scaffold)?,
                    false => FSScaffoldExecutor::new().execute(scaffold)?,
                };
            }
        };
        Ok(())
    }
}

#[cfg(not(tarpaulin_include))]
#[cfg(test)]
mod tests {
    use std::fs::{create_dir, write, remove_file, remove_dir};
    use std::path::Path;

    use indoc::indoc;

    use super::*;


    fn setup_template() {
        let path = Path::new(".mdmg/");
        assert!(write(".mdmg/example.md", indoc! {"
          ## test.md

          ```
          hello
          ```
        "}.to_string()).is_ok());
    }

    fn terradown_template() {
        let path = Path::new(".mdmg/example.md");
        
        assert!(remove_file(path).is_ok());
        assert!(remove_file("test.md").is_ok());
    }

    #[test]
    #[cfg_attr(not(feature = "fs-test"), ignore)]
    pub fn generate_command_run_is_file_delete() {
        setup_template();
        let command = GenerateCommandImpl::new();
        let actual = command.run("example".to_string(), "foo".to_string(), false);
        
        assert!(actual.is_ok());
        terradown_template();
    }
    
}
