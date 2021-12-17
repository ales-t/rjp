use rjp::builders::*;
use rjp::pipeline::main_worker::*;
use rjp::types::*;
use rjp::util;
use std::env;
use std::io;

#[termination::display]
fn main() -> Result<(), RjpError> {
    // bootstraps the program from command line

    let mut commands: Vec<String> = env::args().collect();
    // pop program name
    commands.remove(0);

    // parse input stream
    let in_stream = build_input_stream(&mut commands, util::lines_from_stdin())?;

    // writer of output stream to stdout
    let mut writer = io::BufWriter::with_capacity(util::BUF_SIZE, io::stdout());

    // put input commands and input stream to the shadow main function
    let (total, removed) = main_worker(commands, in_stream, &mut writer)?;

    // final statistics
    eprintln!(
        "[rjp] Processed {} instances, {} instances were removed by filters.",
        total, removed
    );

    Ok(())
}
