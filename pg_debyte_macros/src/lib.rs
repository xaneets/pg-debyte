#[macro_export]
macro_rules! declare_decoder {
    (
        $name:ident,
        ty = $ty:ty,
        type_id = $type_id:expr,
        schema_version = $schema_version:expr,
        codec = $codec:expr,
        codec_ty = $codec_ty:ty,
        actions = [$($action:expr),* $(,)?]
    ) => {
        pub static $name: pg_debyte_core::TypedDecoderEntry<$ty, $codec_ty> =
            pg_debyte_core::TypedDecoderEntry::new(
                pg_debyte_core::TypeKey {
                    type_id: $type_id,
                    schema_version: $schema_version,
                },
                $codec,
                &[$($action),*],
            );
    };
}

#[macro_export]
macro_rules! declare_know_schema {
    (
        $name:ident,
        ty = $ty:ty,
        type_id = $type_id:expr,
        schema_version = $schema_version:expr,
        codec = $codec:expr,
        codec_ty = $codec_ty:ty,
        actions = [$($action:expr),* $(,)?],
        fn_name = $fn_name:ident
    ) => {
        pub static $name: pg_debyte_core::TypedDecoderEntry<$ty, $codec_ty> =
            pg_debyte_core::TypedDecoderEntry::new(
                pg_debyte_core::TypeKey {
                    type_id: $type_id,
                    schema_version: $schema_version,
                },
                $codec,
                &[$($action),*],
            );

        #[pg_extern]
        fn $fn_name(data: Vec<u8>) -> Result<pgrx::JsonB, pg_debyte_core::error::DecodeError> {
            let limits = pg_debyte_pgrx::limits();
            let value = pg_debyte_pgrx::decode_know_schema(&data, &$name, &limits)?;
            Ok(pgrx::JsonB(value))
        }
    };
}
