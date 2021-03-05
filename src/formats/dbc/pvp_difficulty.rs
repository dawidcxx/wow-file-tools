use crate::common::R;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct PvpDifficulty {
    id: u32,
    map_id: u32,
    range_index: u32,
    min_level: u32,
    max_level: u32,
    difficulty: u32,
}

impl super::dbc::DbcRowMapper for PvpDifficulty {
    fn map_dbc_row(row: &super::DbcFileIteratorRow) -> R<Self> {
        let id = row.get_number_column(1)?;
        let map_id = row.get_number_column(2)?;
        let range_index = row.get_number_column(3)?;
        let min_level = row.get_number_column(4)?;
        let max_level = row.get_number_column(5)?;
        let difficulty = row.get_number_column(6)?;
        Ok(PvpDifficulty {
            id,
            map_id,
            range_index,
            min_level,
            max_level,
            difficulty,
        })
    }
}
