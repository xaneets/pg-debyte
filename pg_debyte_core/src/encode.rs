use crate::action::ActionSpec;
use crate::codec::Codec;
use crate::envelope::build_envelope;
use crate::error::DecodeError;
use crate::registry::Registry;
use crate::types::{EncodeLimits, TypeKey};
use serde::Serialize;

pub fn encode_to_envelope<T, C>(
    value: &T,
    codec: &C,
    key: TypeKey,
    actions: &[ActionSpec],
    registry: &dyn Registry,
    limits: &EncodeLimits,
) -> Result<Vec<u8>, DecodeError>
where
    T: Serialize,
    C: Codec,
{
    let mut payload = codec.encode(value, limits)?;
    for action in actions {
        let handler = registry
            .lookup_action(action.id)
            .ok_or(DecodeError::UnknownAction(action.id))?;
        payload = handler.encode(&payload, limits, &action.params)?;
    }
    Ok(build_envelope(key, codec.id(), actions, &payload))
}
