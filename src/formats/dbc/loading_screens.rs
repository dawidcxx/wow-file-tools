use serde::{Deserialize, Serialize};
use crate::formats::dbc::{DbcFile};
use crate::common::R;


#[derive(Debug, Serialize, Deserialize)]
pub struct LoadingScreenDbcRow {
    id: u32,
    name: String,
    path: String,
    has_wide_screen: bool,
}


impl LoadingScreenDbcRow {
    pub fn process(row_builder: &mut Vec<LoadingScreenDbcRow>, dbc_file: &DbcFile) -> R<()> {
        for row in *&dbc_file {
            let id = row.get_number_column(1)?;
            let name = row.get_string_column(2)?;
            let path = row.get_string_column(3)?;
            let has_wide_screen = row.get_bool_column(4)?;
            row_builder.push(LoadingScreenDbcRow {
                id,
                name,
                path,
                has_wide_screen,
            })
        }
        Ok(())
    }
}