use std::io::Write;

use crate::builders::*;
use crate::types::*;

pub fn main_worker<T: std::io::Write>(
    mut commands: Vec<String>,
    in_stream: InstanceIterator,
    writer: &mut std::io::BufWriter<T>,
) -> Result<(i32, i32), RjpError> {
    /* Entry function for actual processing that can be imported from code and tested */

    // create processors
    let mut processors = build_processor_chain(&mut commands)?;

    // get output serializer
    let serializer = build_serializer(&mut commands)?;

    let mut total = 0;
    let mut removed = 0;

    // process the instances
    for maybe_instance in in_stream {
        total += 1;

        let mut maybe_filtered = Some(maybe_instance?);

        for proc in &mut processors {
            match proc.process(maybe_filtered.unwrap()) {
                ProcessorResult::Ok(processed) => maybe_filtered = Some(processed),
                ProcessorResult::Error(err) => return Err(err),
                ProcessorResult::Remove => {
                    maybe_filtered = None;
                    break;
                }
            }
        }

        if let Some(instance) = maybe_filtered {
            let serialized = serializer.serialize(instance)?;

            // gracefully handle potential errors on write
            if let Err(err) = writer.write((serialized + "\n").as_bytes()) {
                return if err.kind() == std::io::ErrorKind::BrokenPipe {
                    Ok((total, removed))
                } else {
                    Err(RjpError::UnhandledError(err.to_string()))
                };
            }
        } else {
            removed += 1;
            continue;
        }
    }

    Ok((total, removed))
}
