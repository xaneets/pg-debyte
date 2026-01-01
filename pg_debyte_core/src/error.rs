use crate::types::TypeKey;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum DecodeError {
    #[error("bad envelope: {0}")]
    BadEnvelope(&'static str),
    #[error("unknown type: {0:?}")]
    UnknownType(TypeKey),
    #[error("unknown action id: {0}")]
    UnknownAction(u16),
    #[error("unknown codec id: {0}")]
    UnknownCodec(u16),
    #[error("limit exceeded for {context}: limit={limit} actual={actual}")]
    LimitExceeded {
        context: &'static str,
        limit: usize,
        actual: usize,
    },
    #[error("serde error: {0}")]
    Serde(String),
    #[error("bincode error: {0}")]
    Bincode(String),
    #[error("zstd error: {0}")]
    Zstd(String),
    #[error("json error: {0}")]
    Json(String),
    #[error("io error: {0}")]
    Io(String),
}

impl From<serde_json::Error> for DecodeError {
    fn from(err: serde_json::Error) -> Self {
        Self::Json(err.to_string())
    }
}

impl From<std::io::Error> for DecodeError {
    fn from(err: std::io::Error) -> Self {
        Self::Io(err.to_string())
    }
}
