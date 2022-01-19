use yansi::Paint;

use crate::markdown::parse;
use crate::scaffold::Scaffold;
use crate::template::{render, MdmgCtx};
use crate::template_repository::{FSTemplateRepository, TemplateRepository};
use crate::MdmgError;
use crate::Result;

use std::env::current_dir;
use std::fs::{read_dir, remove_dir, remove_file};
use std::path::Path;
use std::sync::Arc;

pub trait DeleteCommand {
    fn run(&self, plan_name: String, component_name: String) -> Result<()>;
}

pub struct DeleteCommandImpl;

trait Dependencies {
    fn template_repository(&self) -> Arc<dyn TemplateRepository>;
}

impl Dependencies for DeleteCommandImpl {
    fn template_repository(&self) -> Arc<dyn TemplateRepository> {
        let current_dir = current_dir().expect("failed fetch current dir");
        Arc::new(FSTemplateRepository::new(current_dir.join(".mdmg")))
    }
}

impl DeleteCommandImpl {
    pub fn new() -> Self {
        DeleteCommandImpl
    }
}

impl DeleteCommand for DeleteCommandImpl {
    fn run(&self, plan_name: String, component_name: String) -> Result<()> {
        let template = self.template_repository().resolve(plan_name)?;
        let render_ctx = MdmgCtx::new(component_name);
        if let Ok(scaffolds) = parse(render(template, &render_ctx)?) {
            for scaffold in scaffolds.into_iter() {
                if let Scaffold::Complete {
                    file_name,
                    file_body: _,
                } = scaffold
                {
                    remove_file(&file_name.clone())?;
                    println!("{} {}", Paint::green("Delete"), file_name);
                    let parent_path = Path::new(&file_name)
                        .parent()
                        .ok_or(MdmgError::ParentDirectoryIsNotFound(file_name.clone()))?;
                    if read_dir(&parent_path).iter().len() == 0 {
                        remove_dir(parent_path).map_err(|_| {
                            MdmgError::FailedRemoveParentDirectory(
                                parent_path.as_os_str().to_str().unwrap().to_string(),
                            )
                        })?;
                        println!("{} {}", Paint::green("Delete empty directory"), file_name);
                    }
                }
            }
        };
        Ok(())
    }
}
