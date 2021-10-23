use crate::types::*;

use serde_json::Value;

pub struct AddFields {
    field_mapping: FieldMapping,
}

impl AddFields {
    pub fn new(field_mapping: FieldMapping) -> Self {
        AddFields { field_mapping }
    }
}

impl Processor for AddFields {
    fn process(&mut self, mut instance: Instance) -> ProcessorResult {
        for (name, value) in &self.field_mapping {
            instance.insert(name.clone(), Value::String(value.clone()));
        }

        ProcessorResult::Ok(instance)
    }
}