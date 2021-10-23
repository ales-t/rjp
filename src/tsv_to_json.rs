use serde_json::Value;

use crate::types::*;

pub struct TsvToJson {
    field_names: Vec<String>,
}

impl TsvToJson {
    pub fn new(field_names: Vec<String>) -> Self {
        TsvToJson { field_names }
    }
}

impl InstanceDeserializer for TsvToJson {
    fn deserialize(&self, instance_str: String) -> Result<Instance, RjpError> {
        let mut record = Instance::default();

        let values: Vec<String> = instance_str.split("\t").map(String::from).collect();

        if values.len() != self.field_names.len() {
            return Err(RjpError::BadInput(format!(
                "mismatched number of columns on line: {}", &instance_str)));
        }

        for (name, val) in self.field_names.iter().zip(values) {
            record.insert(name.clone(), Value::String(val));
        }

        Ok(record)
    }
}