use thiserror::Error;

#[derive(Error, Debug, Clone)]
pub enum ScanError {
    #[error("{0}, File Open Error")]
    FileOpenError(String),
    #[error("{0}, Invalid HTTP response")]
    InvalidHttpResponse(String),
    #[error("{0}, Reqwest Error")]
    ReqwestError(String),
    #[error("{0}, Tokio Error")]
    TokioJoinError(String),
}

impl std::convert::From<tokio::task::JoinError> for ScanError {
    fn from(error_message: tokio::task::JoinError) -> Self {
        ScanError::TokioJoinError(error_message.to_string())
    }
}

impl std::convert::From<reqwest::Error> for ScanError {
    fn from(error_message: reqwest::Error) -> Self {
        ScanError::ReqwestError(error_message.to_string())
    }
}

impl std::convert::From<std::io::Error> for ScanError {
    fn from(error_message: std::io::Error) -> Self {
        ScanError::FileOpenError(error_message.to_string())
    }
}
