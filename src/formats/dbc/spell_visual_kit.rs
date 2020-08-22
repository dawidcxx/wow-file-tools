use serde::{Deserialize, Serialize};
use crate::formats::dbc::{DbcFile};
use crate::common::R;

#[derive(Debug, Serialize, Deserialize, Copy, Clone)]
pub struct SpellVisualKitDbcRow {
    pub id: u32,
    pub start_animation_id: u32,
    pub animation_id: u32,
    pub head_effect: u32,
    pub chest_effect: u32,
    pub base_effect: u32,
    pub left_hand_effect: u32,
    pub right_hand_effect: u32,
    pub breath_effect: u32,
    pub left_weapon_effect: u32,
    pub right_weapon_effect: u32,
    pub special_effect_1: u32,
    pub special_effect_2: u32,
    pub special_effect_3: u32,
    pub world_effect: u32,
    pub sound_id: u32,
    pub shake_id: u32,
    pub flags: u32,
}

impl SpellVisualKitDbcRow {
    pub fn process(row_builder: &mut Vec<SpellVisualKitDbcRow>, dbc_file: &DbcFile) -> R<()> {
        for row in *&dbc_file {
            let id = row.get_number_column(1)?;
            let start_animation_id = row.get_number_column(2)?;
            let animation_id = row.get_number_column(3)?;
            let head_effect = row.get_number_column(4)?;
            let chest_effect = row.get_number_column(5)?;
            let base_effect = row.get_number_column(6)?;
            let left_hand_effect = row.get_number_column(7)?;
            let right_hand_effect = row.get_number_column(8)?;
            let breath_effect = row.get_number_column(9)?;
            let left_weapon_effect = row.get_number_column(10)?;
            let right_weapon_effect = row.get_number_column(11)?;
            let special_effect_1 = row.get_number_column(12)?;
            let special_effect_2 = row.get_number_column(13)?;
            let special_effect_3 = row.get_number_column(14)?;
            let world_effect = row.get_number_column(15)?;
            let sound_id = row.get_number_column(16)?;
            let shake_id = row.get_number_column(17)?;
            let flags = row.get_number_column(35)?;
            row_builder.push(SpellVisualKitDbcRow {
                id,
                start_animation_id,
                animation_id,
                head_effect,
                chest_effect,
                base_effect,
                left_hand_effect,
                right_hand_effect,
                breath_effect,
                left_weapon_effect,
                right_weapon_effect,
                special_effect_1,
                special_effect_2,
                special_effect_3,
                world_effect,
                sound_id,
                shake_id,
                flags,
            })
        }
        Ok(())
    }
}
