---
title: Basic Usage | ImageTools Docs
---

[Home](https://19wintersp.github.io/ImageTools/)

# Basic Usage

The application has some basic shared usage and behaviour, independent of the invidual tools.

## Usage

The basic usage help for the application is as follows:

```
USAGE:
    imgt [FLAGS] [OPTIONS]
    imgt <SUBCOMMAND>

FLAGS:
    -h, --help        Prints help information
    -L, --license     Show program license
    -l, --list        List available tools
    -v, --version     Show version information

OPTIONS:
    -o, --output <OUTPUT>       Specify output file
        --oformat <oformat>     Override output format
```

To get this, either run `imgt --help` or `imgt help`.

## Flags

### Help

Help can be shown with the `--help` or `-h` flags (or with a subcommand). Using the help flag will override any other flags if no subcommand is specified. Help is also shown when no arguments or subcommands are provided.

### License

The program's full license can be shown with the `--license` or `-L` flags. Using the license flag will override any lower flags if no subcommand is specified. An abridged version of the license text is shown on the help page.

### List

The `--list` and `-l` flags will display the list of subcommands, as only the subcommands, separated by newlines. Using the list flag will override any lower flags if no subcommand is specified.

### Version

The program name and version will be displayed with the `--version` or `-v` flags. Using the version flag will override any lower flags if no subcommand is specified.

## Common options

### Output

An output file name can be specified with `--output` or `-o`, followed by the file name. If specified, the output (if any) will be written to the specified file instead of to stdout. The file format is guessed from the file extension, unless overridden with *Output format*.

### Output format

The output format can be overriden with `--oformat`, followed by the image format - which should be in [image format form](types.html). When the output (if any) is written to stdout or a file specified by *Output*, it will be written with the specified format. When written to stdout, the format is PNG by default.
