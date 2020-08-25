use serde::{Deserialize, Serialize};
use crate::formats::dbc::{DbcFile};
use crate::common::R;

#[derive(Debug, Serialize, Deserialize)]
pub struct SpellIcon {
    id: u32,
    file_name: String,
}

impl SpellIcon {
    pub fn process(row_builder: &mut Vec<SpellIcon>, dbc_file: &DbcFile) -> R<()> {
        for row in *&dbc_file {
            let id = row.get_number_column(1)?;
            let file_name = row.get_string_column(2)?;
            row_builder.push(SpellIcon {
                id,
                file_name,
            })
        }
        Ok(())
    }
}
