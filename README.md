# DESTRUCTION

Documentation:

- [Language](documentation.md)
- [CLI](#usage-and-cli)

## Practical info

...

## Usage and CLI

### Installation

Notes: You must have rust installed from the official [rust-lang](https://www.rust-lang.org/tools/install) site; There are installation steps on the site.

- Clone the github repository:  

```sh
$ git clone https://github.com/Spu7Nix/langjam2.git
Cloned https://github.com/Spu7Nix/langjam2.git into langjam2
```

- `cd` into `./langjam2` and run the following:

```sh
cargo run -- [command goes here] # Commands are documented below
```

Happy DESTRUCTION-ing!

### CLI docs

```sh
# Subcommands

## Build and run a source file
build

## Evaluate arbitrary code from the command line
eval

## Prints out help :)
help

# Flags, Options, and Args

## DESTRUCTION-build

### Args:
<path:string> # Path to the source file to be built

### Options:
-i | --input <input:string> # String for the interpreter to use as input

### Usage:
DESTRUCTION build <path> --input <input>

#-------------------------------------------------------------#

## DESTRUCTION-eval

### Args:
<code:string> # Code to evaluate

### Options:
-i | --input <input:string> # String for the interpreter to use as input

### Usage:
DESTRUCTION eval <code> --input <input>

## DESTRUCTION-help

### Args:
<subcommand:string> # The subcommand whose help message to display

### Usage:
DESTRUCTION help <subcommand>
```
