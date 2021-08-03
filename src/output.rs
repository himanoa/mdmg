use crate::Result;

pub trait Output {
    fn create_file(&self, name: &str, output_file: &str) -> Result<()>;
}
