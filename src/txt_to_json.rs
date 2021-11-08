use serde_json::Value;

use crate::types::*;

pub struct TxtToJson {
    field_name: String,
}

impl TxtToJson {
    pub fn new(field_name: &str) -> Self {
        TxtToJson {
            field_name: String::from(field_name),
        }
    }
}

impl InstanceDeserializer for TxtToJson {
    fn deserialize(&self, instance_str: String) -> Result<Instance, RjpError> {
        let mut record = Instance::default();
        record.insert(self.field_name.clone(), Value::String(instance_str));

        Ok(record)
    }
}
