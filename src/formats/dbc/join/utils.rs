use std::collections::HashMap;
use std::fs::DirEntry;
use crate::common::R;
use std::path::PathBuf;
use std::iter::FromIterator;


pub fn has_bit_flag(
    value: u32,
    flag: u32,
) -> bool { value & flag == flag }


pub struct DbcLookup {
    lookup: HashMap<String, DirEntry>,
}

impl DbcLookup {
    pub fn from_dbc_entries(
        dbc_entries: Vec<DirEntry>
    ) -> DbcLookup {
        DbcLookup {
            lookup: HashMap::from_iter(
                dbc_entries.into_iter()
                    .map(|dir_entry| {
                        let file_name = dir_entry
                            .file_name()
                            .clone()
                            .to_string_lossy()
                            .to_string();
                        (file_name, dir_entry)
                    })
            )
        }
    }

    pub fn get(
        &self,
        dbc_file_name: &str,
    ) -> R<PathBuf> {
        self.lookup.get(dbc_file_name)
            .map(|entry| entry.path())
            .ok_or(format!("DBC {} not found in provided dbc folder", dbc_file_name).into())
    }
}
