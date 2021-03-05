use serde::{Deserialize, Serialize};
use crate::common::R;

#[derive(Debug, Serialize, Deserialize)]
pub struct GroundEffectDoodadDbcRow {
    id: u32,
    ground_models: String,
    flags: u32,
}

impl super::dbc::DbcRowMapper for GroundEffectDoodadDbcRow {
    fn map_dbc_row(row: &super::DbcFileIteratorRow) -> R<Self> {
        let id = row.get_number_column(1)?;
        let ground_models = row.get_string_column(2)?;
        let flags = row.get_number_column(3)?;
        Ok(GroundEffectDoodadDbcRow {
            id,
            ground_models,
            flags,
        })
    }
}
