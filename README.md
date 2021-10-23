rjp: Rapid JSON-lines processor
===============================

A fast and simple command-line tool for common operations over JSON-lines files, such as:

* converting to and from text files, TSV files
* joining files on (multiple) keys
* merging files line by line
* adding, removing, selecting fields
* ...

You could use `jq` for some of these tasks (and in fact, `jq` is a far more general tool) but:

* `rjp` is designed for the JSON-lines format specifically
* `rjp` can be faster
* some common tasks are more easily done in `rjp`

This is my attempt to learn a bit of Rust, don't take this tool too seriously. That being said,
it is pretty quick and handy, at least for me.

## Build & Installation

Get `rust`:

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
```

Clone and build `rjp`:

```bash
git clone https://github.com/ales-t/rjp.git
cd rjp
cargo build --release
```

You fill find the binary in `target/release/rjp`. You can add it to your `PATH` e.g. like this:

```bash
export PATH="$(pwd)/target/release:$PATH"
```

## Basic usage

```bash
rjp < input_file [INPUT_CONVERSION] [PROCESSOR [PROCESSOR...]] [OUTPUT_CONVERSION] > output_file
```

`rjp` runs a chain of processors on each instance in the input stream (`STDIN`), finally printing
the processed instances to `STDOUT`.

## Input conversions

By default, `rjp` reads the input file as JSON lines. You can optionally specify a file conversion
as the first positional argument.

### TSV

Convert TSV lines with specified field names.

Aliases: `tsv_to_json`, `from_tsv`

Examples:

* `rjp < in.tsv from_tsv first_field_name,second_field_name,... [PROCESSORS] [OUTPUT_CONVERSION] > output_file`

### Plain text

Conversion from TXT treats the whole input line as a single string field, you need to specify its
name.

Aliases: `txt_to_json`, `from_txt`

Examples:

* `rjp < in.txt from_txt field_name [PROCESSORS] [OUTPUT_CONVERSION] > output_file`

## Processors

The following processors are implemented (brackets list shorthand aliases):

### Add fields

Add new fields with constant values.

Aliases: `add_fields`, `af`, `add`

Examples:

* `rjp < in.json add_fields new_field_name:value1,another_field:value2 > out.json`

### Drop fields

Remove existing fields.

Aliases: `drop_fields`, `df`, `drop`

Examples:

* `rjp < in.json to_drop,another_to_drop > out.json`

### Extract items

Extract items from arrays and objects.

Aliases: `extract_items` `e`, `extract`

Examples:

* `rjp < in.json array_field[0]:new_field,object_field[key]:another_field > out.json`

### Join

Perform inner join with another input stream (with optional file conversion).

Note on performance: while the main stream is processed line-by-line, the stream to join is loaded
in RAM (i.e. use the smaller file as the joined stream).

Aliases: `join`, `j`, `inner_join`

Examples:

* `rjp < in.json join file.json key_field_1,key_field_2 > out.json`
* With file conversion: `rjp < in.json join file.tsv key from_tsv key,tsv_value > out.json`

### Left join

Identical to `join`, except that lines from the main stream that don't have a corresponding instance
in the joined stream are kept (and no additional fields are added to them).

Aliases: `lj`, `left_join`

### Merge

Merge wih another input stream line-by-line, with optional file conversion.

Aliases: `merge`, `mrg`

Examples:

* `rjp < in.json merge file_to_merge.json > out.json`
* With file conversion: `rjp < in.json merge to_merge.tsv from_tsv col_a,col_b > out.json`

### Rename fields

Rename fields in instances.

Aliases: `rename`, `rnm`

Examples:

* `rjp < in.json old_name:new_name,another_old:another_new > out.json`

### Select fields

Select a subset of fields (the rest are dropped).

Aliases: `select_fields`, `sf`, `select`, `sel`

Examples:

* `rjp < in.json select_fields first,second > out.json`

### To number

Convert a string field to a numeric one.

Aliases: `to_number`, `num`

Examples:

* `rjp < in.json to_number string_field_name:new_numeric_field_name,another_string:another_numeric > out.json`

## Output conversions

By default, `rjp` will produce JSON-lines. You can change that with a file conversion.

### TSV

Convert into TSV liens with specified fields.

Aliases: `to_tsv`, `json_to_tsv`, `tsv`

Examples:

* `rjp < in.json to_tsv field_1,field_2 > out.tsv`