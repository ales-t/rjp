use serde_json::Value;

use crate::types::*;

pub struct ToNumber {
    field_mapping: FieldMapping,
}

impl ToNumber {
    pub fn new(field_mapping: FieldMapping) -> Self {
        ToNumber { field_mapping }
    }
}

impl Processor for ToNumber {
    fn process(&mut self, mut instance: Instance) -> ProcessorResult {
        for (old_name, new_name) in &self.field_mapping {
            let val = instance.get(old_name).expect("error").to_string();
            let parsed_val: Value = serde_json::from_str(val.as_str()).unwrap();

            if !parsed_val.is_number() {
                return ProcessorResult::Error(RjpError::BadInput(format!(
                    "could not parse as number: {}",
                    val
                )));
            }

            instance.insert(new_name.clone(), parsed_val);
        }

        ProcessorResult::Ok(instance)
    }
}
