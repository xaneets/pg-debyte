use pg_debyte_core::action::{ActionSpec, ZstdAction};
use pg_debyte_core::codec::{BincodeCodec, Codec};
use pg_debyte_core::encode::encode_to_envelope;
use pg_debyte_core::envelope::{try_parse, ParsedEnvelope};
use pg_debyte_core::registry::StaticRegistry;
use pg_debyte_core::types::{EncodeLimits, TypeKey};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Deserialize, Serialize, PartialEq)]
struct Demo {
    id: u32,
    name: String,
}

#[test]
fn encode_builds_envelope() {
    let demo = Demo {
        id: 7,
        name: "test".to_string(),
    };
    let key = TypeKey {
        type_id: Uuid::from_bytes([2; 16]),
        schema_version: 1,
    };
    let codec = BincodeCodec::new(1, 1024);
    let limits = EncodeLimits::new(1024);
    let registry = StaticRegistry::new(&[], &[]);
    let actions: Vec<ActionSpec> = Vec::new();

    let encoded =
        encode_to_envelope(&demo, &codec, key, &actions, &registry, &limits).expect("encode");

    let parsed = try_parse(&encoded).expect("parse");
    let view = match parsed {
        ParsedEnvelope::Envelope(view) => view,
        ParsedEnvelope::None => panic!("expected envelope"),
    };

    assert_eq!(view.key, key);
    assert_eq!(view.codec_id, codec.id());
    assert_eq!(view.actions.len(), 0);

    let decoded: Demo = codec
        .decode(
            view.payload,
            &pg_debyte_core::DecodeLimits::new(1024, 1024, 1024),
        )
        .expect("decode payload");
    assert_eq!(decoded, demo);
}

#[test]
fn encode_roundtrip_with_actions() {
    let demo = Demo {
        id: 9,
        name: "actions".to_string(),
    };
    let key = TypeKey {
        type_id: Uuid::from_bytes([3; 16]),
        schema_version: 2,
    };
    let codec = BincodeCodec::new(2, 1024);
    let limits = EncodeLimits::new(1024);
    static ACTION: ZstdAction = ZstdAction::new(7);
    static ACTIONS: [&'static dyn pg_debyte_core::action::ByteAction; 1] = [&ACTION];
    let registry = StaticRegistry::new(&[], &ACTIONS);
    let actions = vec![ActionSpec::new(7, 1, vec![1])];

    let encoded =
        encode_to_envelope(&demo, &codec, key, &actions, &registry, &limits).expect("encode");

    let parsed = try_parse(&encoded).expect("parse");
    let view = match parsed {
        ParsedEnvelope::Envelope(view) => view,
        ParsedEnvelope::None => panic!("expected envelope"),
    };

    assert_eq!(view.key, key);
    assert_eq!(view.codec_id, codec.id());
    assert_eq!(view.actions.len(), 1);
    assert_eq!(view.actions[0].id, 7);
    assert_eq!(view.actions[0].flags, 1);
    assert_eq!(view.actions[0].params, [1]);
}

#[test]
fn encode_rejects_unknown_action() {
    let demo = Demo {
        id: 10,
        name: "unknown".to_string(),
    };
    let key = TypeKey {
        type_id: Uuid::from_bytes([4; 16]),
        schema_version: 1,
    };
    let codec = BincodeCodec::new(3, 1024);
    let limits = EncodeLimits::new(1024);
    let registry = StaticRegistry::new(&[], &[]);
    let actions = vec![ActionSpec::new(99, 0, vec![0])];

    let err = encode_to_envelope(&demo, &codec, key, &actions, &registry, &limits)
        .expect_err("expected unknown action");
    match err {
        pg_debyte_core::error::DecodeError::UnknownAction(id) => assert_eq!(id, 99),
        other => panic!("unexpected error: {other:?}"),
    }
}
