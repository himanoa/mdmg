use thiserror::Error;

#[derive(Error, Debug)]
pub enum MdmbError {
    #[error("ApplicationError")]
    ApplicationError,
    #[error("this template is invalid syntax")]
    TempalteRenderError { reason: String },
    #[error("pending scaffold exists")]
    ReadPendingScaffoldError { file_name: String },
    #[error("io error")]
    IoError(#[from] std::io::Error),
    #[error("file name convert error")]
    FileNameConvertError(std::ffi::OsString),
    #[error("unknown error")]
    Unknown(#[from] anyhow::Error)
}
