use pg_debyte_core::registry::{Registry, StaticRegistry};
use pg_debyte_core::types::TypeKey;
use uuid::Uuid;

#[test]
fn registry_lookup_missing_entries() {
    let registry = StaticRegistry::new(&[], &[]);
    let key = TypeKey {
        type_id: Uuid::from_bytes([9; 16]),
        schema_version: 1,
    };

    assert!(registry.lookup_decoder(key).is_none());
    assert!(registry.lookup_action(42).is_none());
}
