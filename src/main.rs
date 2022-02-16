use clap::ArgEnum;
use clap::Parser;
use rjp::builders::*;
use rjp::pipeline::main_worker::*;
use rjp::types::*;
use rjp::util;
use std::io;

/// Rapid JSON-lines processor
#[derive(Parser, Debug)]
#[clap(version, about, long_about = None)]
struct Args {
    /// How to handle lines with errors
    #[clap(arg_enum, default_value = "stop")]
    errors: ErrorHandling,

    /// Commands
    commands: Vec<String>,
}

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ArgEnum, Debug)]
enum ErrorHandling {
    Keep,
    Skip,
    Stop,
}

#[termination::display]
fn main() -> Result<(), RjpError> {
    let args = Args::parse();

    let mut commands: Vec<String> = args.commands.clone();

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
