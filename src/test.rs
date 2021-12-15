#[cfg(test)]
mod tests {
    use crate::builders::*;
    use crate::main_worker::*;
    use crate::util;
    use std::path::Path;
    use std::{fs, io};

    #[test]
    fn test_in_dirs() {
        // Runs all tests in the `tests` directory. This assumes that the directory `tests` contains multiple
        // directories, each with at least two files: `command` and `output`. The command is executed and
        // compared with the output (assert). The test is run from the top-level project directory
        // but arguments in the command ending with `.json` and `.tsv` are automatically prefixed
        // with the path to the subdirectory. The first parameter of the command is the file that's
        // redirected to the worker (imagine prefixing the whole command with a `<`).
        // To add more tests, simply copy one of the existing folders.

        let test_dirs_path =
            fs::read_dir(Path::new(env!("CARGO_MANIFEST_DIR")).join(Path::new("tests"))).unwrap();
        for path in test_dirs_path {
            // process individual tests in the directory
            let path = path.unwrap().path();
            let command_path = path.join(Path::new("command"));
            let output_path = path.join(Path::new("output"));

            if !std::path::Path::new(&command_path).exists() {
                panic!(
                    "Could not perform test for {:?} because the associated \"command\" is missing",
                    command_path.file_stem().unwrap()
                );
            }
            let command_raw = fs::read_to_string(command_path).unwrap();
            let mut commands = command_raw
                .split_ascii_whitespace()
                .map(|s| s.to_owned())
                .collect::<Vec<String>>();

            // Prefix everything that ends with ".json" or ".tsv" with the current path
            // TODO: this is a hacky solution and needs a more robust approach
            // TODO: will break when more files are added
            commands = commands
                .into_iter()
                .map(|command| {
                    if command.ends_with(".json") || command.ends_with(".tsv") {
                        String::from(path.join(command).to_str().unwrap())
                    } else {
                        command
                    }
                })
                .collect();

            let input_file_path = path.join(Path::new(&commands.remove(0)));

            // parse input stream
            let in_stream = build_input_stream(
                &mut commands,
                util::lines_from_file(input_file_path.to_str().unwrap()),
            )
            .unwrap();

            // TODO:
            // This construction is rather clumsy and could probably be solved if main_worker
            // had better trait bounds on the writer object.
            
            let buffer = Vec::new();
            let mut writer = io::BufWriter::with_capacity(util::BUF_SIZE, buffer);
            // put input commands and input stream to the shadow main function
            // TODO: currently do not expect errors
            // In the future we may want to test that we error correctly
            main_worker(commands, in_stream, &mut writer).unwrap();

            // retrieve the original buffer
            // this also flushes everything into the buffer
            let buffer = writer.into_inner().unwrap();
            // construct a string based on the buffer
            // this may be unnecessary because the main worker has this data directly
            let output = String::from_utf8(buffer).unwrap();

            if !std::path::Path::new(&output_path).exists() {
                panic!(
                    "Could not perform test for {:?} because the associated \"output\" is missing",
                    output_path.file_stem().unwrap()
                );
            }
            let output_goal = fs::read_to_string(output_path).unwrap();

            assert_eq!(output, output_goal);
        }
    }
}
