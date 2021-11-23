use crate::scaffold::Scaffold;
use crate::Result;

use std::fs::{create_dir_all, File};
use std::io::Write;
use std::path::Path;

use yansi::Paint;

pub trait ScaffoldExecutor {
    fn execute(self, scaffold: &Scaffold) -> Result<()>;
}

#[derive(Clone, Debug, Copy)]
pub struct DryRunScaffoldExecutor {}

#[derive(Clone, Debug, Copy)]
pub struct FSScaffoldExecutor {}

impl FSScaffoldExecutor {
    pub fn new() -> Self {
        FSScaffoldExecutor {}
    }
}

impl DryRunScaffoldExecutor {
    pub fn new() -> Self {
        DryRunScaffoldExecutor {}
    }
}

impl ScaffoldExecutor for DryRunScaffoldExecutor {
    fn execute(self, scaffold: &Scaffold) -> Result<()> {
        if let Scaffold::Complete {
            file_name,
            file_body,
        } = scaffold
        {
            println!("=== filename: {} ===", file_name);
            println!("{}", file_body);
            println!("====================");
        }
        Ok(())
    }
}

impl ScaffoldExecutor for FSScaffoldExecutor {
    fn execute(self, scaffold: &Scaffold) -> Result<()> {
        if let Scaffold::Complete {
            file_name,
            file_body,
        } = scaffold
        {
            if Path::new(file_name).exists() {
                println!(
                    "{} {} (file_exists)",
                    Paint::yellow("Skip generate:"),
                    file_name
                );
                return Ok(());
            }
            let parent = Path::new(file_name).parent();
            if let Some(parent_path) = parent {
                create_dir_all(parent_path)?;
            }
            let mut file = File::create(file_name)?;
            file.write_all(file_body.as_bytes())?;
            println!("{} {}", Paint::green("Generated:"), file_name);
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::{FSScaffoldExecutor, ScaffoldExecutor};
    use crate::scaffold::Scaffold;
    use std::fs::{read_to_string, remove_dir_all};

    #[test]
    #[cfg_attr(not(feature = "fs-test"), ignore)]
    pub fn fsscaffold_executor_execute_is_created_files() {
        let executor = FSScaffoldExecutor {};
        let path = "support/fs_scaffold_executor_execute/foobar.md".to_string();
        let scaffold = Scaffold::Complete {
            file_name: path.clone(),
            file_body: "hello_world".to_string(),
        };
        executor.execute(&scaffold).unwrap();
        let actual_file_body = read_to_string(path).expect("file is not found");
        assert_eq!(actual_file_body, "hello_world".to_string());
        remove_dir_all("support/fs_scaffold_executor_execute").unwrap();
    }
}
