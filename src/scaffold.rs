use crate::output::Output;
use crate::Result;

#[derive(Default, Debug, PartialEq, Eq)]
pub struct Scaffold {
    pub file_name: String,
    pub file_body: String
}

impl Scaffold {
    pub fn execute(&self, output: &impl Output) -> Result<()> {
        output.create_file(
            self.file_name.as_str(),
            self.file_body.as_str()
        )
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

        let scaffold: Scaffold = Default::default();
        let output: DummyOutput = Default::default();
        let result = scaffold.execute(&output);
        assert!(result.is_err());
    }
}
