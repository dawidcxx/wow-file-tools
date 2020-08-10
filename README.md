# About

General purpose utility used @ [ArenaCraft](https://github.com/arenacraftwow). Mostly for manipulating client side files. This is not meant to be one tool but rather a collection of different tools sharing the same framework. 

# Features

- ### Resolve Map assets

The `resolve-map-assets` command will, for a given `map-id` output all the maps dependencies. The `map-id` referes to a entry in `Map.dbc`. 

```
FLAGS:
    -h, --help                          Prints help information
    -a, --make-result-paths-absolute    
    -p, --prune-unused                  Remove unneeded files within the workspace
    -V, --version                       Prints version information

OPTIONS:
    -m, --map-id <map-id>          
    -w, --workspace <workspace>    
```


- ### View Command

The `view` command will dump the conversion of the binary file into JSON. Supported formats: 

* DBC
    - [x] Map.dbc
    - [x] LoadingScreens.dbc
    - [x] AreaTable.dbc
    - [x] LightSkybox.dbc
    - [x] Light.dbc
    - [x] BattlemasterList.dbc
    - [x] GroundEffectTexture.dbc
    - [x] GroundEffectDoodad.dbc
* [x] ADT
* [x] WDT
* [x] WMO

*Note:* some formats might be lacking fields

# Compiling

This is a [rust](https://www.rust-lang.org/) project. Should successfully compile with `rustc 1.43.0-nightly (75cf41afb 2020-03-04)`. To compile, run: 

`cargo build --release` 

The binary will be in `$PROJECT/target/release`. 
