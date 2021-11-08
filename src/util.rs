use std::fs;
use std::io;
use std::io::BufRead;

use serde::ser::Serialize;
use serde_json::ser::Serializer;

use crate::json_lines_formatter::JsonLinesFormatter;
use crate::types::*;

pub const BUF_SIZE: usize = 8 * 1024;

pub fn lines_from_file(file_name: &str) -> InputStreamIterator {
    Box::new(io::BufReader::with_capacity(BUF_SIZE, fs::File::open(file_name).unwrap()).lines())
}

pub fn lines_from_stdin() -> InputStreamIterator {
    Box::new(io::BufReader::with_capacity(BUF_SIZE, io::stdin()).lines())
}

pub fn check_config(commands: &[String], length: usize) -> Result<(), RjpError> {
    if commands.len() < length {
        Err(RjpError::BadConfig(format!(
            "too few arguments, expected {}, got {}: {}",
            length,
            commands.len(),
            commands.join(" ")
        )))
    } else {
        Ok(())
    }
}

pub fn serialize_into_json_line(item: &Instance) -> String {
    // this is based on functions to_vec_pretty, to_string_pretty etc. in serde_json
    let mut writer: Vec<u8> = Vec::with_capacity(128);
    let mut serializer = Serializer::with_formatter(&mut writer, JsonLinesFormatter::new());

    item.serialize(&mut serializer).unwrap();

    unsafe { String::from_utf8_unchecked(writer) }
}

pub fn parse_mapping_def(mapping_str: &str) -> Result<FieldMapping, RjpError> {
    let mut mapping: FieldMapping = Vec::new();

    for field_pair in mapping_str.split(',') {
        let split: Vec<&str> = field_pair.split(':').collect();
        if split.len() != 2 {
            return Err(RjpError::BadConfig(format!(
                "expected format name:value,... (got {})",
                mapping_str
            )));
        }
        mapping.push((String::from(split[0]), String::from(split[1])));
    }

    Ok(mapping)
}

pub fn parse_comma_delimited_list(list_str: &str) -> Vec<String> {
    list_str.split(',').map(String::from).collect()
}
