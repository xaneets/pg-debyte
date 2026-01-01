pub mod action;
pub mod codec;
pub mod encode;
pub mod envelope;
pub mod error;
pub mod registry;
pub mod types;

pub use action::{ActionSpec, ActionSpecRef, ByteAction, ZstdAction};
pub use codec::{BincodeCodec, Codec};
pub use encode::encode_to_envelope;
pub use envelope::{EnvelopeView, ParsedEnvelope};
pub use error::DecodeError;
pub use registry::{DecoderEntry, Registry, StaticRegistry, TypedDecoderEntry};
pub use types::{DecodeLimits, EncodeLimits, TypeKey};
