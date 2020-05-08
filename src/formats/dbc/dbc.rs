use crate::formats::dbc::{DbcHeader, DbcFile};
use serde::{Serialize, Deserialize};
use crate::formats::dbc::map::MapDbcRow;
use crate::common::R;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Dbc<T> {
    pub header: DbcHeader,
    pub rows: Vec<T>,
}

pub fn load_map_dbc_from_path(path: &str) -> R<Dbc<MapDbcRow>> {
    let dbc = DbcFile::new(path)?;
    let mut row_builder = Vec::with_capacity(dbc.header.field_count as usize);

    MapDbcRow::process(&dbc, &mut row_builder)?;

    Ok(Dbc {
        header: dbc.header,
        rows: row_builder,
    })
}
