use crate::common::R;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Copy, Clone)]
pub struct SpellVisualDbcRow {
    pub id: u32,
    pub pre_cast_kit_id: u32,
    pub cast_kit_id: u32,
    pub impact_kit_id: u32,
    pub state_kit_id: u32,
    pub state_done_kit_id: u32,
    pub channel_kit_id: u32,
    pub has_middle: bool,
    pub missile_model_id: u32,
    pub missile_path_type: u32,
    pub missile_destination_attachment: u32,
    pub missile_sound: u32,
    pub anim_event_sound_id: u32,
    pub flags: u32,
    pub caster_impact_kit: u32,
    pub target_impact_kit: u32,
    pub missile_attachment: u32,
    pub missile_follow_ground_height: u32,
    pub missile_follow_ground_drop_speed: u32,
    pub missile_follow_ground_approach: u32,
    pub missile_follow_ground_flags: u32,
    pub missile_motion: u32,
    pub missile_targeting_kit: u32,
    pub instant_area_kit: u32,
    pub impact_area_kit: u32,
    pub persistent_area_kit: u32,
}

impl super::dbc::DbcRowMapper for SpellVisualDbcRow {
    fn map_dbc_row(row: &super::DbcFileIteratorRow) -> R<Self> {
        let id = row.get_number_column(1)?;
        let pre_cast_kit_id = row.get_number_column(2)?;
        let cast_kit_id = row.get_number_column(3)?;
        let impact_kit_id = row.get_number_column(4)?;
        let state_kit_id = row.get_number_column(5)?;
        let state_done_kit_id = row.get_number_column(6)?;
        let channel_kit_id = row.get_number_column(7)?;
        let has_middle = row.get_bool_column(8)?;
        let missile_model_id = row.get_number_column(9)?;
        let missile_path_type = row.get_number_column(10)?;
        let missile_destination_attachment = row.get_number_column(11)?;
        let missile_sound = row.get_number_column(12)?;
        let anim_event_sound_id = row.get_number_column(13)?;
        let flags = row.get_number_column(14)?;
        let caster_impact_kit = row.get_number_column(15)?;
        let target_impact_kit = row.get_number_column(16)?;
        let missile_attachment = row.get_number_column(17)?;
        let missile_follow_ground_height = row.get_number_column(18)?;
        let missile_follow_ground_drop_speed = row.get_number_column(19)?;
        let missile_follow_ground_approach = row.get_number_column(20)?;
        let missile_follow_ground_flags = row.get_number_column(21)?;
        let missile_motion = row.get_number_column(22)?;
        let missile_targeting_kit = row.get_number_column(23)?;
        let instant_area_kit = row.get_number_column(24)?;
        let impact_area_kit = row.get_number_column(25)?;
        let persistent_area_kit = row.get_number_column(26)?;
        Ok(SpellVisualDbcRow {
            id,
            pre_cast_kit_id,
            cast_kit_id,
            impact_kit_id,
            state_kit_id,
            state_done_kit_id,
            channel_kit_id,
            has_middle,
            missile_model_id,
            missile_path_type,
            missile_destination_attachment,
            missile_sound,
            anim_event_sound_id,
            flags,
            caster_impact_kit,
            target_impact_kit,
            missile_attachment,
            missile_follow_ground_height,
            missile_follow_ground_drop_speed,
            missile_follow_ground_approach,
            missile_follow_ground_flags,
            missile_motion,
            missile_targeting_kit,
            instant_area_kit,
            impact_area_kit,
            persistent_area_kit,
        })
    }
}
