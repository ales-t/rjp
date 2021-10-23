use regex::Regex;

use crate::types::*;
use crate::extract_items::ExtractedItem::{ArrayItem, ObjectItem};

#[derive(Debug)]
enum ExtractedItem {
    ArrayItem(String, usize),
    ObjectItem(String, String),
}

pub struct ExtractItems {
    field_mapping: Vec<(ExtractedItem, String)>,
}

impl ExtractItems {
    pub fn new(field_mapping: FieldMapping) -> Result<Self, RjpError> {
        let mut parsed_field_mapping: Vec<(ExtractedItem, String)> = Vec::new();

        for (item_descr, new_name) in &field_mapping {
            let parsed_descr = ExtractItems::parse_item_description(&item_descr)?;
            parsed_field_mapping.push((parsed_descr, new_name.clone()));
        }

        Ok(ExtractItems { field_mapping: parsed_field_mapping })
    }

    fn parse_item_description(item_descr: &str) -> Result<ExtractedItem, RjpError> {
        let re = Regex::new(r"^(.*)\[(.*)\]$").unwrap();

        if let Some(captured) = re.captures(item_descr) {
            let field_name = String::from(captured.get(1).unwrap().as_str());
            let key = String::from(captured.get(2).unwrap().as_str());

            if let Ok(position) = key.parse::<usize>() {
                Ok(ArrayItem(field_name, position))
            } else {
                Ok(ObjectItem(field_name, key))
            }

        } else {
            Err(RjpError::BadConfig(format!("malformed item description: {}", item_descr)))
        }
    }
}

impl Processor for ExtractItems {
    fn process(&mut self, mut instance: Instance) -> ProcessorResult {
        for (to_extract, new_name) in &self.field_mapping {
            let extracted = match to_extract {
                ArrayItem(field_name, position) => instance[field_name][position].clone(),
                ObjectItem(field_name, key) => instance[field_name][key].clone(),
            };
            instance.insert(new_name.clone(), extracted);
        }

        ProcessorResult::Ok(instance)
    }
}
