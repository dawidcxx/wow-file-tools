use serde::{Deserialize, Serialize};
use crate::formats::dbc::{DbcFile};
use crate::common::R;

#[derive(Debug, Serialize, Deserialize)]
pub struct LightSkyBoxDbcRow {
    id: u32,
    reference_path: String,
    flags: u32,
}

impl LightSkyBoxDbcRow {
    pub fn process(row_builder: &mut Vec<LightSkyBoxDbcRow>, dbc_file: &DbcFile) -> R<()> {
        for row in *&dbc_file {
            let id = row.get_number_column(1)?;
            let reference_path = row.get_string_column(2)?;
            let flags = row.get_number_column(3)?;
            row_builder.push(LightSkyBoxDbcRow {
                id,
                reference_path,
                flags,
            })
        }
        Ok(())
    }
}
