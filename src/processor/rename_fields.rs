use crate::types::*;

pub struct RenameFields {
    field_mapping: FieldMapping,
}

impl RenameFields {
    pub fn new(field_mapping: FieldMapping) -> Self {
        RenameFields { field_mapping }
    }
}

impl Processor for RenameFields {
    fn process(&mut self, mut instance: Instance) -> ProcessorResult {
        for (old_name, new_name) in &self.field_mapping {
            if let Some(val) = instance.remove(old_name) {
                instance.insert(new_name.clone(), val);
            } else {
                return ProcessorResult::Error(RjpError::BadInput(format!(
                    "missing field {}",
                    old_name
                )));
            }
        }

        ProcessorResult::Ok(instance)
    }
}
