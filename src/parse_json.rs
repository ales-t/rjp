use crate::types::*;

pub struct ParseJson {}

impl ParseJson {
    pub fn new() -> Self {
        ParseJson {}
    }
}

impl InstanceDeserializer for ParseJson {
    fn deserialize(&self, instance_str: String) -> Result<Instance, RjpError> {
        match serde_json::from_str(&instance_str) {
            Ok(record) => Ok(record),
            Err(error) => Err(RjpError::BadInput(error.to_string())),
        }
    }
}
