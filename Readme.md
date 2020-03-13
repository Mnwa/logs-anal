# Simple and fast logs analysis utils
[![](https://docs.rs/logs-anal/badge.svg)](https://docs.rs/logs-anal/)
![Rust](https://github.com/Mnwa/logs-anal/workflows/Build/badge.svg?branch=master)
[![](https://img.shields.io/crates/v/logs-anal.svg)](https://crates.io/crates/logs-anal)
[![](https://img.shields.io/crates/d/logs-anal.svg)](https://crates.io/crates/logs-anal)

## Installing with cargo
```bash
cargo install logs-anal
```

## Usage

```bash
logs-anal -h

#Logs Analysis Tool 0.1.0
#Mnwa
#Analyze logs with comfort
#
#USAGE:
#    logs-anal [OPTIONS] --command <Enum> [INPUT]
#
#FLAGS:
#    -h, --help       Prints help information
#    -V, --version    Prints version information
#
#OPTIONS:
#    -k, --column <Number>       Column for sorting
#    -c, --command <Enum>        Runs that command  [possible values: sort, head, skip, tail, uniq, wc]
#    -d, --delimiter <String>    Delimiter for sorting (default is whitespace)
#    -n, --lines <Number>        Lines for skip or take rows
#    -o, --order <ORDER_TYPE>    Sorting order [possible values: asc, desc]
#
#ARGS:
#    <INPUT>    Sets the input file to use
```

## Usage example
```bash
echo "Joe   56789
Sam   45678
Wendy 23456
Adam  12345
Bob   34567" | logs-anal -c sort -k 1

# Adam  12345
# Bob   34567
# Joe   56789
# Sam   45678
# Wendy 23456
```

### Supported tools
* sort
* head
* skip
* tail
* uniq
* wc