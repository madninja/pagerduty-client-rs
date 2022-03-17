use serde::Deserialize;
use std::fmt;
use thiserror::Error;
pub type Result<T = ()> = std::result::Result<T, Error>;

#[derive(Error, Debug)]
pub enum Error {
    #[error("request error")]
    Request(#[from] reqwest::Error),
    #[error("response error: {0}")]
    Response(#[from] ResponseError),
    #[error("json error")]
    JsonError(#[from] serde_json::Error),
    #[error("api error")]
    ApiError(String),
    #[error("custom error")]
    Custom(String),
}

#[derive(Error, Deserialize, Debug)]
pub struct ResponseError {
    code: Option<u32>,
    message: String,
}

impl fmt::Display for ResponseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::result::Result<(), std::fmt::Error> {
        let code = self
            .code
            .map_or_else(|| "none".to_string(), |v| v.to_string());
        f.write_fmt(format_args!("{} ({})", self.message, code))
    }
}

impl Error {
    pub(crate) fn api_error<T: ToString>(msg: T) -> Self {
        Self::ApiError(msg.to_string())
    }

    pub(crate) fn custom<T: ToString>(msg: T) -> Self {
        Self::Custom(msg.to_string())
    }

    pub(crate) fn from_json_error(value: serde_json::Value) -> Self {
        match serde_json::from_value::<ResponseError>(value) {
            Ok(response_error) => Self::Response(response_error),
            Err(err) => Self::from(err),
        }
    }
}
