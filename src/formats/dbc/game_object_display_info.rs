use crate::common::R;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct GameObjectDisplayInfo {
    id: u32,
    model_name: String,
}

impl super::dbc::DbcRowMapper for GameObjectDisplayInfo {
    fn map_dbc_row(row: &super::DbcFileIteratorRow) -> R<Self> {
        let id = row.get_number_column(1)?;
        let model_name = row.get_string_column(2)?;
        Ok(GameObjectDisplayInfo { id, model_name })
    }
}
