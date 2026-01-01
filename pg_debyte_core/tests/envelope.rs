use pg_debyte_core::envelope::{try_parse, ParsedEnvelope};
use uuid::Uuid;

#[test]
fn envelope_parses() {
    let type_id = Uuid::from_bytes([1; 16]);
    let schema_version = 42u16;
    let codec_id = 7u16;
    let payload = b"payload";

    let mut bytes = Vec::new();
    bytes.extend_from_slice(b"PGDEBYTE");
    bytes.push(1);
    bytes.extend_from_slice(type_id.as_bytes());
    bytes.extend_from_slice(&schema_version.to_le_bytes());
    bytes.extend_from_slice(&codec_id.to_le_bytes());
    bytes.push(1);
    bytes.extend_from_slice(&9u16.to_le_bytes());
    bytes.push(0);
    bytes.extend_from_slice(&3u16.to_le_bytes());
    bytes.extend_from_slice(b"abc");
    bytes.extend_from_slice(payload);

    let parsed = try_parse(&bytes).expect("parse");
    let view = match parsed {
        ParsedEnvelope::Envelope(view) => view,
        ParsedEnvelope::None => panic!("expected envelope"),
    };

    assert_eq!(view.key.type_id, type_id);
    assert_eq!(view.key.schema_version, schema_version);
    assert_eq!(view.codec_id, codec_id);
    assert_eq!(view.actions.len(), 1);
    assert_eq!(view.actions[0].id, 9);
    assert_eq!(view.actions[0].params, b"abc");
    assert_eq!(view.payload, payload);
}

#[test]
fn envelope_missing_magic() {
    let bytes = b"not-an-envelope";
    let parsed = try_parse(bytes).expect("parse");
    match parsed {
        ParsedEnvelope::None => {}
        ParsedEnvelope::Envelope(_) => panic!("expected none"),
    }
}
