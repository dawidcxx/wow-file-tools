use serde::{Deserialize, Serialize};
use crate::formats::dbc::{DbcFile};
use crate::common::R;


#[derive(Debug, Serialize, Deserialize)]
pub struct BattleMasterListDbcRow {
    id: u32,
    map_ref_ids: [i32; 8],
    instance_type: u32,
    groups_allowed: bool,
    name: String,
    max_group_size: u32,
    holiday_world_state: u32,
    min_level: u32,
    max_level: u32,
}

impl BattleMasterListDbcRow {
    pub fn process(row_builder: &mut Vec<BattleMasterListDbcRow>, dbc_file: &DbcFile) -> R<()> {
        for row in *&dbc_file {
            let id = row.get_number_column(1)?;
            let map_ref_id_1 = row.get_number_column_signed(2)?;
            let map_ref_id_2 = row.get_number_column_signed(3)?;
            let map_ref_id_3 = row.get_number_column_signed(4)?;
            let map_ref_id_4 = row.get_number_column_signed(5)?;
            let map_ref_id_5 = row.get_number_column_signed(6)?;
            let map_ref_id_6 = row.get_number_column_signed(7)?;
            let map_ref_id_7 = row.get_number_column_signed(8)?;
            let map_ref_id_8 = row.get_number_column_signed(9)?;
            let map_ref_ids = [
                map_ref_id_1,
                map_ref_id_2,
                map_ref_id_3,
                map_ref_id_4,
                map_ref_id_5,
                map_ref_id_6,
                map_ref_id_7,
                map_ref_id_8,
            ];
            let instance_type = row.get_number_column(10)?;
            let groups_allowed = row.get_bool_column(11)?;
            let name = row.get_string_column(12)?;
            let max_group_size = row.get_number_column(29)?;
            let holiday_world_state = row.get_number_column(30)?;
            let min_level = row.get_number_column(31)?;
            let max_level = row.get_number_column(32)?;

            row_builder.push(BattleMasterListDbcRow {
                id,
                map_ref_ids,
                instance_type,
                groups_allowed,
                name,
                max_group_size,
                holiday_world_state,
                min_level,
                max_level
            })

        }
        Ok(())
    }

    pub fn is_arena(&self) -> bool {
        self.instance_type == 4
    }

}