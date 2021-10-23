use crate::types::*;

pub struct RenameFields {
    field_mapping: FieldMapping,}

impl RenameFields {
    pub fn new(field_mapping: FieldMapping) -> Self {
        RenameFields { field_mapping }
    }
}

impl Processor for RenameFields {
    fn process(&mut self, mut instance: Instance) -> ProcessorResult {
        for (old_name, new_name) in &self.field_mapping {
            let val = instance.remove(old_name).expect("error");
            instance.insert(new_name.clone(), val);
        }

        ProcessorResult::Ok(instance)
    }
}