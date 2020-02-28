#Simple and fast logs analytics utils
## Usage

Help
```bash
logs-anal -h                                                                              ✔  19:15:15  

#Logs Analytics Tool 0.1.0
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
echo "Joe   56789                                                                                                    ✔  19:20:23  
Sam   45678
Wendy 23456
Adam  12345
Bob   34567" | target/release/logs-anal -c sort -k 1 zipcode

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