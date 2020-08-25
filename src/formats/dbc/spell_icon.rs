use serde::{Deserialize, Serialize};
use crate::formats::dbc::{DbcFile};
use crate::common::R;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SpellIconDbcRow {
    pub id: u32,
    pub file_name: String,
}

impl SpellIconDbcRow {
    pub fn process(row_builder: &mut Vec<SpellIconDbcRow>, dbc_file: &DbcFile) -> R<()> {
        for row in *&dbc_file {
            let id = row.get_number_column(1)?;
            let file_name = row.get_string_column(2)?;
            row_builder.push(SpellIconDbcRow {
                id,
                file_name,
            })
        }
        Ok(())
    }
}
