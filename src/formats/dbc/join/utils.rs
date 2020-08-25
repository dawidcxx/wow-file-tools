use std::collections::HashMap;
use std::fs::{DirEntry, read_dir};
use crate::common::{R, err};
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

// validates if the folder exist
// and if it has any dbc entries
pub fn common_join_command_validation(
    dbc_folder: &String,
) -> R<DbcLookup> {
    let dbc_folder = PathBuf::from(dbc_folder);

    if !dbc_folder.exists() {
        return err(format!("Folder {} does not exist!", dbc_folder.to_string_lossy()));
    }

    let dbc_file_entries: Vec<DirEntry> = read_dir(&dbc_folder)?
        .filter_map(|entry| entry.ok())
        .filter(|entry| entry.file_name().to_string_lossy().ends_with(".dbc") ||
            entry.file_name().to_string_lossy().ends_with(".DBC")
        )
        .collect();

    if dbc_file_entries.is_empty() {
        return err(format!("DBC Folder {} does not contain any DBC files!", dbc_folder.to_string_lossy()));
    }

    let dbc_lookup = DbcLookup::from_dbc_entries(dbc_file_entries);

    Ok(dbc_lookup)
}


pub fn group_by<T, ID>(
    rows: Vec<T>,
    group_fn: fn(T) -> (ID, T),
) -> HashMap<ID, T>
    where ID: std::hash::Hash + std::cmp::Eq {
    HashMap::from_iter(
        rows.into_iter()
            .map(|it| group_fn(it))
    )
}