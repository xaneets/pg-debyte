//! Example extension crate showing how to wire pg_debyte into PG17.

use pgrx::prelude::*;
use pgrx::JsonB;

#[cfg(any(test, feature = "pg_test"))]
pub mod pg_test {
    pub fn setup(_options: Vec<&str>) {}

    pub fn postgresql_conf_options() -> Vec<&'static str> {
        vec![]
    }
}
use pg_debyte_core::{BincodeCodec, DecodeError, StaticRegistry, ZstdAction};
use pg_debyte_macros::declare_decoder;
use serde::{Deserialize, Serialize};
use uuid::Uuid as CoreUuid;

pg_module_magic!();

#[derive(Debug, Deserialize, Serialize)]
struct DemoRecord {
    id: u32,
    label: String,
}

const DEMO_TYPE_ID: CoreUuid = CoreUuid::from_bytes([0x11; 16]);
const DEMO_SCHEMA_VERSION: u16 = 1;
const DEMO_CODEC_ID: u16 = 1;
const ZSTD_ACTION_ID: u16 = 1;

const DEMO_CODEC: BincodeCodec = BincodeCodec::new(DEMO_CODEC_ID, 32 * 1024 * 1024);
const ZSTD_ACTION: ZstdAction = ZstdAction::new(ZSTD_ACTION_ID);

declare_decoder!(
    DEMO_DECODER,
    ty = DemoRecord,
    type_id = DEMO_TYPE_ID,
    schema_version = DEMO_SCHEMA_VERSION,
    codec = DEMO_CODEC,
    codec_ty = BincodeCodec,
    actions = []
);

static REGISTRY: StaticRegistry = StaticRegistry::new(&[&DEMO_DECODER], &[&ZSTD_ACTION]);

#[pg_guard]
pub unsafe extern "C-unwind" fn _PG_init() {
    pg_debyte_pgrx::init_gucs();
    pg_debyte_pgrx::set_registry(&REGISTRY);
}

#[pg_extern]
fn bytea_to_json_by_id(
    data: Vec<u8>,
    type_id: pgrx::Uuid,
    schema_version: i16,
) -> Result<JsonB, DecodeError> {
    let limits = pg_debyte_pgrx::limits();
    let core_uuid = CoreUuid::from_bytes(*type_id.as_bytes());
    let value = pg_debyte_pgrx::decode_by_id(&data, core_uuid, schema_version, &limits)?;
    Ok(JsonB(value))
}

#[pg_extern]
fn bytea_to_json_auto(data: Vec<u8>) -> Result<JsonB, DecodeError> {
    let limits = pg_debyte_pgrx::limits();
    let value = pg_debyte_pgrx::decode_auto(&data, &limits)?;
    Ok(JsonB(value))
}

#[cfg(any(test, feature = "pg_test"))]
#[pg_schema]
mod tests {
    use super::*;
    use hex::encode;
    use pg_debyte_core::action::ActionSpec;
    use pg_debyte_core::codec::BincodeCodec;
    use pg_debyte_core::encode::encode_to_envelope;
    use pg_debyte_core::registry::StaticRegistry;
    use pg_debyte_core::types::{EncodeLimits, TypeKey};
    use serde_json::json;

    fn demo_envelope_hex() -> String {
        #[derive(Serialize)]
        struct DemoRecord {
            id: u32,
            label: String,
        }

        let record = DemoRecord {
            id: 1,
            label: "demo".to_string(),
        };
        let key = TypeKey {
            type_id: CoreUuid::from_bytes([0x11; 16]),
            schema_version: 1,
        };
        let codec = BincodeCodec::new(1, 32 * 1024 * 1024);
        let limits = EncodeLimits::new(32 * 1024 * 1024);
        let registry = StaticRegistry::new(&[], &[]);
        let actions: Vec<ActionSpec> = Vec::new();

        let envelope =
            encode_to_envelope(&record, &codec, key, &actions, &registry, &limits).unwrap();
        encode(envelope)
    }

    #[pg_test]
    fn test_bytea_to_json_by_id() {
        let json = Spi::get_one::<JsonB>(
            "SELECT bytea_to_json_by_id(decode('010464656d6f', 'hex'), \
             '11111111-1111-1111-1111-111111111111'::uuid, 1::smallint)",
        )
        .expect("spi")
        .expect("json");

        assert_eq!(json.0, json!({"id": 1, "label": "demo"}));
    }

    #[pg_test]
    fn test_bytea_to_json_auto() {
        let hex = demo_envelope_hex();
        let query = format!("SELECT bytea_to_json_auto(decode('{}', 'hex'))", hex);
        let json = Spi::get_one::<JsonB>(&query).expect("spi").expect("json");

        assert_eq!(json.0, json!({"id": 1, "label": "demo"}));
    }

    #[pg_test]
    fn test_auto_rejects_raw() {
        let ok = PgTryBuilder::new(|| {
            let _ =
                Spi::get_one::<JsonB>("SELECT bytea_to_json_auto(decode('010464656d6f', 'hex'))")
                    .expect("spi");
            true
        })
        .catch_others(|_| false)
        .execute();

        assert!(!ok);
    }
}
