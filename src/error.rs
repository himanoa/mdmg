#[derive(Debug)]
pub enum MdmbError {
    Default,
    ApplicationError,
    TempalteRenderError { reason: String },
    ReadPendingScaffoldError { file_name: String },
}
