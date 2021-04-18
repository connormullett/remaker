
# Remake
A make clone for linux in Rust.

## Installation
Requires Rust

Because of the current implementation (using the `fork` syscall), this tool is only availabe for Linux and potentially Mac. Windows uses `CreateProcess` to spawn processes. I haven't found something to help make this cross platform as of yet.

- `git clone https://github.com/connormullett/remaker.git`
- `cd remaker`
- `cargo install --path .`

## Usage
Remake uses roughly the same syntax as make. The big difference being wildcards/variabls are expanded by name, rather than with `${value}` syntax. Rules are not tab sensitive and can use tabs or as many spaces as desired.

A remake file needs to be named `remaker`.

Remake uses the first rule found in the remake file as the default rule. To specify a rule to use (such as test), use `remake test`.

Currently, targets and dependencies do not expand with placeholders or wildcards. Only recipes check for these values.

Using `*` for file matching is not supported as of now. For example, using `*.c` to refer to all C source files will not expand as expected, and will instead look for a file named `*.c`.

To specify a remake file in a separate directory or by another name, use `--path ./path/to/remaker`. This option uses relative pathing.

An example remake file looks like the following:

```
CC=gcc
CFLAGS=-g
CLEAN_FILES=foo.o main
OBJS=foo.o math.o
BIN=main

# this is the default rule
BIN: OBJS
    # wildcards get replaced with their value
    CC CFLAGS $^ -o $@

# expand similar rules with placeholders
%.o: %.c
    CC CFLAGS -c $^ -o $@

# run tests
test:
    cargo $@

# clean up all the object and executable files
clean:
    rm OBJS BIN

```

Remake will create a `remake-lock.json`. This file should NOT be edited. This file is used when the remaker file hasn't been changed before runs. This way, remake doesn't need to parse the remake file again. It just needs to deserialize the JSON to the appropriate struct and execute. When the remake file is updated, the remake-lock.json file will also be updated.

Then, you can use remake from the command line by invoking `remake`.

## Currently Not Supported
- conditionals

## Want to help or see that I'm missing something above?
Send it my way in issues, or open a PR. Everyone is welcome to contribute.
