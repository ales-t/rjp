use clap::{App, Arg, ArgEnum, PossibleValue};
use rjp::builders::*;
use rjp::pipeline::main_worker::*;
use rjp::types::*;
use rjp::util;
use std::env;
use std::io;

#[derive(Copy, Clone, PartialEq, Eq, ArgEnum)]
enum BadInstanceHandling {
    SKIP,
    STOP,
    KEEP,
}

impl BadInstanceHandling {
    pub fn possible_values() -> impl Iterator<Item = PossibleValue<'static>> {
        BadInstanceHandling::value_variants()
            .iter()
            .filter_map(ArgEnum::to_possible_value)
    }
}


#[derive(Copy, Clone, PartialEq, Eq, ArgEnum)]
enum FromToOptions {
    TSV,
    JSON,
}

impl FromToOptions {
    pub fn possible_values() -> impl Iterator<Item = PossibleValue<'static>> {
        FromToOptions::value_variants()
            .iter()
            .filter_map(ArgEnum::to_possible_value)
    }
}

#[termination::display]
fn main() -> Result<(), RjpError> {
    // bootstraps the program from command line

    let args = App::new("rjp")
        .arg(
            Arg::new("processor")
                .help("A command to execute")
                .multiple_values(true)
                .takes_value(true)
                .required(true),
        )
        .arg(
            Arg::new("bad_instance")
                .long("--bad-instance")
                .possible_values(BadInstanceHandling::possible_values())
                .default_value("skip")
                .help("Handling of bad instances")
                .takes_value(true),
        )
        .arg(
            Arg::new("from")
                .long("--from")
                .short('f')
                .possible_values(FromToOptions::possible_values())
                .default_value("json")
                .help("From input format")
                .takes_value(true),
        )
        .arg(
            Arg::new("to")
                .long("--to")
                .short('t')
                .possible_values(FromToOptions::possible_values())
                .default_value("json")
                .help("To input format")
                .takes_value(true),
        )
        .get_matches();

    
    println!("{:?}", args.values_of("processor").unwrap().collect::<Vec<&str>>());
    println!("{:?}", args.value_of("bad_instance"));
    println!("{:?}", args.value_of("to"));
    println!("{:?}", args.value_of("from"));

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
