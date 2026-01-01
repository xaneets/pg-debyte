use crate::action::ActionSpecRef;
use crate::codec::Codec;
use crate::error::DecodeError;
use crate::types::{DecodeLimits, TypeKey};
use serde::de::DeserializeOwned;
use serde::Serialize;
use std::marker::PhantomData;

pub trait DecoderEntry: Send + Sync {
    fn key(&self) -> TypeKey;
    fn codec_id(&self) -> u16;
    fn default_actions(&self) -> &'static [ActionSpecRef];
    fn decode_payload(
        &self,
        payload: &[u8],
        limits: &DecodeLimits,
    ) -> Result<serde_json::Value, DecodeError>;
}

pub struct TypedDecoderEntry<T, C> {
    key: TypeKey,
    codec: C,
    default_actions: &'static [ActionSpecRef],
    _marker: PhantomData<T>,
}

impl<T, C> TypedDecoderEntry<T, C> {
    pub const fn new(key: TypeKey, codec: C, default_actions: &'static [ActionSpecRef]) -> Self {
        Self {
            key,
            codec,
            default_actions,
            _marker: PhantomData,
        }
    }
}

impl<T, C> DecoderEntry for TypedDecoderEntry<T, C>
where
    T: DeserializeOwned + Serialize + Send + Sync,
    C: Codec + Send + Sync,
{
    fn key(&self) -> TypeKey {
        self.key
    }

    fn codec_id(&self) -> u16 {
        self.codec.id()
    }

    fn default_actions(&self) -> &'static [ActionSpecRef] {
        self.default_actions
    }

    fn decode_payload(
        &self,
        payload: &[u8],
        limits: &DecodeLimits,
    ) -> Result<serde_json::Value, DecodeError> {
        let value: T = self.codec.decode(payload, limits)?;
        serde_json::to_value(value).map_err(|err| DecodeError::Serde(err.to_string()))
    }
}

pub trait Registry: Send + Sync {
    fn lookup_decoder(&self, key: TypeKey) -> Option<&'static dyn DecoderEntry>;
    fn lookup_action(&self, id: u16) -> Option<&'static dyn crate::action::ByteAction>;
}

pub struct StaticRegistry {
    decoders: &'static [&'static dyn DecoderEntry],
    actions: &'static [&'static dyn crate::action::ByteAction],
}

impl StaticRegistry {
    pub const fn new(
        decoders: &'static [&'static dyn DecoderEntry],
        actions: &'static [&'static dyn crate::action::ByteAction],
    ) -> Self {
        Self { decoders, actions }
    }
}

impl Registry for StaticRegistry {
    fn lookup_decoder(&self, key: TypeKey) -> Option<&'static dyn DecoderEntry> {
        self.decoders
            .iter()
            .copied()
            .find(|entry| entry.key() == key)
    }

    fn lookup_action(&self, id: u16) -> Option<&'static dyn crate::action::ByteAction> {
        self.actions
            .iter()
            .copied()
            .find(|action| action.id() == id)
    }
}
