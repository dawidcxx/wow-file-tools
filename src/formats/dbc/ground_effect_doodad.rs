use serde::{Deserialize, Serialize};
use crate::formats::dbc::{DbcFile};
use crate::common::R;


#[derive(Debug, Serialize, Deserialize)]
pub struct GroundEffectDoodadDbcRow {
    id: u32,
    ground_models: String,
    flags: u32,
}


impl GroundEffectDoodadDbcRow {
    pub fn process(row_builder: &mut Vec<GroundEffectDoodadDbcRow>, dbc_file: &DbcFile) -> R<()> {
        for row in *&dbc_file {
            let id = row.get_number_column(1)?;
            let ground_models = row.get_string_column(2)?;
            let flags = row.get_number_column(3)?;
            row_builder.push(GroundEffectDoodadDbcRow {
                id,
                ground_models,
                flags,
            })
        }
        Ok(())
    }
}