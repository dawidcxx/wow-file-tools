# About

General purpose CLI utility. Mostly for manipulating client side files. This is not meant to be one tool but rather a collection of different tools sharing the same framework. Supports both linux & windows.

# Features

- ### Resolve Map assets

Output all of the map dependencies. The given `--map-ids` must be found in `Map.dbc`.

```
OPTIONS:
    -m, --map-ids <map-id>...      
    -w, --workspace <workspace>   
```

- ### View Command

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

- ### DbcJoin Command

Like the view command will output DBC info. However unlike the view command it will work across multiple files and join the results together to a more readable format. 


```
OPTIONS:
    -d, --dbc-folder <dbc-folder>    
    -j, --join-name <join>           join to display, one of: SPELLS, TALENTS
    -r, --record-id <record-id> 
```

# Compiling

This is a [rust](https://www.rust-lang.org/) project. Should successfully compile with `rustc 1.43.0-nightly (75cf41afb 2020-03-04)`. To compile, run: 

`cargo build --release` 

The binary will be in `$PROJECT/target/release`. 
