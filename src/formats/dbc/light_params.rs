use crate::common::R;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct LightParamsDbcRow {
    pub id: u32,
    pub highlight_sky: bool,
    pub light_sky_box_id: u32,
    pub cloud_type_id: u32,
    pub glow: f32,
    pub water_shallow_alpha: f32,
    pub water_deep_alpha: f32,
    pub ocean_shallow_alpha: f32,
    pub ocean_deep_alpha: f32,
}

impl super::dbc::DbcRowMapper for LightParamsDbcRow {
    fn map_dbc_row(row: &super::DbcFileIteratorRow) -> R<Self> {
        let id = row.get_number_column(1)?;
        let highlight_sky = row.get_bool_column(2)?;
        let light_sky_box_id = row.get_number_column(3)?;
        let cloud_type_id = row.get_number_column(4)?;
        let glow = row.get_float_column(5)?;
        let water_shallow_alpha = row.get_float_column(6)?;
        let water_deep_alpha = row.get_float_column(7)?;
        let ocean_shallow_alpha = row.get_float_column(8)?;
        let ocean_deep_alpha = row.get_float_column(9)?;
        Ok(LightParamsDbcRow {
            id,
            highlight_sky,
            light_sky_box_id,
            cloud_type_id,
            glow,
            water_shallow_alpha,
            water_deep_alpha,
            ocean_shallow_alpha,
            ocean_deep_alpha,
        })
    }
}
