#[derive(Debug, Eq, PartialEq, Clone, PartialOrd, Ord)]
pub struct FileName(pub String);

impl FileName {
    pub fn new<T: Into<String>>(filename: T) -> Self {
        FileName(filename.into())
    }
}
