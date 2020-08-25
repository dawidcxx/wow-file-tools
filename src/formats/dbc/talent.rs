use serde::{Deserialize, Serialize};
use crate::formats::dbc::{DbcFile};
use crate::common::R;

#[derive(Debug, Serialize, Deserialize)]
pub struct TalentDbcRow {
    pub id: u32,
    pub talent_tab_id: u32,
    pub tier: u32,
    pub column_index: u32,
    pub spell_rank_ids: [u32; 9],
    pub required_talent_ids: [u32; 3],
    pub required_talent_point_rank_ids: [u32; 3],
    pub only_one_point: bool,
}

impl TalentDbcRow {
    pub fn process(row_builder: &mut Vec<TalentDbcRow>, dbc_file: &DbcFile) -> R<()> {
        for row in *&dbc_file {
            let id = row.get_number_column(1)?;
            let talent_tab_id = row.get_number_column(2)?;
            let tier_id = row.get_number_column(3)?;
            let column_index = row.get_number_column(4)?;
            let spell_rank_ids = {
                let sr1 = row.get_number_column(5)?;
                let sr2 = row.get_number_column(6)?;
                let sr3 = row.get_number_column(7)?;
                let sr4 = row.get_number_column(8)?;
                let sr5 = row.get_number_column(9)?;
                let sr6 = row.get_number_column(10)?;
                let sr7 = row.get_number_column(11)?;
                let sr8 = row.get_number_column(12)?;
                let sr9 = row.get_number_column(13)?;
                [sr1, sr2, sr3, sr4, sr5, sr6, sr7, sr8, sr9]
            };
            let required_talent_ids = {
                let rti1 = row.get_number_column(14)?;
                let rti2 = row.get_number_column(15)?;
                let rti3 = row.get_number_column(16)?;
                [rti1, rti2, rti3]
            };
            let required_talent_point_rank_ids = {
                let ri1 = row.get_number_column(17)?;
                let ri2 = row.get_number_column(18)?;
                let ri3 = row.get_number_column(19)?;
                [ri1, ri2, ri3]
            };
            let only_one_point = row.get_bool_column(20)?;

            row_builder.push(TalentDbcRow {
                id,
                talent_tab_id,
                tier: tier_id,
                column_index,
                spell_rank_ids,
                required_talent_ids,
                required_talent_point_rank_ids,
                only_one_point,
            });
        }
        Ok(())
    }
}
