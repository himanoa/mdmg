use crate::delete_executor::{DeleteExecutor, FSDeleteExecutor, FSDeleteExecutorDeps};
use crate::markdown::parse;
use crate::template::{render, MdmgCtx};
use crate::template_repository::{FSTemplateRepository, TemplateRepository};
use crate::Result;

use std::env::current_dir;
use std::sync::Arc;

pub trait DeleteCommand {
    fn run(&self, plan_name: String, component_name: String) -> Result<()>;
}

pub struct DeleteCommandImpl {
    template_repository_ref: Arc<dyn TemplateRepository>,
    delete_executor_ref: Arc<dyn DeleteExecutor>,
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
            delete_executor_ref: Arc::new(FSDeleteExecutor::new(delete_executor_deps)),
        }
    }
}

impl DeleteCommand for DeleteCommandImpl {
    fn run(&self, plan_name: String, component_name: String) -> Result<()> {
        let template = self.template_repository().resolve(plan_name)?;
        let render_ctx = MdmgCtx::new(component_name);
        let scaffolds = parse(render(template, &render_ctx)?)?;

        for scaffold in scaffolds.into_iter() {
            match &self.delete_executor().execute(&scaffold) {
                Ok(_) => {}
                Err(e) => eprintln!("{}", e.to_string()),
            }
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        commands::delete::DeleteCommand, delete_executor::DeleteExecutor,
        template_repository::TemplateRepository,
    };

    use super::DeleteCommandImpl;

    use indoc::indoc;
    use std::cell::RefCell;
    use std::sync::Arc;

    #[test]
    pub fn success_delete_command() {
        #[derive(Default)]
        struct StubTemplateRepository {}

        #[derive(Default)]
        struct StubDeleteExecutor {
            pub deleted_file: RefCell<Vec<String>>,
        }

        impl TemplateRepository for StubTemplateRepository {
            fn resolve(&self, _template_name: String) -> crate::Result<crate::template::Template> {
                Ok(crate::template::Template::new(
                    indoc! {"
                    ## foobar/foo/bar01.md

                    ```
                    dummy
                    ```

                    ## foobar/foo/bar02.md

                    ```
                    dummy
                    ```
                "}
                    .to_string(),
                ))
            }

            fn list(&self) -> crate::Result<Vec<crate::template_repository::FileName>> {
                unimplemented!()
            }
        }

        impl DeleteExecutor for StubDeleteExecutor {
            fn execute(&self, scaffold: &crate::scaffold::Scaffold) -> crate::Result<()> {
                match scaffold {
                    crate::scaffold::Scaffold::Complete {
                        file_name,
                        file_body: _,
                    } => self.deleted_file.borrow_mut().push(file_name.clone()),
                    crate::scaffold::Scaffold::Pending { file_name } => {
                        self.deleted_file.borrow_mut().push(file_name.clone())
                    }
                }
                Ok(())
            }
        }

        let stub_delete_executor_ref = Arc::new(StubDeleteExecutor::default());

        impl DeleteCommandImpl {
            fn dummy_new(stub_delete_executor_ref: Arc<StubDeleteExecutor>) -> Self {
                DeleteCommandImpl {
                    template_repository_ref: Arc::new(StubTemplateRepository::default()),
                    delete_executor_ref: stub_delete_executor_ref,
                }
            }
        }

        let delete_command = DeleteCommandImpl::dummy_new(stub_delete_executor_ref.clone());
        let actual = delete_command.run("dummy".to_string(), "dummy".to_string());

        assert!(actual.is_ok());
        assert_eq!(
            *stub_delete_executor_ref.deleted_file.borrow(),
            vec![
                "foobar/foo/bar01.md".to_string(),
                "foobar/foo/bar02.md".to_string()
            ]
        );
    }
}
