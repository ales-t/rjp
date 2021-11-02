use crate::types::*;

use crate::parse_json::*;
use crate::tsv_to_json::*;
use crate::txt_to_json::*;

use crate::add_fields::*;
use crate::drop_fields::*;
use crate::extract_items::*;
use crate::join::*;
use crate::merge::*;
use crate::rename_fields::*;
use crate::select_fields::*;
use crate::to_number::*;

use crate::output_json::*;
use crate::output_tsv::*;

use crate::util::*;

fn build_deserializer(
    commands: &mut Vec<String>,
) -> Result<Box<dyn InstanceDeserializer>, RjpError> {
    if !commands.is_empty() {
        let cmd = commands.remove(0);
        match cmd.as_str() {
            "txt_to_json" | "from_txt" => {
                check_config(commands, 1)?;
                Ok(Box::new(TxtToJson::new(commands.remove(0).as_str())))
            }
            "tsv_to_json" | "from_tsv" => {
                check_config(commands, 1)?;
                Ok(Box::new(TsvToJson::new(parse_comma_delimited_list(
                    commands.remove(0).as_str(),
                ))))
            }
            _ => {
                commands.insert(0, cmd); // not a known input parser
                Ok(Box::new(ParseJson::new()))
            }
        }
    } else {
        Ok(Box::new(ParseJson::new()))
    }
}

pub fn build_input_stream(
    commands: &mut Vec<String>,
    reader_iter: InputStreamIterator,
) -> Result<InstanceIterator, RjpError> {
    let deserializer = build_deserializer(commands)?;
    Ok(Box::new(reader_iter.map(
        move |maybe_str| match maybe_str {
            Ok(instance_str) => deserializer.deserialize(instance_str),
            Err(error) => Err(RjpError::UnhandledError(error.to_string())),
        },
    )))
}

pub fn build_processor_chain(commands: &mut Vec<String>) -> Result<ProcessorList, RjpError> {
    let mut processors: ProcessorList = Vec::new();

    while !commands.is_empty() {
        let cmd = commands.remove(0);
        match cmd.as_str() {
            "rename" | "rnm" => {
                check_config(commands, 1)?;
                let mapping = parse_mapping_def(&commands.remove(0))?;
                processors.push(Box::new(RenameFields::new(mapping)));
            }
            "join" | "j" | "inner_join" => {
                check_config(commands, 2)?;
                let file_name = commands.remove(0);
                let key_fields = parse_comma_delimited_list(&commands.remove(0));
                let stream_to_merge = build_input_stream(commands, lines_from_file(&file_name))?;
                let processor = Join::new(stream_to_merge, key_fields, false)?;
                processors.push(Box::new(processor));
            }
            "lj" | "left_join" => {
                check_config(commands, 2)?;
                let file_name = commands.remove(0);
                let key_fields = parse_comma_delimited_list(&commands.remove(0));
                let stream_to_merge = build_input_stream(commands, lines_from_file(&file_name))?;
                let processor = Join::new(stream_to_merge, key_fields, true)?;
                processors.push(Box::new(processor));
            }
            "merge" | "mrg" => {
                check_config(commands, 1)?;
                let file_name = commands.remove(0);
                let stream_to_merge = build_input_stream(commands, lines_from_file(&file_name))?;
                processors.push(Box::new(Merge::new(stream_to_merge)));
            }
            "add_fields" | "af" | "add" => {
                check_config(commands, 1)?;
                let field_mapping = parse_mapping_def(&commands.remove(0))?;
                processors.push(Box::new(AddFields::new(field_mapping)));
            }
            "drop_fields" | "df" | "drop" => {
                check_config(commands, 1)?;
                let fields = parse_comma_delimited_list(&commands.remove(0));
                processors.push(Box::new(DropFields::new(fields)));
            }
            "extract_items" | "e" | "extract" => {
                check_config(commands, 1)?;
                let field_mapping = parse_mapping_def(&commands.remove(0))?;
                let processor = ExtractItems::new(field_mapping)?;
                processors.push(Box::new(processor));
            }
            "select_fields" | "sf" | "select" | "sel" => {
                check_config(commands, 1)?;
                let fields = parse_comma_delimited_list(&commands.remove(0));
                processors.push(Box::new(SelectFields::new(fields)));
            }
            "to_number" | "num" => {
                check_config(commands, 1)?;
                let field_mapping = parse_mapping_def(&commands.remove(0))?;
                processors.push(Box::new(ToNumber::new(field_mapping)));
            }
            _ => {
                commands.insert(0, cmd); // not a known processor
                break;
            }
        }
    }

    Ok(processors)
}

pub fn build_serializer(
    commands: &mut Vec<String>,
) -> Result<Box<dyn InstanceSerializer>, RjpError> {
    if !commands.is_empty() {
        match commands[0].as_str() {
            "to_tsv" | "json_to_tsv" | "tsv" => {
                commands.remove(0);
                check_config(commands, 1)?;
                let fields = parse_comma_delimited_list(&commands.remove(0));
                return Ok(Box::new(OutputTsv::new(fields)));
            }
            _ => {
                return Err(RjpError::BadConfig(format!(
                    "unknown command: {}",
                    commands[0].as_str()
                )));
            }
        }
    }

    Ok(Box::new(OutputJson::new()))
}
