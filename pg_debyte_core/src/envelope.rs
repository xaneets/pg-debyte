use crate::action::ActionSpec;
use crate::error::DecodeError;
use crate::types::TypeKey;
use uuid::Uuid;

const MAGIC: &[u8; 8] = b"PGDEBYTE";
const ENVELOPE_VERSION: u8 = 1;
const MIN_HEADER_LEN: usize = 8 + 1 + 16 + 2 + 2 + 1;

#[derive(Debug)]
pub struct EnvelopeView<'a> {
    pub key: TypeKey,
    pub codec_id: u16,
    pub actions: Vec<ActionSpec>,
    pub payload: &'a [u8],
}

#[derive(Debug)]
pub enum ParsedEnvelope<'a> {
    None,
    Envelope(EnvelopeView<'a>),
}

pub fn try_parse(input: &[u8]) -> Result<ParsedEnvelope<'_>, DecodeError> {
    if input.len() < MIN_HEADER_LEN {
        return Ok(ParsedEnvelope::None);
    }
    if &input[..MAGIC.len()] != MAGIC {
        return Ok(ParsedEnvelope::None);
    }

    let mut offset = MAGIC.len();
    let envelope_version = input[offset];
    offset += 1;
    if envelope_version != ENVELOPE_VERSION {
        return Err(DecodeError::BadEnvelope("unsupported envelope version"));
    }

    let type_id = Uuid::from_bytes(
        input[offset..offset + 16]
            .try_into()
            .map_err(|_| DecodeError::BadEnvelope("invalid type_id length"))?,
    );
    offset += 16;

    let schema_version = u16::from_le_bytes(
        input[offset..offset + 2]
            .try_into()
            .map_err(|_| DecodeError::BadEnvelope("invalid schema_version length"))?,
    );
    offset += 2;

    let codec_id = u16::from_le_bytes(
        input[offset..offset + 2]
            .try_into()
            .map_err(|_| DecodeError::BadEnvelope("invalid codec_id length"))?,
    );
    offset += 2;

    let actions_count = input[offset];
    offset += 1;

    let mut actions = Vec::with_capacity(actions_count as usize);
    for _ in 0..actions_count {
        if input.len() < offset + 2 + 1 + 2 {
            return Err(DecodeError::BadEnvelope("action header out of bounds"));
        }
        let action_id = u16::from_le_bytes(
            input[offset..offset + 2]
                .try_into()
                .map_err(|_| DecodeError::BadEnvelope("invalid action_id length"))?,
        );
        offset += 2;
        let flags = input[offset];
        offset += 1;
        let params_len = u16::from_le_bytes(
            input[offset..offset + 2]
                .try_into()
                .map_err(|_| DecodeError::BadEnvelope("invalid params_len length"))?,
        ) as usize;
        offset += 2;
        if input.len() < offset + params_len {
            return Err(DecodeError::BadEnvelope("params out of bounds"));
        }
        let params = input[offset..offset + params_len].to_vec();
        offset += params_len;
        actions.push(ActionSpec::new(action_id, flags, params));
    }

    if input.len() < offset {
        return Err(DecodeError::BadEnvelope("payload out of bounds"));
    }

    let payload = &input[offset..];
    let key = TypeKey {
        type_id,
        schema_version,
    };

    Ok(ParsedEnvelope::Envelope(EnvelopeView {
        key,
        codec_id,
        actions,
        payload,
    }))
}

pub fn build_envelope(
    key: TypeKey,
    codec_id: u16,
    actions: &[ActionSpec],
    payload: &[u8],
) -> Vec<u8> {
    let mut output = Vec::new();
    output.extend_from_slice(MAGIC);
    output.push(ENVELOPE_VERSION);
    output.extend_from_slice(key.type_id.as_bytes());
    output.extend_from_slice(&key.schema_version.to_le_bytes());
    output.extend_from_slice(&codec_id.to_le_bytes());
    output.push(actions.len() as u8);
    for action in actions {
        output.extend_from_slice(&action.id.to_le_bytes());
        output.push(action.flags);
        output.extend_from_slice(&(action.params.len() as u16).to_le_bytes());
        output.extend_from_slice(&action.params);
    }
    output.extend_from_slice(payload);
    output
}
