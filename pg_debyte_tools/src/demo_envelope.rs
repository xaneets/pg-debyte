use hex::encode;
use pg_debyte_core::action::ActionSpec;
use pg_debyte_core::codec::BincodeCodec;
use pg_debyte_core::encode::encode_to_envelope;
use pg_debyte_core::registry::StaticRegistry;
use pg_debyte_core::types::{EncodeLimits, TypeKey};
use serde::Serialize;
use uuid::Uuid;

#[derive(Debug, Serialize)]
struct DemoRecord {
    id: u32,
    label: String,
}

fn main() {
    let record = DemoRecord {
        id: 1,
        label: "demo".to_string(),
    };

    let key = TypeKey {
        type_id: Uuid::from_bytes([0x11; 16]),
        schema_version: 1,
    };
    let codec = BincodeCodec::new(1, 32 * 1024 * 1024);
    let limits = EncodeLimits::new(32 * 1024 * 1024);
    let registry = StaticRegistry::new(&[], &[]);
    let actions: Vec<ActionSpec> = Vec::new();

    let envelope = encode_to_envelope(&record, &codec, key, &actions, &registry, &limits)
        .expect("encode envelope");

    println!("{}", encode(envelope));
}
