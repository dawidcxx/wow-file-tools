use serde::{Deserialize, Serialize};
use crate::formats::dbc::{DbcFile};
use crate::common::R;

#[derive(Debug, Serialize, Deserialize, Copy, Clone)]
pub struct SpellCategoryDbcRow {
    pub id: u32,
    pub flags: u32,
}

impl SpellCategoryDbcRow {
    pub fn process(row_builder: &mut Vec<SpellCategoryDbcRow>, dbc_file: &DbcFile) -> R<()> {
        for row in *&dbc_file {
            let id = row.get_number_column(1)?;
            let flags = row.get_number_column(2)?;
            row_builder.push(SpellCategoryDbcRow {
                id,
                flags,
            })
        }
        Ok(())
    }
}
