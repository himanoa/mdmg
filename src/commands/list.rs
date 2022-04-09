use crate::logger::{Logger, StdoutLogger};
use crate::template_repository::{FSTemplateRepository, TemplateRepository};
use crate::Result;
use std::env::current_dir;
use std::sync::Arc;

pub trait ListCommand {
    fn run(&self) -> Result<()>;
}

pub trait Dependencies {
    fn template_repository(&self) -> Arc<dyn TemplateRepository>;
    fn logger(&self) -> Arc<dyn Logger>;
}

pub struct ListCommandImpl {
    logger_instance: Arc<dyn Logger>,
    template_repository_instance: Arc<dyn TemplateRepository>,
}

impl Default for ListCommandImpl {
    fn default() -> Self {
        let current_dir = current_dir().expect("failed fetch current dir");
        ListCommandImpl {
            template_repository_instance: Arc::new(FSTemplateRepository::new(
                current_dir.join(".mdmg"),
            )),
            logger_instance: Arc::new(StdoutLogger::new()),
        }
    }
}

impl Dependencies for ListCommandImpl {
    fn template_repository(&self) -> Arc<dyn TemplateRepository> {
        self.template_repository_instance.clone()
    }
    fn logger(&self) -> Arc<dyn Logger> {
        self.logger_instance.clone()
    }
}

impl ListCommand for ListCommandImpl {
    fn run(&self) -> Result<()> {
        let template_list = self.template_repository().list()?;
        for template in template_list.iter() {
            self.logger().info(&template.0)
        }
        Ok(())
    }
}

#[cfg(not(tarpaulin_include))]
#[cfg(test)]
mod tests {
    use std::sync::Mutex;

    use super::*;
    use crate::logger::Logger;
    use crate::template_repository::{FileName, TemplateRepository};
    use crate::Result;
    use derive_more::Constructor;

    #[test]
    fn test_list_command_output() {
        #[derive(Debug, Constructor, Copy, Clone)]
        struct DummyTemplateRepository;

        #[derive(Debug, Constructor)]
        struct DummyLogger {
            outputs: Mutex<Vec<String>>,
        }

        impl TemplateRepository for DummyTemplateRepository {
            fn list(&self) -> Result<Vec<crate::template_repository::FileName>> {
                Ok(vec![FileName::new("foo")])
            }
            fn resolve(&self, _: String) -> Result<crate::template::Template> {
                unimplemented!()
            }
        }

        impl Logger for DummyLogger {
            fn info(&self, info: &str) {
                self.outputs.lock().unwrap().push(info.to_string());
            }
            fn debug(&self, log: &str) {
                unreachable!();
            }
        }

        let logger = Arc::new(DummyLogger::new(Mutex::new(vec![])));
        let command = ListCommandImpl {
            logger_instance: logger.clone(),
            template_repository_instance: Arc::new(DummyTemplateRepository::new()),
        };

        assert!(command.run().is_ok());
        assert_eq!(*logger.outputs.lock().unwrap(), vec!["foo".to_string()]);
    }
}
