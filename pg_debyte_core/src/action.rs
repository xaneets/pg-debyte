use crate::error::DecodeError;
use crate::types::{DecodeLimits, EncodeLimits};
use std::io::Read;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ActionSpec {
    pub id: u16,
    pub flags: u8,
    pub params: Vec<u8>,
}

impl ActionSpec {
    pub fn new(id: u16, flags: u8, params: Vec<u8>) -> Self {
        Self { id, flags, params }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ActionSpecRef {
    pub id: u16,
    pub flags: u8,
    pub params: &'static [u8],
}

impl ActionSpecRef {
    pub const fn new(id: u16, flags: u8, params: &'static [u8]) -> Self {
        Self { id, flags, params }
    }
}

pub trait ByteAction: Send + Sync {
    fn id(&self) -> u16;
    fn decode(
        &self,
        input: &[u8],
        limits: &DecodeLimits,
        params: &[u8],
    ) -> Result<Vec<u8>, DecodeError>;
    fn encode(
        &self,
        input: &[u8],
        limits: &EncodeLimits,
        params: &[u8],
    ) -> Result<Vec<u8>, DecodeError>;
}

#[derive(Debug, Clone, Copy)]
pub struct ZstdAction {
    pub id: u16,
}

impl ZstdAction {
    pub const fn new(id: u16) -> Self {
        Self { id }
    }
}

impl ByteAction for ZstdAction {
    fn id(&self) -> u16 {
        self.id
    }

    fn decode(
        &self,
        input: &[u8],
        limits: &DecodeLimits,
        _params: &[u8],
    ) -> Result<Vec<u8>, DecodeError> {
        let mut decoder = zstd::stream::read::Decoder::new(input)
            .map_err(|err| DecodeError::Zstd(err.to_string()))?;
        let mut output = Vec::new();
        let mut buffer = [0u8; 8192];
        loop {
            let read = decoder.read(&mut buffer)?;
            if read == 0 {
                break;
            }
            if output.len().saturating_add(read) > limits.max_output_bytes {
                return Err(DecodeError::LimitExceeded {
                    context: "action_output_bytes",
                    limit: limits.max_output_bytes,
                    actual: output.len().saturating_add(read),
                });
            }
            output.extend_from_slice(&buffer[..read]);
        }
        Ok(output)
    }

    fn encode(
        &self,
        input: &[u8],
        limits: &EncodeLimits,
        params: &[u8],
    ) -> Result<Vec<u8>, DecodeError> {
        let level = params.first().map(|b| *b as i32).unwrap_or(0);
        let output =
            zstd::encode_all(input, level).map_err(|err| DecodeError::Zstd(err.to_string()))?;
        if output.len() > limits.max_output_bytes {
            return Err(DecodeError::LimitExceeded {
                context: "action_output_bytes",
                limit: limits.max_output_bytes,
                actual: output.len(),
            });
        }
        Ok(output)
    }
}
