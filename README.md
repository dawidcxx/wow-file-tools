# About

General purpose CLI utility. Mostly for manipulating client side files. This is not meant to be one tool but rather a collection of different tools sharing the same framework. Supports both linux & windows.

# Features


- ## MPQ tool

A sub-tool to work with MPQs. 

Examples:
 - Get the (listfile) `wow-file-tools mpq view -a  ./Work/patch-A.MPQ`
 - Extract a file from a MPQ as hex `wow-file-tools mpq extract -a  ./Work/patch-A.MPQ -f "World\wmo\kalimdor\uldum\uldum_fishing_village.wmo"`
 - Extract a file from a MPQ to disk `wow-file-tools mpq extract -a  ./Work/patch-A.MPQ -f "World\wmo\kalimdor\uldum\uldum_fishing_village.wmo" -t ./`
- Extract everything from a MPQ to ./Work `wow-file-tools mpq extract-tree -a .\Work\patch-A.mpq -t "/" -d .\Work\`

- ## Resolve Map assets

Output all of the map dependencies. The given `--map-ids` must be found in `Map.dbc`.

```
OPTIONS:
    -m, --map-ids <map-id>...      
    -w, --workspace <workspace>   
```

- ## View Command

Dump the conversion of the binary file into JSON. Supported formats: 

* DBC
    - [x] Map.dbc
    - [x] LoadingScreens.dbc
    - [x] AreaTable.dbc
    - [x] LightSkybox.dbc
    - [x] LightParams.dbc
    - [x] Light.dbc
    - [x] BattlemasterList.dbc
    - [x] GroundEffectTexture.dbc
    - [x] GroundEffectDoodad.dbc
    - [x] GameObjectDisplayInfo.dbc
    - [x] PvpDifficulty.dbc
    - [x] Spell.dbc
    - [x] SpellIcon.dbc
    - [x] SpellVisual.dbc
    - [x] SpellVisualKit.dbc
    - [x] SpellVisualEffectName.dbc
    - [x] Talent.dbc
    - [x] TalentTab.dbc
* [x] ADT
* [x] WDT
* [x] WMO

*Note:* some formats might be lacking fields

```
OPTIONS:
    -f, --file <file>    
```

- ## DbcJoin Command

Like the view command will output DBC info. However unlike the view command it will work across multiple files and join the results together to a more readable format. 


```
OPTIONS:
    -d, --dbc-folder <dbc-folder>    
    -j, --join-name <join>           join to display, one of: SPELLS, TALENTS
    -r, --record-id <record-id> 
```

# Compiling

In order to compile this project you will need 

- [rust](https://www.rust-lang.org/) language toolchain, successfully compiled with `rustc 1.43.0-nightly (75cf41afb 2020-03-04)` but other version should work fine as well
- A C/C++ compiler
- [CMake](https://cmake.org/)
- (On linux)`bzip2` / `zlib`. Depending on the distro you most likely have them installed already.


To compile, run: 

`cargo build --release` 

The binary will be in `$PROJECT/target/release`. 
