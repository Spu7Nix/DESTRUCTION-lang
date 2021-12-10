# DESTRUCTION

## Practical info

...

## Usage and CLI

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
