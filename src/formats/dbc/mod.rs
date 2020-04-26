use std::clone::Clone;
use std::fs::File;
use std::io::Read;
use std::convert::TryInto;
use serde::{Serialize, Deserialize};
use std::rc::Rc;
use crate::byte_utils::*;
use crate::common::R;


#[derive(Debug, Clone)]
pub struct DbcHeader {
    // 4
    magic: [u8; 4],
    // 4
    record_count: u32,
    // 4
    field_count: u32,
    // 4
    record_size: u32,
    // 4
    string_block_size: u32,
}

#[derive(Debug, Clone)]
pub struct DbcFile {
    header: DbcHeader,
    file: Rc<Vec<u8>>,
}

pub struct DbcFileIterator {
    file_bytes: Rc<Vec<u8>>,
    offset: usize,
    offset_bump: usize,
    body_end_offset: usize,
}

#[derive(Debug)]
pub struct DbcFileIteratorRow {
    file_bytes: Rc<Vec<u8>>,
    current_offset: usize,
    body_end_offset: usize,
}

impl DbcFileIteratorRow {
    pub fn get_string_column(&self, column: usize) -> R<String> {
        let bytes = self.file_bytes.clone();
        let str_location = bytes.get_u32(self.get_col_offset(column))? as usize;
        Ok(bytes.get_string_null_terminated(self.body_end_offset + str_location)?)
    }

    pub fn get_column_bytes_hex(&self, column: usize) -> R<String> {
        let bytes = self.file_bytes.clone().get_four_bytes(self.get_col_offset(column))?;
        Ok(hex::encode(bytes))
    }

    pub fn get_number_column(&self, column: usize) -> R<u32> {
        self.file_bytes.get_u32(self.get_col_offset(column))
    }

    pub fn get_bool_column(&self, column: usize) -> R<bool> {
        let res = self.get_number_column(column)?;
        Ok(res == 1)
    }

    fn get_col_offset(&self, column: usize) -> usize {
        self.current_offset + (column - 1) * 4
    }
}

impl DbcFile {
    pub fn new(path: &str) -> R<DbcFile> {
        let mut f = File::open(path)?;
        let mut dbc_content = Vec::new();
        f.read_to_end(&mut dbc_content)?;
        let header = get_dbc_header(&dbc_content)?;
        Ok(DbcFile {
            header,
            file: Rc::new(dbc_content),
        })
    }

    pub fn get_strings(&self) -> Vec<String> {
        let grant_offset = 20 + (self.header.record_size * self.header.record_count) as usize;
        let g: Vec<u8> = self.file
            .iter()
            .skip(grant_offset)
            .map(|it| *it)
            .collect();
        let mut strs = Vec::new();
        let mut from: usize = 0;
        let mut to: usize = from + 1;
        for x in &g {
            if *x == 0 {
                let sub_string_bytes: Vec<u8> = g[from..(to - 1)].to_vec();
                let found_string = String::from_utf8(sub_string_bytes)
                    .unwrap();
                strs.push(found_string);
                from = to
            }
            to += 1;
        }
        strs
    }
}

impl IntoIterator for &DbcFile {
    type Item = DbcFileIteratorRow;
    type IntoIter = DbcFileIterator;

    fn into_iter(self) -> Self::IntoIter {
        let body_end_offset = (self.header.record_size * self.header.record_count + 20) as usize;

        DbcFileIterator {
            file_bytes: self.file.clone(),
            offset: 20,
            offset_bump: self.header.record_size as usize,
            body_end_offset,
        }
    }
}

impl Iterator for DbcFileIterator {
    type Item = DbcFileIteratorRow;

    fn next(&mut self) -> Option<Self::Item> {
        if self.offset != self.body_end_offset {
            let current_offset = self.offset;
            self.offset += self.offset_bump;
            Some(DbcFileIteratorRow {
                file_bytes: self.file_bytes.clone(),
                current_offset,
                body_end_offset: self.body_end_offset,
            })
        } else {
            None
        }
    }
}

fn get_dbc_header(dbc_content: &Vec<u8>) -> R<DbcHeader> {
    let magic = dbc_content.get_string(0, 4)?.as_bytes().try_into()?;
    let record_count = dbc_content.get_u32(4)?;
    let field_count = dbc_content.get_u32(8)?;
    let record_size = dbc_content.get_u32(12)?;
    let string_block_size = dbc_content.get_u32(16)?;
    let result = DbcHeader {
        magic,
        record_count,
        field_count,
        record_size,
        string_block_size,
    };
    Ok(result)
}

impl DbcFile {
    pub fn get_map_dbc_rows() -> R<Vec<MapDbcRow>> {
        let mut builder = Vec::with_capacity(10);
        let file = DbcFile::new("dbc/Map.dbc")?;
        for row in &file {
            let row = MapDbcRow {
                id: row.get_number_column(1)?,
                name: row.get_string_column(2)?,
                area_table_ref: row.get_number_column(23)?,
                loading_screen_ref: row.get_number_column(58)?,
            };
            builder.push(row);
        }
        Ok(builder)
    }

    pub fn get_battle_master_list_entries() -> R<Vec<BattleMasterListDbcRow>> {
        let mut builder = Vec::with_capacity(10);
        let file = DbcFile::new("dbc/BattlemasterList.dbc")?;
        for row in &file {
            let row = BattleMasterListDbcRow {
                id: row.get_number_column(1)?,
                map_ref: row.get_number_column(2)?,
                instance_type: row.get_number_column(10)?,
                name: row.get_string_column(12)?,
            };
            if row.is_arena() {
                builder.push(row);
            }
        }
        Ok(builder)
    }

    pub fn get_area_table_entries() -> R<Vec<AreaTableDbcRow>> {
        let mut builder = Vec::with_capacity(10);
        let file = DbcFile::new("dbc/AreaTable.dbc")?;

        for row in &file {
            let row = AreaTableDbcRow {
                id: row.get_number_column(1)?,
                map_ref: row.get_number_column(2)?,
                sub_area_ref: row.get_number_column(3)?,
                area_name: row.get_string_column(12)?,
            };
            builder.push(row);
        }

        Ok(builder)
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MapDbcRow {
    pub id: u32,
    pub name: String,
    pub area_table_ref: u32,
    pub loading_screen_ref: u32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BattleMasterListDbcRow {
    pub id: u32,
    pub map_ref: u32,
    pub instance_type: u32,
    pub name: String,
}

impl BattleMasterListDbcRow {
    pub fn is_arena(&self) -> bool {
        self.instance_type == 4
    }
}


#[derive(Debug, Serialize, Deserialize)]
pub struct AreaTableDbcRow {
    pub  id: u32,
    pub  map_ref: u32,
    pub  sub_area_ref: u32,
    pub area_name: String,
}


