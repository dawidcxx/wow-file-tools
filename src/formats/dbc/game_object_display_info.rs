use serde::{Deserialize, Serialize};
use crate::formats::dbc::{DbcFile};
use crate::common::R;

#[derive(Debug, Serialize, Deserialize)]
pub struct GameObjectDisplayInfo {
    id: u32,
    model_name: String,
}

impl GameObjectDisplayInfo {
    pub fn process(row_builder: &mut Vec<GameObjectDisplayInfo>, dbc_file: &DbcFile) -> R<()> {
        for row in *&dbc_file {
            let id = row.get_number_column(1)?;
            let model_name = row.get_string_column(2)?;
            row_builder.push(GameObjectDisplayInfo {
                id,
                model_name,
            })
        }
        Ok(())
    }
}
