use crate::types::*;
use crate::util::serialize_into_json_line;

pub struct OutputJson {}

impl OutputJson {
    pub fn new() -> Self {
        OutputJson {}
    }
}

impl InstanceSerializer for OutputJson {
    fn serialize(&self, instance: Instance) -> Result<String, RjpError> {
        Ok(serialize_into_json_line(&instance))
    }
}
