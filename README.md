
# Remake
A make clone for linux in Rust.

## Usage
Remake uses roughly the same syntax as make. The big difference being wildcards/variabls are expanded by name, rather than with `${value}` syntax. Rules are not tab sensitive and can use tabs or as many spaces as desired.

A remake file needs to be named `remaker`.

Remake uses the first rule found in the remake file as the default rule. To specify a rule to use (such as test), use `remake test`.

An example remake file looks like the following:

```
CC=gcc

main: foo.o
    CC foo.o -o main
    echo "it worked"

foo.o: foo.c
    CC -c foo.c -o foo.o
```

## Installation
- `git clone https://github.com/connormullett/remaker.git`
- `cd remaker`
- `cargo install --path .`

Then, you can use remake from the command line by invoking `remake`.

## Currently Not Supported
- Phony rules
- conditionals
- appending values to variables
- wildcard and placeholders (i.e. ^$, $@, and % )
