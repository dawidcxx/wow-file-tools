use serde::{Deserialize, Serialize};
use crate::formats::dbc::{DbcFile};
use crate::common::R;

#[derive(Debug, Serialize, Deserialize)]
pub struct TalentTabRow {
    pub id: u32,
    pub name: String,
    pub spell_icon_id: u32,
    pub race_mask: u32,
    pub class_mask: u32,
    pub hunter_pet_category_id: u32,
    pub order_index: u32,
    pub background_file: String,

}

impl TalentTabRow {
    pub fn process(row_builder: &mut Vec<TalentTabRow>, dbc_file: &DbcFile) -> R<()> {
        for row in *&dbc_file {
            let id = row.get_number_column(1)?;
            let name = row.get_string_column(2)?;
            let spell_icon_id = row.get_number_column(19)?;
            let race_mask = row.get_number_column(20)?;
            let class_mask = row.get_number_column(21)?;
            let hunter_pet_category_id = row.get_number_column(22)?;
            let order_index = row.get_number_column(23)?;
            let background_file = row.get_string_column(24)?;
            row_builder.push(TalentTabRow {
                id,
                name,
                spell_icon_id,
                race_mask,
                class_mask,
                hunter_pet_category_id,
                order_index,
                background_file,
            })
        }
        Ok(())
    }
}
