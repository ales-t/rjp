use crate::types::*;
use crate::util;

pub struct OutputTsv {
    fields: Vec<String>,
}

impl OutputTsv {
    pub fn new(fields: Vec<String>) -> Self {
        OutputTsv { fields }
    }
}

impl InstanceSerializer for OutputTsv {
    fn serialize(&self, instance: Instance) -> Result<String, RjpError> {
        let mut out = Vec::<String>::with_capacity(self.fields.len());
        for field_name in self.fields.iter() {
            if !instance.contains_key(field_name) {
                return Err(RjpError::BadInput(format!(
                    "instance does not contain field {}: {}",
                    field_name,
                    util::serialize_into_json_line(&instance)
                )));
            } else {
                let val = &instance[field_name];
                if val.is_string() {
                    // minor optimization, avoid calling to_string() on strings
                    out.push(val.as_str().unwrap().replace("\t", "\\t"));
                } else {
                    out.push(val.to_string());
                }
            }
        }

        Ok(out.join("\t"))
    }
}
