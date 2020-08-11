use serde::{Deserialize, Serialize};
use crate::formats::dbc::{DbcFile};
use crate::common::R;

#[derive(Debug, Serialize, Deserialize)]
pub struct LightDbcRow {
   pub id: u32,
   pub ref_map_id: u32,
   pub position: [f32; 3],
   pub falloff_start: f32,
   pub falloff_end: f32,
   pub params_clear: u32,
   pub params_clear_water: u32,
   pub params_storm: u32,
   pub params_storm_water: u32,
   pub params_death: u32,
}

impl LightDbcRow {
    pub fn process(row_builder: &mut Vec<LightDbcRow>, dbc_file: &DbcFile) -> R<()> {
        for row in *&dbc_file {
            let id = row.get_number_column(1)?;
            let ref_map_id = row.get_number_column(2)?;
            let pos1 = row.get_float_column(3)?;
            let pos2 = row.get_float_column(4)?;
            let pos3 = row.get_float_column(5)?;
            let falloff_start = row.get_float_column(6)?;
            let falloff_end = row.get_float_column(7)?;
            let params_clear = row.get_number_column(8)?;
            let params_clear_water = row.get_number_column(9)?;
            let params_storm = row.get_number_column(10)?;
            let params_storm_water = row.get_number_column(11)?;
            let params_death = row.get_number_column(12)?;
            row_builder.push(LightDbcRow {
                id,
                ref_map_id,
                position: [pos1, pos2, pos3],
                falloff_start,
                falloff_end,
                params_clear,
                params_clear_water,
                params_storm,
                params_storm_water,
                params_death
            })
        }
        Ok(())
    }
}
