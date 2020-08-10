pub mod map;
pub mod dbc;
pub mod loading_screens;
pub mod area_table;
pub mod light_sky_box;
pub mod light;
pub mod battle_master_list;
pub mod ground_effect_texture;
pub mod ground_effect_doodad;

use std::clone::Clone;
use std::fs::File;
use std::io::Read;
use std::convert::TryInto;
use serde::{Serialize, Deserialize};
use std::rc::Rc;
use crate::byte_utils::*;
use crate::common::R;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DbcHeader {
    // 4
    pub magic: [u8; 4],
    // 4
    pub record_count: u32,
    // 4
    pub field_count: u32,
    // 4
    pub record_size: u32,
    // 4
    pub string_block_size: u32,
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

    pub fn get_number_column_signed(&self, column: usize) -> R<i32> {
        self.file_bytes.get_i32(self.get_col_offset(column))
    }

    pub fn get_float_column(&self, column: usize) -> R<f32> {
        self.file_bytes.get_f32(self.get_col_offset(column))
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
