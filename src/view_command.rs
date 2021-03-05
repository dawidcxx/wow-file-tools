use crate::formats::adt::AdtFile;
use crate::formats::dbc::dbc::*;
use crate::formats::m2::M2File;
use crate::formats::wdt::WdtFile;
use crate::formats::wmo::WmoFile;
use crate::{common::R, ViewCmd};
use std::{ops::Deref, path::PathBuf};

pub fn handle_view_command(view_cmd: &ViewCmd) -> R<Box<dyn erased_serde::Serialize>> {
    let file_path = PathBuf::from(view_cmd.file.as_str());
    let extension = file_path
        .extension()
        .map(|e| e.to_string_lossy().to_lowercase())
        .ok_or("Given file doesn't have a valid extension")?;
    let file_name = file_path
        .file_name()
        .map(|e| e.to_string_lossy())
        .ok_or("Given file is missing a filename")?;

    let result: Box<dyn erased_serde::Serialize> = match extension.deref() {
        "dbc" => match file_name.deref() {
            "Spell.dbc" => Box::new(load_spell_dbc_from_path(file_path)?),
            "SpellVisualKit.dbc" => Box::new(load_spell_visual_kit_dbc_from_path(file_path)?),
            "SpellVisualEffectName.dbc" => {
                Box::new(load_spell_visual_effect_name_dbc_from_path(file_path)?)
            }
            "SpellVisual.dbc" => Box::new(load_spell_visual_dbc_from_path(file_path)?),
            "SpellIcon.dbc" => Box::new(load_spell_icon_dbc_from_path(file_path)?),
            "GroundEffectDoodad.dbc" => Box::new(load_ground_effect_doodad_from_path(file_path)?),
            "GroundEffectTexture.dbc" => Box::new(load_ground_effect_texture_from_path(file_path)?),
            "BattlemasterList.dbc" => Box::new(load_battle_master_list_from_path(file_path)?),
            "LightSkybox.dbc" => Box::new(load_light_sky_box_from_path(file_path)?),
            "Light.dbc" => Box::new(load_light_from_path(file_path)?),
            "LightParams.dbc" => Box::new(load_light_params_from_path(file_path)?),
            "AreaTable.dbc" => Box::new(load_area_table_from_path(file_path)?),
            "Map.dbc" => Box::new(load_map_dbc_from_path(file_path)?),
            "LoadingScreens.dbc" => Box::new(load_loading_screens_dbc_from_path(file_path)?),
            "PvpDifficulty.dbc" => Box::new(load_pvp_difficulty_from_path(file_path)?),
            "GameObjectDisplayInfo.dbc" => {
                Box::new(load_game_object_display_info_from_path(file_path)?)
            }
            "Talent.dbc" => Box::new(load_talent_dbc_from_path(file_path)?),
            "TalentTab.dbc" => Box::new(load_talent_tab_dbc_from_path(file_path)?),
            _ => {
                let err_msg = format!("Unsupported DBC file: `{}`", file_name);
                return Err(err_msg.into());
            }
        },
        "wdt" => Box::new(WdtFile::from_path(file_path)?),
        "wmo" => Box::new(WmoFile::from_path(file_path)?),
        "adt" => Box::new(AdtFile::from_path(file_path)?),
        "m2" => Box::new(M2File::from_path(file_path)?),
        _ => {
            let err_msg = format!("Unsupported file extension: `{}`", extension);
            return Err(err_msg.into());
        }
    };

    return Ok(result);
}
