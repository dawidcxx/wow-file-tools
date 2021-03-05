use crate::common::R;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct LoadingScreenDbcRow {
    pub id: u32,
    pub name: String,
    pub path: String,
    pub has_wide_screen: bool,
}

impl super::dbc::DbcRowMapper for LoadingScreenDbcRow {
    fn map_dbc_row(row: &super::DbcFileIteratorRow) -> R<Self> {
        let id = row.get_number_column(1)?;
        let name = row.get_string_column(2)?;
        let path = row.get_string_column(3)?;
        let has_wide_screen = row.get_bool_column(4)?;
        Ok(LoadingScreenDbcRow {
            id,
            name,
            path,
            has_wide_screen,
        })
    }
}
