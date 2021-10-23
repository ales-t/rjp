use crate::types::*;

pub struct DropFields {
    fields: Vec<String>,
}

impl DropFields {
    pub fn new(fields: Vec<String>) -> Self {
        DropFields { fields}
    }
}

impl Processor for DropFields {
    fn process(&mut self, mut instance: Instance) -> ProcessorResult {
        for to_drop in &self.fields {
            instance.remove(to_drop);
        }

        ProcessorResult::Ok(instance)
    }
}
