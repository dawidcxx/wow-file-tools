use crate::common::R;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SpellVisualEffectNameDbcRow {
    pub id: u32,
    pub name: String,
    pub file_name: String,
    pub area_effect_size: f32,
    pub scale: f32,
}

impl super::dbc::DbcRowMapper for SpellVisualEffectNameDbcRow {
    fn map_dbc_row(row: &super::DbcFileIteratorRow) -> R<Self> {
        let id = row.get_number_column(1)?;
        let name = row.get_string_column(2)?;
        let file_name = row.get_string_column(3)?;
        let area_effect_size = row.get_float_column(4)?;
        let scale = row.get_float_column(5)?;
        Ok(SpellVisualEffectNameDbcRow {
            id,
            name,
            file_name,
            area_effect_size,
            scale,
        })
    }
}
