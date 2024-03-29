# orgtabler

Transform a csv file into an org-mode formatted table.

[![Rust](https://github.com/juan-leon/orgtabler/actions/workflows/test.yml/badge.svg)](https://github.com/juan-leon/orgtabler/actions/workflows/test.yml)
[![dependency status](https://deps.rs/repo/github/juan-leon/orgtabler/status.svg)](https://deps.rs/repo/github/juan-leon/orgtabler)
[![Release](https://img.shields.io/github/release/juan-leon/orgtabler.svg)](https://github.com/juan-leon/orgtabler/releases/latest)


Emacs already has a function that does that, but I am not happy with its
performance for big input files (and, because of that, it is capped to 999
lines).

This tool also has the option of colorizing the output (useless for feeding
tables to emacs, but convenient to displaying them on a terminal).

## Usage

```
USAGE:
    orgtabler [FLAGS] <filename>

ARGS:
    <filename>    Sets the input file to use

FLAGS:
    -c, --color        Colorize output
    -h, --help         Prints help information
    -n, --no-header    Assume CSV has no header
    -V, --version      Prints version information
```

## Installation

### Via release

Go over https://github.com/juan-leon/orgtabler/releases/ and download the binary
you want.  Decompress the file and copy the binary to your path.

#### Via local compilation

```
$ git clone https://github.com/juan-leon/orgtabler
$ cd orgtabler
$ cargo install --path .
```

## Contributing

Feedback, ideas and pull requests are welcomed.
