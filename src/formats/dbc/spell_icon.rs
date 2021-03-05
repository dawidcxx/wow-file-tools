use crate::common::R;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SpellIconDbcRow {
    pub id: u32,
    pub file_name: String,
}

impl super::dbc::DbcRowMapper for SpellIconDbcRow {
    fn map_dbc_row(row: &super::DbcFileIteratorRow) -> R<Self> {
        let id = row.get_number_column(1)?;
        let file_name = row.get_string_column(2)?;
        Ok(SpellIconDbcRow { id, file_name })
    }
}
