use serde_json as json;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("Internal error")]
    Internal(String),

    #[error("{0}")]
    NotFound(String),

    #[error("{0}")]
    PermissionDenied(String),

    #[error("{0}")]
    InvalidArgument(String),

    #[error("{0}")]
    AlreadyExists(String),
}

impl std::convert::From<json::Error> for Error {
    fn from(err: json::Error) -> Self {
        Error::InvalidArgument(format!("failed to parse json: {}", err))
    }
}

impl std::convert::From<reqwest::Error> for Error {
    fn from(err: reqwest::Error) -> Self {
        Error::InvalidArgument(err.to_string())
    }
}
