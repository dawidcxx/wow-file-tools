use crate::common::R;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct LightSkyBoxDbcRow {
    id: u32,
    reference_path: String,
    flags: u32,
}

impl super::dbc::DbcRowMapper for LightSkyBoxDbcRow {
    fn map_dbc_row(row: &super::DbcFileIteratorRow) -> R<Self> {
        let id = row.get_number_column(1)?;
        let reference_path = row.get_string_column(2)?;
        let flags = row.get_number_column(3)?;
        Ok(LightSkyBoxDbcRow {
            id,
            reference_path,
            flags,
        })
    }
}
