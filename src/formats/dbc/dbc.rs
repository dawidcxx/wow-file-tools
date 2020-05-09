use crate::formats::dbc::{DbcHeader, DbcFile};
use serde::{Serialize, Deserialize};
use crate::formats::dbc::map::MapDbcRow;
use crate::common::R;
use crate::formats::dbc::loading_screens::LoadingScreenDbcRow;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Dbc<T> {
    pub header: DbcHeader,
    pub rows: Vec<T>,
}

type DbcRowProcessor<T> = dyn Fn(&mut Vec<T>, &DbcFile) -> R<()>;

fn load_dbc<T>(path: &str, row_mapper: Box<DbcRowProcessor<T>>) -> R<Dbc<T>>
{
    let dbc = DbcFile::new(path)?;
    let mut row_builder = Vec::with_capacity(dbc.header.field_count as usize);
    row_mapper.call((&mut row_builder, &dbc))?;
    Ok(Dbc {
        header: dbc.header,
        rows: row_builder
    })
}

pub fn load_map_dbc_from_path(path: &str) -> R<Dbc<MapDbcRow>> {
    load_dbc(path, Box::new(MapDbcRow::process))
}


pub fn load_loading_screens_dbc_from_path(path: &str) -> R<Dbc<LoadingScreenDbcRow>> {
    load_dbc(path, Box::new(LoadingScreenDbcRow::process))
}
