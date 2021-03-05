use crate::common::R;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Copy, Clone)]
pub struct SpellCategoryDbcRow {
    pub id: u32,
    pub flags: u32,
}

impl super::dbc::DbcRowMapper for SpellCategoryDbcRow {
    fn map_dbc_row(row: &super::DbcFileIteratorRow) -> R<Self> {
        let id = row.get_number_column(1)?;
        let flags = row.get_number_column(2)?;
        Ok(SpellCategoryDbcRow { id, flags })
    }
}