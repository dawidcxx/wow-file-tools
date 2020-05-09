- ⚠ WIP

# About

This tool allows you to output wow file contents into your terminal. 


> `./wow-file-tools --help`

```
wow-file-tools 1.0
ArenaCraft

USAGE:
    wow-file-tools <SUBCOMMAND>

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

SUBCOMMANDS:
    help    Prints this message or the help of the given subcommand(s)
    view    
```

> `./wow-file-tools view --help`

```
wow-file-tools-view 

USAGE:
    wow-file-tools view [FLAGS] --file <file>

FLAGS:
    -c, --compact    Output JSON will no longer be pretty printed
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
    -f, --file <file>    

```

example command:

`./wow-file-tools view --compact -f ./mpq/TolVirArena_29_51.adt`

If you want to try out, check out the [releases](https://github.com/arenacraftwow/wow-file-tools/releases) tab and grab the latest one.

# Features

- ### View Command

The view command will dump the conversion of the binary file into JSON. Supported formats: 

 * DBC
    - [x] Map.dbc
    - [x] LoadingScreens.dbc
* [x] ADT
* [x] WDT
* [x] WMO

*Note:* some formats might be lacking fields


# Compiling

This is a [rust](https://www.rust-lang.org/) project. Should successfully compile with `rustc 1.43.0-nightly (75cf41afb 2020-03-04)`. To compile, run: 

`cargo build --release` 

The binary will be in `$PROJECT/target/release`. 
