#![feature(backtrace)]
#![feature(fn_traits)]

pub mod byte_utils;
pub mod formats;
pub mod common;
mod resolve_map_assets;

use clap::Clap;
use crate::common::{R};
use std::path::Path;
use serde::{Serialize, Serializer};
use std::error::Error;
use serde::ser::SerializeStruct;
use crate::formats::adt::AdtFile;
use crate::formats::wmo::{WmoFile};
use crate::formats::dbc::dbc::*;
use crate::formats::wdt::WdtFile;
use crate::formats::m2::M2File;
use crate::resolve_map_assets::ResolveMapAssetsCmdResult;
use crate::formats::dbc::join::spell::get_spells_join;
use crate::formats::dbc::join::talents::get_talents_join;

fn main() {
    let root_cmd = RootCmd::parse();
    let cmd_result = handle_cmd(root_cmd)
        .map_err(|error| ProgramErr { error });

    std::process::exit(match cmd_result {
        Err(e) => {
            let json = serde_json::to_string_pretty(&e).unwrap();
            eprintln!("{}", json);
            727
        }
        _ => 0
    })
}

fn handle_cmd(root_cmd: RootCmd) -> R<()> {
    let result = match &root_cmd.cmd {
        Cmd::View(v) => {
            let file_path = Path::new(&v.file);
            let file_path_str = file_path.to_str().unwrap();

            if !file_path.exists() {
                return Err("This file does not exist".into());
            }

            let extension = match file_path.extension() {
                None => {
                    return Err("Given file has no extension".into());
                }
                Some(v) => {
                    v.to_str().unwrap()
                }
            };

            get_view_result(&root_cmd, file_path_str, extension)?
        }
        Cmd::ResolveMapAssets(cmd) => {
            serialize_result(&root_cmd, handle_resolve_map_assets_cmd(cmd))?
        }
        Cmd::DbcJoin(cmd) => {
            match cmd.join {
                AggregateViewCmdChoice::SPELLS => {
                    serialize_result(&root_cmd, get_spells_join(&cmd.dbc_folder, &cmd.record_id))?
                }
                AggregateViewCmdChoice::TALENTS => {
                    serialize_result(&root_cmd, get_talents_join(&cmd.dbc_folder, &cmd.record_id))?
                }
            }
        }
    };

    println!("{}", result);

    Ok(())
}

fn handle_resolve_map_assets_cmd(
    cmd: &ResolveMapAssetsCmd
) -> R<ResolveMapAssetsCmdResult> {
    resolve_map_assets::resolve_map_assets(
        Path::new(cmd.workspace.as_str()),
        &cmd.map_id,
        cmd.prune_unused,
    )
}

fn get_view_result(
    root_cmd: &RootCmd,
    file_path_str: &str,
    extension: &str,
) -> R<String> {
    let result = match extension {
        "dbc" => {
            let file_name = extract_file_name(file_path_str);
            match file_name {
                "Spell.dbc" => serialize_result(root_cmd, load_spell_dbc_from_path(file_path_str))?,
                "SpellVisualKit.dbc" => serialize_result(root_cmd, load_spell_visual_kit_dbc_from_path(file_path_str))?,
                "SpellVisualEffectName.dbc" => serialize_result(root_cmd, load_spell_visual_effect_name_dbc_from_path(file_path_str))?,
                "SpellVisual.dbc" => serialize_result(root_cmd, load_spell_visual_dbc_from_path(file_path_str))?,
                "SpellIcon.dbc" => serialize_result(root_cmd, load_spell_icon_dbc_from_path(file_path_str))?,
                "GroundEffectDoodad.dbc" => serialize_result(root_cmd, load_ground_effect_doodad_from_path(file_path_str))?,
                "GroundEffectTexture.dbc" => serialize_result(root_cmd, load_ground_effect_texture_from_path(file_path_str))?,
                "BattlemasterList.dbc" => serialize_result(root_cmd, load_battle_master_list_from_path(file_path_str))?,
                "LightSkybox.dbc" => serialize_result(root_cmd, load_light_sky_box_from_path(file_path_str))?,
                "Light.dbc" => serialize_result(root_cmd, load_light_from_path(file_path_str))?,
                "LightParams.dbc" => serialize_result(root_cmd, load_light_params_from_path(file_path_str))?,
                "AreaTable.dbc" => serialize_result(root_cmd, load_area_table_from_path(file_path_str))?,
                "Map.dbc" => serialize_result(root_cmd, load_map_dbc_from_path(file_path_str))?,
                "LoadingScreens.dbc" => serialize_result(root_cmd, load_loading_screens_dbc_from_path(file_path_str))?,
                "PvpDifficulty.dbc" => serialize_result(root_cmd, load_pvp_difficulty_from_path(file_path_str))?,
                "GameObjectDisplayInfo.dbc" => serialize_result(root_cmd, load_game_object_display_info_from_path(file_path_str))?,
                "Talent.dbc" => serialize_result(root_cmd, load_talent_dbc_from_path(file_path_str))?,
                "TalentTab.dbc" => serialize_result(root_cmd, load_talent_tab_dbc_from_path(file_path_str))?,
                _ => {
                    let err_msg = format!("Unsupported DBC file: ({})", file_name);
                    return Err(err_msg.into());
                }
            }
        }
        "wdt" => serialize_result(root_cmd, WdtFile::from_path(file_path_str))?,
        "wmo" => serialize_result(root_cmd, WmoFile::from_path(file_path_str))?,
        "adt" => serialize_result(root_cmd, AdtFile::from_path(file_path_str))?,
        "m2" => serialize_result(root_cmd, M2File::from_path(file_path_str))?,
        _ => {
            let err_msg = format!("Unsupported file extension: ({})", extension);
            return Err(err_msg.into());
        }
    };

    Ok(result)
}

fn extract_file_name(file_path_str: &str) -> &str {
    Path::new(file_path_str)
        .file_name()
        .unwrap()
        .to_str()
        .unwrap()
}

fn serialize_result(cmd: &RootCmd, result: R<impl Serialize>) -> R<String> {
    if cmd.no_result {
        return Ok("".to_string());
    }
    let output_str = if cmd.compact {
        serde_json::to_string(&result?)?
    } else {
        serde_json::to_string_pretty(&result?)?
    };
    Ok(output_str)
}

#[derive(Clap)]
#[clap(version = "1.0", author = "ArenaCraft")]
struct RootCmd {
    #[clap(short = "c", long = "compact", help = "Output JSON will no longer be pretty printed")]
    compact: bool,

    #[clap(long = "no-result", help = "Don't output any result, useful for testing")]
    no_result: bool,

    #[clap(subcommand)]
    cmd: Cmd,
}

#[derive(Clap)]
enum Cmd {
    View(ViewCmd),
    ResolveMapAssets(ResolveMapAssetsCmd),
    DbcJoin(DbcJoinCmd),
}

#[derive(Clap)]
#[clap(about = "View given file as JSON")]
struct ViewCmd {
    #[clap(short = "f", long = "file")]
    file: String,
}

#[derive(Clap)]
#[clap(about = "Resolve all map dependencies")]
pub struct ResolveMapAssetsCmd {
    #[clap(short = "w", long = "workspace")]
    workspace: String,

    #[clap(short = "m", long = "map-ids")]
    map_id: Vec<u32>,

    #[clap(short = "p", long = "prune-unused", help = "Remove unneeded files within the workspace")]
    prune_unused: bool,
}

#[derive(Clap)]
#[clap(about = "Show a joined view of multiple dbc files")]
struct DbcJoinCmd {
    #[clap(short = "d", long = "dbc-folder")]
    dbc_folder: String,

    #[clap(short = "j", long = "join-name", help = "join to display, one of: SPELLS")]
    join: AggregateViewCmdChoice,

    #[clap(short = "r", long = "record-id")]
    record_id: Option<u32>,
}

enum AggregateViewCmdChoice {
    SPELLS,
    TALENTS,
}

impl std::str::FromStr for AggregateViewCmdChoice {
    type Err = Box<dyn Error>;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_uppercase().as_str() {
            "SPELLS" => Ok(Self::SPELLS),
            "TALENTS" => Ok(Self::TALENTS),
            _ => {
                Err("Must be one of ( SPELLS, TALENTS )\n".into())
            }
        }
    }
}

struct ProgramErr {
    error: Box<dyn Error>,
}

impl Serialize for ProgramErr {
    fn serialize<S>(&self, serializer: S) -> Result<<S as Serializer>::Ok, <S as Serializer>::Error> where
        S: Serializer {
        // 3 is the number of fields in the struct.
        let mut state = serializer.serialize_struct("ProgramErr", 3)?;
        state.serialize_field("error", &self.error.to_string())?;
        state.serialize_field(
            "backtrace",
            &self.error
                .backtrace()
                .map(|b| format!("{:?}", b)),
        )?;
        state.end()
    }
}