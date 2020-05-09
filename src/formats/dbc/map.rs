use serde::{Deserialize, Serialize};
use crate::formats::dbc::{DbcFile};
use crate::common::R;

#[derive(Debug, Serialize, Deserialize)]
pub struct MapDbcRow {
    pub id: u32,
    pub internal_name: String,
    pub flags: u32,
    pub map_type: u32,
    pub is_bg: bool,
    pub name: String,
    pub area_table_ref_id: u32,
    pub map_description_alliance: String,
    pub map_description_horde: String,
    pub loading_screen_ref_id: u32,
    pub bg_map_icon_scale: f32,
    pub ghost_entrance_map_ref_id: u32,
    pub ghost_entrance_x: f32,
    pub ghost_entrance_y: f32,
    pub time_of_the_day_override: u32,
    pub expansion: u32,
    pub raid_offset: u32,
    pub max_players: u32,
}

impl MapDbcRow {
    pub fn process(row_builder: &mut Vec<MapDbcRow>, dbc_file: &DbcFile) -> R<()> {
        for row in *&dbc_file {
            let id = row.get_number_column(1)?;
            let internal_name = row.get_string_column(2)?;
            let flags = row.get_number_column(3)?;
            let map_type = row.get_number_column(4)?;
            let is_bg = row.get_bool_column(5)?;
            let name = row.get_string_column(6)?;
            let area_table_ref_id = row.get_number_column(23)?;
            let map_description_alliance = row.get_string_column(24)?;
            let map_description_horde = row.get_string_column(41)?;
            let loading_screen_ref_id = row.get_number_column(58)?;
            let bg_map_icon_scale = row.get_float_column(59)?;
            let ghost_entrance_map_ref_id = row.get_number_column(60)?;
            let ghost_entrance_x = row.get_float_column(61)?;
            let ghost_entrance_y = row.get_float_column(62)?;
            let time_of_the_day_override = row.get_number_column(63)?;
            let expansion = row.get_number_column(64)?;
            let raid_offset = row.get_number_column(65)?;
            let max_players = row.get_number_column(66)?;
            row_builder.push(MapDbcRow {
                id,
                internal_name,
                flags,
                map_type,
                is_bg,
                name,
                area_table_ref_id,
                map_description_alliance,
                map_description_horde,
                loading_screen_ref_id,
                bg_map_icon_scale,
                ghost_entrance_map_ref_id,
                ghost_entrance_x,
                ghost_entrance_y,
                time_of_the_day_override,
                expansion,
                raid_offset,
                max_players,
            })
        }
        Ok(())
    }
}