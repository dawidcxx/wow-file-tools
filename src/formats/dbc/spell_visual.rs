use serde::{Deserialize, Serialize};
use crate::formats::dbc::{DbcFile};
use crate::common::R;

#[derive(Debug, Serialize, Deserialize, Copy, Clone)]
pub struct SpellVisual {
    pub id: u32,
    pub pre_cast_kit_id: u32,
    pub cast_kit_id: u32,
    pub impact_kit_id: u32,
    pub state_kit_id: u32,
    pub state_done_kit_id: u32,
    pub channel_kit_id: u32,
    pub has_middle: bool,
    pub missile_model_id: u32,
}

impl SpellVisual {
    pub fn process(row_builder: &mut Vec<SpellVisual>, dbc_file: &DbcFile) -> R<()> {
        for row in *&dbc_file {
            let id = row.get_number_column(1)?;
            let pre_cast_kit_id = row.get_number_column(2)?;
            let cast_kit_id = row.get_number_column(3)?;
            let impact_kit_id = row.get_number_column(4)?;
            let state_kit_id = row.get_number_column(5)?;
            let state_done_kit_id = row.get_number_column(6)?;
            let channel_kit_id = row.get_number_column(7)?;
            let has_middle = row.get_bool_column(8)?;
            let missile_model_id = row.get_number_column(9)?;
            row_builder.push(SpellVisual {
                id,
                pre_cast_kit_id,
                cast_kit_id,
                impact_kit_id,
                state_kit_id,
                state_done_kit_id,
                channel_kit_id,
                has_middle,
                missile_model_id
            })
        }
        Ok(())
    }
}
