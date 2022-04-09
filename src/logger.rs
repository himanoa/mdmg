use derive_more::Constructor;

#[cfg(not(tarpaulin_include))]
pub trait Logger {
    fn info(&self, info: &str);
    fn debug(&self, log: &str);
}

#[cfg(not(tarpaulin_include))]
#[derive(Debug, Clone, Copy, Constructor)]
pub struct StdoutLogger {}

#[cfg(not(tarpaulin_include))]
impl Logger for StdoutLogger {
    fn info(&self, info: &str) {
        println!("{}", info);
    }
    fn debug(&self, log: &str) {
        println!("{}", log);
    }
}
