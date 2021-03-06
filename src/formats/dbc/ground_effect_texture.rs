use crate::common::R;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct GroundEffectTextureDbcRow {
    id: u32,
    effect_doodad_ref_ids: [u32; 4],
    weights: [u32; 4],
    amount_and_coverage: u32,
    terrain_type_ref_id: u32,
}

impl super::dbc::DbcRowMapper for GroundEffectTextureDbcRow {
    fn map_dbc_row(row: &super::DbcFileIteratorRow) -> R<Self> {
        let id = row.get_number_column(1)?;
        let e1 = row.get_number_column(2)?;
        let e2 = row.get_number_column(3)?;
        let e3 = row.get_number_column(4)?;
        let e4 = row.get_number_column(5)?;
        let effect_doodad_ref_ids = [e1, e2, e3, e4];
        let w1 = row.get_number_column(6)?;
        let w2 = row.get_number_column(7)?;
        let w3 = row.get_number_column(8)?;
        let w4 = row.get_number_column(9)?;
        let weights = [w1, w2, w3, w4];
        let amount_and_coverage = row.get_number_column(10)?;
        let terrain_type_ref_id = row.get_number_column(11)?;
        Ok(GroundEffectTextureDbcRow {
            id,
            effect_doodad_ref_ids,
            weights,
            amount_and_coverage,
            terrain_type_ref_id,
        })
    }
}
