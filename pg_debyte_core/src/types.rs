use uuid::Uuid;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct TypeKey {
    pub type_id: Uuid,
    pub schema_version: u16,
}

#[derive(Debug, Clone, Copy)]
pub struct DecodeLimits {
    pub max_input_bytes: usize,
    pub max_output_bytes: usize,
    pub max_json_bytes: usize,
}

impl DecodeLimits {
    pub fn new(max_input_bytes: usize, max_output_bytes: usize, max_json_bytes: usize) -> Self {
        Self {
            max_input_bytes,
            max_output_bytes,
            max_json_bytes,
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct EncodeLimits {
    pub max_output_bytes: usize,
}

impl EncodeLimits {
    pub fn new(max_output_bytes: usize) -> Self {
        Self { max_output_bytes }
    }
}
