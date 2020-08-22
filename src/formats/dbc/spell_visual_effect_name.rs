use serde::{Deserialize, Serialize};
use crate::formats::dbc::{DbcFile};
use crate::common::R;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SpellVisualEffectNameDbcRow {
    pub id: u32,
    pub name: String,
    pub file_name: String,
    pub area_effect_size: f32,
    pub scale: f32,
}

impl SpellVisualEffectNameDbcRow {
    pub fn process(row_builder: &mut Vec<SpellVisualEffectNameDbcRow>, dbc_file: &DbcFile) -> R<()> {
        for row in *&dbc_file {
            let id = row.get_number_column(1)?;
            let name = row.get_string_column(2)?;
            let file_name = row.get_string_column(3)?;
            let area_effect_size = row.get_float_column(4)?;
            let scale = row.get_float_column(5)?;
            row_builder.push(SpellVisualEffectNameDbcRow {
                id,
                name,
                file_name,
                area_effect_size,
                scale,
            })
        }
        Ok(())
    }
}
