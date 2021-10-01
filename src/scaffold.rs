use crate::error::MdmbError;
use crate::output::Output;
use crate::Result;

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum Scaffold {
    Complete {
        file_name: String,
        file_body: String
    },
    Pending {
        file_name: String,
    }
}

impl Scaffold {
    pub fn execute(&self, output: &impl Output) -> Result<()> {
        if let Scaffold::Complete { file_name, file_body } = self {
            output.create_file(
                file_name.as_str(),
                file_body.as_str()
            )
        } else if let Scaffold::Pending { file_name } = self {
            Err(MdmbError::ReadPendingScaffoldError { file_name: file_name.clone() })
        } else {
            unimplemented!()
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::error::MdmbError;
    use super::{Output, Result, Scaffold};

    #[test]
    fn it_called_create_file() {
        #[derive(Debug, Default)]
        struct DummyOutput {
        }
        impl Output for DummyOutput {
            fn create_file(& self, _file_name: &str, _output: &str) -> Result<()> {
                Err(MdmbError::ApplicationError)
            }
        }

        impl Default for Scaffold {
            fn default() -> Self {
                Scaffold::Complete { file_name: "test".to_string(), file_body: "foo".to_string() }
            }
        }

        let scaffold: Scaffold = Default::default();
        let output: DummyOutput = Default::default();
        let result = scaffold.execute(&output);
        assert!(result.is_err());
    }
}
