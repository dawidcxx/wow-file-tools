use serde::{Deserialize, Serialize};
use crate::formats::dbc::{DbcFile};
use crate::common::R;

#[derive(Debug, Serialize, Deserialize)]
pub struct PvpDifficulty {
    id: u32,
    map_id: u32,
    range_index: u32,
    min_level: u32,
    max_level: u32,
    difficulty: u32,
}

impl PvpDifficulty {
    pub fn process(row_builder: &mut Vec<PvpDifficulty>, dbc_file: &DbcFile) -> R<()> {
        for row in *&dbc_file {
            let id = row.get_number_column(1)?;
            let map_id = row.get_number_column(2)?;
            let range_index = row.get_number_column(3)?;
            let min_level = row.get_number_column(4)?;
            let max_level = row.get_number_column(5)?;
            let difficulty = row.get_number_column(6)?;
            row_builder.push(PvpDifficulty {
                id,
                map_id,
                range_index,
                min_level,
                max_level,
                difficulty
            })
        }
        Ok(())
    }
}
