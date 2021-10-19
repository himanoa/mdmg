use crate::template_repository::{FSTemplateRepository, TemplateRepository};
use crate::Result;
use std::env::current_dir;
use std::sync::Arc;

pub trait ListCommand {
    fn run(&self) -> Result<()>;
}

pub trait Dependencies {
    fn template_repository(&self) -> Arc<dyn TemplateRepository>;
}

pub struct ListCommandImpl;

impl ListCommandImpl {
    pub fn new() -> Self {
        ListCommandImpl
    }
}

impl Dependencies for ListCommandImpl {
    fn template_repository(&self) -> Arc<dyn TemplateRepository> {
        let current_dir = current_dir().expect("failed fetch current dir");
        Arc::new(FSTemplateRepository::new(current_dir.join(".mdmg")))
    }
}

impl ListCommand for ListCommandImpl {
    fn run(&self) -> Result<()> {
        let template_list = self.template_repository().list()?;
        for template in template_list.iter() {
            println!("{}", &template.0);
        }
        Ok(())
    }
}
