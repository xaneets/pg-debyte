use pg_debyte_core::envelope::try_parse;
use pg_debyte_core::error::DecodeError;
use uuid::Uuid;

fn base_header(version: u8, actions_count: u8) -> Vec<u8> {
    let mut bytes = Vec::new();
    bytes.extend_from_slice(b"PGDEBYTE");
    bytes.push(version);
    bytes.extend_from_slice(Uuid::from_bytes([0x11; 16]).as_bytes());
    bytes.extend_from_slice(&1u16.to_le_bytes());
    bytes.extend_from_slice(&2u16.to_le_bytes());
    bytes.push(actions_count);
    bytes
}

#[test]
fn envelope_unsupported_version() {
    let bytes = base_header(2, 0);
    let err = try_parse(&bytes).expect_err("expected error");
    match err {
        DecodeError::BadEnvelope(msg) => assert_eq!(msg, "unsupported envelope version"),
        other => panic!("unexpected error: {other:?}"),
    }
}

#[test]
fn envelope_action_header_out_of_bounds() {
    let bytes = base_header(1, 1);
    let err = try_parse(&bytes).expect_err("expected error");
    match err {
        DecodeError::BadEnvelope(msg) => assert_eq!(msg, "action header out of bounds"),
        other => panic!("unexpected error: {other:?}"),
    }
}

#[test]
fn envelope_params_out_of_bounds() {
    let mut bytes = base_header(1, 1);
    bytes.extend_from_slice(&9u16.to_le_bytes());
    bytes.push(0);
    bytes.extend_from_slice(&8u16.to_le_bytes());
    bytes.extend_from_slice(b"abc");

    let err = try_parse(&bytes).expect_err("expected error");
    match err {
        DecodeError::BadEnvelope(msg) => assert_eq!(msg, "params out of bounds"),
        other => panic!("unexpected error: {other:?}"),
    }
}
