use crate::formats::dbc::{DbcHeader, DbcFile};
use serde::{Serialize, Deserialize};
use crate::formats::dbc::map::MapDbcRow;
use crate::common::R;
use crate::formats::dbc::loading_screens::LoadingScreenDbcRow;
use crate::formats::dbc::area_table::AreaTableDbcRow;
use crate::formats::dbc::light_sky_box::LightSkyBoxDbcRow;
use crate::formats::dbc::battle_master_list::BattleMasterListDbcRow;
use crate::formats::dbc::ground_effect_texture::GroundEffectTextureDbcRow;
use crate::formats::dbc::ground_effect_doodad::GroundEffectDoodadDbcRow;
use crate::formats::dbc::light::LightDbcRow;
use crate::formats::dbc::light_params::LightParamsDbcRow;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Dbc<T> {
    pub header: DbcHeader,
    pub rows: Vec<T>,
}

type DbcRowProcessor<T> = dyn Fn(&mut Vec<T>, &DbcFile) -> R<()>;

fn load_dbc<T>(path: &str, row_mapper: Box<DbcRowProcessor<T>>) -> R<Dbc<T>>
{
    let dbc = DbcFile::new(path)?;
    let mut row_builder = Vec::with_capacity(dbc.header.field_count as usize);
    row_mapper.call((&mut row_builder, &dbc))?;
    Ok(Dbc {
        header: dbc.header,
        rows: row_builder,
    })
}

pub fn load_map_dbc_from_path(path: &str) -> R<Dbc<MapDbcRow>> {
    load_dbc(path, Box::new(MapDbcRow::process))
}

pub fn load_loading_screens_dbc_from_path(path: &str) -> R<Dbc<LoadingScreenDbcRow>> {
    load_dbc(path, Box::new(LoadingScreenDbcRow::process))
}

pub fn load_area_table_from_path(path: &str) -> R<Dbc<AreaTableDbcRow>> {
    load_dbc(path, Box::new(AreaTableDbcRow::process))
}

pub fn load_light_sky_box_from_path(path: &str) -> R<Dbc<LightSkyBoxDbcRow>> {
    load_dbc(path, Box::new(LightSkyBoxDbcRow::process))
}

pub fn load_light_from_path(path: &str) -> R<Dbc<LightDbcRow>> {
    load_dbc(path, Box::new(LightDbcRow::process))
}

pub fn load_light_params_from_path(path: &str) -> R<Dbc<LightParamsDbcRow>> {
    load_dbc(path, Box::new(LightParamsDbcRow::process))
}

pub fn load_battle_master_list_from_path(path: &str) -> R<Dbc<BattleMasterListDbcRow>> {
    load_dbc(path, Box::new(BattleMasterListDbcRow::process))
}

pub fn load_ground_effect_texture_from_path(path: &str) -> R<Dbc<GroundEffectTextureDbcRow>> {
    load_dbc(path, Box::new(GroundEffectTextureDbcRow::process))
}

pub fn load_ground_effect_doodad_from_path(path: &str) -> R<Dbc<GroundEffectDoodadDbcRow>> {
    load_dbc(path, Box::new(GroundEffectDoodadDbcRow::process))
}
