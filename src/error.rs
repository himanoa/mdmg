#[derive(Debug)]
pub enum MdmbError {
    Default,
    ApplicationError,
    TempalteRenderError{reason: String}
}
