rjp -- Rapid JSON-lines processor
=================================

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

### Usage

TODO :-)