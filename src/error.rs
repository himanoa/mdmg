use thiserror::Error;

#[derive(Error, Debug)]
pub enum MdmgError {
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
    #[error("template({0}) is not found")]
    TemplateIsNotFound(String),
    #[error("file path ({0})'s parent path is not found")]
    ParentDirectoryIsNotFound(String),
    #[error("failed remove parent directory. path: {0}")]
    FailedRemoveParentDirectory(String),
    #[error("failed remove file. path: {0}")]
    FailedDeleteFile(String),
    #[error("file({0}) is not found")]
    GeneratedFileIsNotFound(String),
    #[error("unknown error")]
    Unknown(#[from] anyhow::Error),
}
