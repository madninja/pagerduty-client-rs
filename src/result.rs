use thiserror::Error;
pub type Result<T = ()> = std::result::Result<T, Error>;

#[derive(Error, Debug)]
pub enum Error {
    #[error("request error")]
    Request(#[from] reqwest::Error),
    #[error("json error")]
    JsonError(#[from] serde_json::Error),
    #[error("api error")]
    ApiError(String),
    #[error("custom error")]
    Custom(String),
}

impl Error {
    pub(crate) fn api_error(msg: String) -> Self {
        Self::ApiError(msg)
    }

    pub(crate) fn custom<T: ToString>(msg: T) -> Self {
        Self::Custom(msg.to_string())
    }
}
