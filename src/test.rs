#[cfg(test)]
mod tests {
    use crate::builders::*;
    use crate::pipeline::main_worker::*;
    use crate::util;
    use std::path::Path;
    use std::{fs, io};

    fn test_valid_rjp_output(
        input_file: &str,
        command: &str,
        output_file: &str,
    ) -> (String, String) {
        // Runs a test given an input file a command and a target output file against which to compare the result

        let path = Path::new(env!("CARGO_MANIFEST_DIR")).join(Path::new("tests"));

        let mut command = command
            .split_ascii_whitespace()
            .map(|s| s.to_owned())
            .collect::<Vec<String>>();

        // Prefix everything that ends with ".json" or ".tsv" with the current path
        // TODO: this is a hacky solution and needs a more robust approach
        // TODO: will break when more files are added
        command = command
            .into_iter()
            .map(|command| {
                if command.ends_with(".json") || command.ends_with(".tsv") {
                    String::from(path.join(command).to_str().unwrap())
                } else {
                    command
                }
            })
            .collect();

        // parse input stream
        let input_file_path = path.join(Path::new(input_file));
        let in_stream = build_input_stream(
            &mut command,
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
        main_worker(command, in_stream, &mut writer).unwrap();

        // retrieve the original buffer
        // this also flushes everything into the buffer
        let buffer = writer.into_inner().unwrap();
        // construct a string based on the buffer
        // this may be unnecessary because the main worker has this data directly
        let output = String::from_utf8(buffer).unwrap();

        let output_file = path.join(Path::new(output_file));
        if !output_file.exists() {
            panic!(
                "Could not perform test because the output file \"{}\" is missing",
                output_file.file_stem().unwrap().to_str().unwrap()
            );
        }
        let output_goal = fs::read_to_string(output_file).unwrap();

        let output_lines = output.split('\n');
        let output_goal_lines = output_goal.split('\n');
        // assert individual lines
        assert!(output_lines.eq(output_goal_lines));

        return (output, output_goal);
    }

    #[test]
    fn simple_join() {
        test_valid_rjp_output("a.json", "join b.json a,b", "a_join_b_output.json");
    }

    #[test]
    fn simple_index() {
        test_valid_rjp_output(
            "c.json",
            "e a[0]:first_item,c[foo]:foo_item",
            "c_index_output.json",
        );
    }
}
