use crate::error::DecodeError;
use crate::types::{DecodeLimits, EncodeLimits};
use bincode::Options;
use serde::de::DeserializeOwned;
use serde::Serialize;

pub trait Codec: Send + Sync {
    fn id(&self) -> u16;
    fn decode<T: DeserializeOwned>(
        &self,
        bytes: &[u8],
        limits: &DecodeLimits,
    ) -> Result<T, DecodeError>;
    fn encode<T: Serialize>(
        &self,
        value: &T,
        limits: &EncodeLimits,
    ) -> Result<Vec<u8>, DecodeError>;
}

#[derive(Debug, Clone, Copy)]
pub struct BincodeCodec {
    pub id: u16,
    pub byte_limit: u64,
}

impl BincodeCodec {
    pub const fn new(id: u16, byte_limit: u64) -> Self {
        Self { id, byte_limit }
    }
}

impl Codec for BincodeCodec {
    fn id(&self) -> u16 {
        self.id
    }

    fn decode<T: DeserializeOwned>(
        &self,
        bytes: &[u8],
        limits: &DecodeLimits,
    ) -> Result<T, DecodeError> {
        let limit = self.byte_limit.min(limits.max_output_bytes as u64);
        if bytes.len() as u64 > limit {
            return Err(DecodeError::LimitExceeded {
                context: "codec_input_bytes",
                limit: limit as usize,
                actual: bytes.len(),
            });
        }
        bincode::DefaultOptions::new()
            .with_limit(limit)
            .deserialize(bytes)
            .map_err(|err| DecodeError::Bincode(err.to_string()))
    }

    fn encode<T: Serialize>(
        &self,
        value: &T,
        limits: &EncodeLimits,
    ) -> Result<Vec<u8>, DecodeError> {
        let limit = self.byte_limit.min(limits.max_output_bytes as u64);
        bincode::DefaultOptions::new()
            .with_limit(limit)
            .serialize(value)
            .map_err(|err| DecodeError::Bincode(err.to_string()))
    }
}
