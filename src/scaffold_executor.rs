use crate::scaffold::Scaffold;
use crate::Result;

pub trait ScaffoldExecutor {
    fn execute(self, scaffold: &Scaffold) -> Result<()>;
}

#[derive(Clone, Debug, Copy)]
pub struct DryRunScaffoldExecutor{
}

impl DryRunScaffoldExecutor {
    pub fn new () -> Self {
        DryRunScaffoldExecutor {}
    }
}

impl ScaffoldExecutor for DryRunScaffoldExecutor {
    fn execute(self, scaffold: &Scaffold) -> Result<()> {
        match scaffold {
            Scaffold::Complete { file_name, file_body } => {
                println!("=== filename: {} ===", file_name);
                println!("{}", file_body);
                println!("====================");
            },
            _ => {}
        };
        Ok(())
    }
}
