use std::env;
use std::io;
use std::io::Write;

use rayon::iter::IntoParallelRefIterator;
use rayon::iter::ParallelIterator;
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


fn run_processor_chain(mut instance: Instance, processors: &mut ProcessorList
) -> Result<Option<Instance>, RjpError> {
    for proc in processors {
        match proc.process(instance) {
            ProcessorResult::Ok(processed) => instance = processed,
            ProcessorResult::Error(err) => return Err(err),
            ProcessorResult::Remove => {
                return Ok(None);
            }
        }
    }

    Ok(Some(instance))
}


fn write_instance(instance: Instance, writer: &mut impl Write, serializer: &Box<dyn InstanceSerializer>
) -> Result<(), RjpError> {
    let serialized = serializer.serialize(instance)?;

    // gracefully handle potential errors on write
    if let Err(err) = writer.write((serialized + "\n").as_bytes()) {
        return if err.kind() == std::io::ErrorKind::BrokenPipe {
            Err(RjpError::BrokenPipeError())
        } else {
            Err(RjpError::UnhandledError(err.to_string()))
        };
    }

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
    let batch_size = 10000;
    let mut batch: Vec<Instance> = Vec::with_capacity(batch_size);

    for maybe_instance in in_stream {
        if batch.len() < batch_size {
            batch.push(maybe_instance?);
        } else {
            batch.par_iter().map(|mut instance| run_processor_chain(instance.to_owned(), &mut processors));
        }
    }

    for maybe_instance in in_stream {
        total += 1;

        let maybe_filtered = run_processor_chain(maybe_instance?, &mut processors)?;

        if let Some(instance) = maybe_filtered {
            match write_instance(instance, &mut writer, &serializer) {
                Err(err) => {
                    match err {
                        RjpError::BrokenPipeError() => return Ok((total, removed)),
                        RjpError::UnhandledError(msg) => return Err(RjpError::UnhandledError(msg)),
                        _ => unreachable!(),
                    }
                }
                Ok(()) => (),
            }
        } else {
            removed += 1;
        }
    }

    Ok((total, removed))
}
