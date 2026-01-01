use bincode::Options;
use pg_debyte_core::action::ZstdAction;
use pg_debyte_core::codec::{BincodeCodec, Codec};
use pg_debyte_core::error::DecodeError;
use pg_debyte_core::types::DecodeLimits;
use pg_debyte_core::ByteAction;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, PartialEq)]
struct Demo {
    id: u32,
    name: String,
}

#[test]
fn bincode_decode_roundtrip() {
    let demo = Demo {
        id: 1,
        name: "demo".to_string(),
    };
    let bytes = bincode::DefaultOptions::new()
        .with_limit(1024)
        .serialize(&demo)
        .expect("serialize");
    let codec = BincodeCodec::new(1, 1024);
    let limits = DecodeLimits::new(1024, 1024, 1024);
    let decoded: Demo = codec.decode(&bytes, &limits).expect("decode");
    assert_eq!(decoded, demo);
}

#[test]
fn zstd_decode_respects_limit() {
    let payload = b"hello hello hello";
    let encoded = zstd::encode_all(&payload[..], 0).expect("encode");
    let action = ZstdAction::new(1);
    let limits = DecodeLimits::new(1024, 4, 1024);
    let err = action
        .decode(&encoded, &limits, &[])
        .expect_err("expected limit error");
    match err {
        DecodeError::LimitExceeded { context, .. } => {
            assert_eq!(context, "action_output_bytes");
        }
        other => panic!("unexpected error: {other:?}"),
    }
}
