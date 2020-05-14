use std::path::{Path, PathBuf};
use crate::common::R;
use crate::{ResolveMapAssetsCmd};
use crate::formats::dbc::dbc::load_map_dbc_from_path;
use crate::formats::dbc::map::MapDbcRow;
use walkdir::{WalkDir, DirEntry};
use crate::formats::adt::AdtFile;
use std::fs::read_dir;
use serde::{Serialize, Deserialize};
use crate::formats::wmo::WmoFile;

#[derive(Debug, Serialize, Deserialize)]
pub enum ResolveMapAssetsCmdWarn {
    MISSING(String),
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ResolveMapAssetsCmdResult {
    pub warns: Vec<ResolveMapAssetsCmdWarn>,
    pub dependencies: Vec<String>,
}

pub fn resolve_map_assets(
    cmd: &ResolveMapAssetsCmd
) -> R<ResolveMapAssetsCmdResult> {
    let mut map_dependencies = Vec::new();
    let mut warns = Vec::new();
    let workspace_root = Path::new(cmd.workspace.as_str());
    let map_dbc = load_map_dbc_from_path(
        workspace_root.join("DBFilesClient/Map.dbc").to_str().unwrap()
    )?;
    let map_row = map_dbc.rows.into_iter()
        .find(|it| it.id == cmd.map_id as u32)
        .unwrap();

    let wdt_file_name = get_adt_file_name(map_row);
    let wdt_file = find_file_by_filename(&workspace_root.join("World/Maps"), 2, wdt_file_name.as_str());
    if wdt_file.is_empty() {
        // ensure wdt file exist
        let msg = format!(
            "Could not locate {0} within the workspace, ensure $WORKSPACE/World/Maps/[..]/{0} exist!",
            wdt_file_name
        );
        return Err(msg.into());
    }

    for adt_entry in find_file_by_extension(&workspace_root.join("World/Maps"), 2, ".adt") {
        let adt_path = adt_entry.path();
        let adt = AdtFile::from_path(adt_path)?;
        push_dep(&mut map_dependencies, &adt.mtex.0);
        push_dep(&mut map_dependencies, &adt.mmdx.0);
        push_dep(&mut map_dependencies, &adt.mwmo.0);
        for wmo in adt.mwmo.0 {
            let wmo = normalize_path(&wmo);
            if let Some(wmo_path) = join_path_ignoring_casing(workspace_root, &wmo) {
                let wmo = WmoFile::from_path(wmo_path.to_str().unwrap())?;
                push_dep(&mut map_dependencies, &wmo.root.motx.0);
                push_dep(&mut map_dependencies, &wmo.root.modn.0);
            } else {
                let msg = format!(
                    "ADT {} wmo chunk {} could not be found on the disk.",
                    adt_path.to_string_lossy(), wmo
                );
                warns.push(ResolveMapAssetsCmdWarn::MISSING(msg));
            }
        }
    }


    Ok(ResolveMapAssetsCmdResult {
        warns,
        dependencies: map_dependencies,
    })
}


fn push_dep(dependency_list: &mut Vec<String>, src: &Vec<String>) {
    for dep in src {
        dependency_list.push(normalize_path(dep));
    }
}

fn normalize_path(dep: &String) -> String {
    dep
        .replace(r"\", "/")
        .to_uppercase()
}

fn get_adt_file_name(map_row: MapDbcRow) -> String {
    let mut adt_file_name = String::new();
    adt_file_name.push_str(map_row.internal_name.as_str());
    adt_file_name.push_str(".wdt");
    adt_file_name
}


fn find_file_by_filename<P: AsRef<Path>>(
    path: P,
    depth: usize,
    file_name: &str,
) -> Vec<DirEntry> {
    let file_name = file_name.to_string().to_uppercase();
    find_file_by_predicate(path, depth, Box::new(move |entry| {
        let curr_file_name = entry.file_name().to_str().unwrap().to_uppercase();
        curr_file_name.eq(&file_name)
    }))
}


fn find_file_by_extension<P: AsRef<Path>>(
    path: P,
    depth: usize,
    extension: &str,
) -> Vec<DirEntry> {
    let extension = extension.to_string().to_uppercase();
    find_file_by_predicate(path, depth, Box::new(move |entry| {
        let curr_file_name = entry.file_name().to_str().unwrap().to_uppercase();
        curr_file_name.ends_with(&extension)
    }))
}

type FileFinderPredicate = dyn Fn(&DirEntry) -> bool;

fn find_file_by_predicate<P: AsRef<Path>>(
    path: P,
    depth: usize,
    predicate: Box<FileFinderPredicate>,
) -> Vec<DirEntry> {
    let mut res = Vec::new();
    for entry in WalkDir::new(path)
        .max_depth(depth)
        .into_iter()
        .filter_map(|e| e.ok())
    {
        let metadata = entry.metadata();
        if metadata.is_err() { continue; }
        let metadata = metadata.unwrap();
        if metadata.is_dir() { continue; }
        if predicate.call((&entry, )) {
            res.push(entry);
        }
    }
    res
}

fn join_path_ignoring_casing(
    base: &Path,
    join: &String,
) -> Option<Box<Path>> {
    let parts: Vec<&str> = join.split("/").collect();
    let mut buf = PathBuf::new();
    buf.push(base);

    for part in parts {
        let part = part.to_uppercase();
        if let Some(read_dir) = read_dir(&buf).ok() {
            let next = read_dir
                .filter_map(|it| it.ok())
                .find(|dir_entry| {
                    let dir_entry_file_name = dir_entry.file_name().to_str().unwrap().to_uppercase();
                    dir_entry_file_name.eq(&part)
                });
            match next {
                Some(next_path) => {
                    buf.push(next_path.file_name());
                }
                None => {
                    return None;
                }
            }
        } else {
            panic!("#join_path_ignoring_casing should not arrive at a invalid path: {:?}", buf);
        }
    }

    Some(buf.into_boxed_path())
}