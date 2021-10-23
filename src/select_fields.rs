use std::collections::HashSet;

use crate::types::*;

pub struct SelectFields {
    fields: HashSet<String>,
}

impl SelectFields {
    pub fn new(fields: Vec<String>) -> Self {
        SelectFields { fields: fields.into_iter().collect() }
    }
}

impl Processor for SelectFields {
    fn process(&mut self, instance: Instance) -> ProcessorResult {
        ProcessorResult::Ok(
            instance.into_iter().filter(|(k, _)| self.fields.contains(k)).collect()
        )
    }
}