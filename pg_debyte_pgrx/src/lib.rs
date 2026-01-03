use pg_debyte_core::action::{ActionSpec, ActionSpecRef};
use pg_debyte_core::envelope::{try_parse, ParsedEnvelope};
use pg_debyte_core::error::DecodeError;
use pg_debyte_core::registry::Registry;
use pg_debyte_core::types::{DecodeLimits, TypeKey};
use pg_debyte_core::DecoderEntry;
use pgrx::guc::{GucContext, GucFlags, GucRegistry, GucSetting};
use std::sync::OnceLock;
use uuid::Uuid;

static REGISTRY: OnceLock<&'static dyn Registry> = OnceLock::new();

const DEFAULT_MAX_BYTES: i32 = 950 * 1024 * 1024;

static MAX_INPUT_BYTES: GucSetting<i32> = GucSetting::<i32>::new(DEFAULT_MAX_BYTES);
static MAX_OUTPUT_BYTES: GucSetting<i32> = GucSetting::<i32>::new(DEFAULT_MAX_BYTES);
static MAX_JSON_BYTES: GucSetting<i32> = GucSetting::<i32>::new(DEFAULT_MAX_BYTES);

pub fn init_gucs() {
    GucRegistry::define_int_guc(
        c"pg_debyte.max_input_bytes",
        c"Maximum input byte length for pg_debyte",
        c"Guards against oversized bytea inputs",
        &MAX_INPUT_BYTES,
        0,
        i32::MAX,
        GucContext::Userset,
        GucFlags::default(),
    );
    GucRegistry::define_int_guc(
        c"pg_debyte.max_output_bytes",
        c"Maximum decoded output byte length for pg_debyte",
        c"Guards against oversized action output",
        &MAX_OUTPUT_BYTES,
        0,
        i32::MAX,
        GucContext::Userset,
        GucFlags::default(),
    );
    GucRegistry::define_int_guc(
        c"pg_debyte.max_json_bytes",
        c"Maximum JSON byte length for pg_debyte",
        c"Guards against oversized JSON output",
        &MAX_JSON_BYTES,
        0,
        i32::MAX,
        GucContext::Userset,
        GucFlags::default(),
    );
}

pub fn set_registry(registry: &'static dyn Registry) {
    let _ = REGISTRY.set(registry);
}

pub fn registry() -> Result<&'static dyn Registry, DecodeError> {
    REGISTRY
        .get()
        .copied()
        .ok_or(DecodeError::BadEnvelope("registry not initialized"))
}

pub fn limits() -> DecodeLimits {
    DecodeLimits::new(
        MAX_INPUT_BYTES.get() as usize,
        MAX_OUTPUT_BYTES.get() as usize,
        MAX_JSON_BYTES.get() as usize,
    )
}

pub fn decode_by_id(
    data: &[u8],
    type_id: Uuid,
    schema_version: i16,
    limits: &DecodeLimits,
) -> Result<serde_json::Value, DecodeError> {
    ensure_limit("input_bytes", data.len(), limits.max_input_bytes)?;
    let reg = registry()?;
    let key = TypeKey {
        type_id,
        schema_version: schema_version as u16,
    };
    let entry = reg
        .lookup_decoder(key)
        .ok_or(DecodeError::UnknownType(key))?;
    let payload = apply_actions_refs(reg, entry.default_actions(), data, limits)?;
    let value = entry.decode_payload(&payload, limits)?;
    ensure_json_limit(&value, limits)?;
    Ok(value)
}

pub fn decode_know_schema(
    data: &[u8],
    decoder: &dyn DecoderEntry,
    limits: &DecodeLimits,
) -> Result<serde_json::Value, DecodeError> {
    ensure_limit("input_bytes", data.len(), limits.max_input_bytes)?;
    let value = if decoder.default_actions().is_empty() {
        decoder.decode_payload(data, limits)?
    } else {
        let reg = registry()?;
        let payload = apply_actions_refs(reg, decoder.default_actions(), data, limits)?;
        decoder.decode_payload(&payload, limits)?
    };
    ensure_json_limit(&value, limits)?;
    Ok(value)
}

pub fn decode_auto(data: &[u8], limits: &DecodeLimits) -> Result<serde_json::Value, DecodeError> {
    ensure_limit("input_bytes", data.len(), limits.max_input_bytes)?;
    let reg = registry()?;
    let parsed = try_parse(data)?;
    let envelope = match parsed {
        ParsedEnvelope::None => return Err(DecodeError::BadEnvelope("no envelope")),
        ParsedEnvelope::Envelope(view) => view,
    };

    let entry = reg
        .lookup_decoder(envelope.key)
        .ok_or(DecodeError::UnknownType(envelope.key))?;
    if envelope.codec_id != entry.codec_id() {
        return Err(DecodeError::UnknownCodec(envelope.codec_id));
    }

    let payload = apply_actions(reg, &envelope.actions, envelope.payload, limits)?;
    let value = entry.decode_payload(&payload, limits)?;
    ensure_json_limit(&value, limits)?;
    Ok(value)
}

fn apply_actions(
    reg: &dyn Registry,
    actions: &[ActionSpec],
    payload: &[u8],
    limits: &DecodeLimits,
) -> Result<Vec<u8>, DecodeError> {
    let mut buffer = payload.to_vec();
    for action in actions.iter().rev() {
        let handler = reg
            .lookup_action(action.id)
            .ok_or(DecodeError::UnknownAction(action.id))?;
        buffer = handler.decode(&buffer, limits, &action.params)?;
    }
    Ok(buffer)
}

fn apply_actions_refs(
    reg: &dyn Registry,
    actions: &[ActionSpecRef],
    payload: &[u8],
    limits: &DecodeLimits,
) -> Result<Vec<u8>, DecodeError> {
    let mut buffer = payload.to_vec();
    for action in actions.iter().rev() {
        let handler = reg
            .lookup_action(action.id)
            .ok_or(DecodeError::UnknownAction(action.id))?;
        buffer = handler.decode(&buffer, limits, action.params)?;
    }
    Ok(buffer)
}

fn ensure_limit(context: &'static str, actual: usize, limit: usize) -> Result<(), DecodeError> {
    if actual > limit {
        return Err(DecodeError::LimitExceeded {
            context,
            limit,
            actual,
        });
    }
    Ok(())
}

fn ensure_json_limit(value: &serde_json::Value, limits: &DecodeLimits) -> Result<(), DecodeError> {
    let json = serde_json::to_vec(value)?;
    ensure_limit("json_bytes", json.len(), limits.max_json_bytes)
}
