use std::env;
use std::io;
use std::io::Write;

use rjp::builders::*;
use rjp::types::*;
use rjp::util;

#[termination::display]
fn main() -> Result<(), RjpError> {
    // bootstraps the program from command line

    let mut commands: Vec<String> = env::args().collect();
    commands.remove(0); // program name

    // parse input stream
    let in_stream = build_input_stream(&mut commands, util::lines_from_stdin())?;

    // put input commands and input stream to the shadow main function
    let (total, removed) = main_entry(commands, in_stream)?;

    eprintln!(
        "[rjp] Processed {} instances, {} instances were removed by filters.",
        total, removed
    );

    Ok(())
}

fn main_entry(
    mut commands: Vec<String>,
    in_stream: InstanceIterator,
) -> Result<(i32, i32), RjpError> {
    // entry function for actual processing that can be imported from code and tested
    // TODO: rename to something more fitting

    // create processors
    let mut processors = build_processor_chain(&mut commands)?;

    // get output serializer
    let serializer = build_serializer(&mut commands)?;

    // writer of output stream to stdout
    let mut writer = io::BufWriter::with_capacity(util::BUF_SIZE, io::stdout());

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
