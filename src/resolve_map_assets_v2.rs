use std::path::{PathBuf, Path};
use crate::common::{R, err};
use serde::{Serialize, Deserialize};
use crate::resolve_map_assets_v2::ResolveMapAssetsCmdWarn::MISSING;
use std::fs::read_dir;
use crate::formats::dbc::dbc::load_map_dbc_from_path;
use crate::formats::dbc::map::MapDbcRow;
use walkdir::{DirEntry, WalkDir};
use crate::formats::adt::AdtFile;
use crate::formats::wmo::WmoFile;
use crate::formats::m2::M2File;

#[derive(Debug, Serialize, Deserialize)]
pub enum ResolveMapAssetsCmdWarn {
    MISSING(String),
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ResolveMapAssetsCmdResult {
    pub warns: Vec<ResolveMapAssetsCmdWarn>,
    pub results: Vec<PathBuf>,
}

pub fn resolve_map_assets(
    workspace_path: &Path,
    map_id: u32,
) -> R<ResolveMapAssetsCmdResult> {
    if !workspace_path.exists() {
        return err(format!("Error: workspace '{:?}' not found on the file system.", workspace_path));
    }

    let mut results = Vec::new();
    let mut warns: Vec<ResolveMapAssetsCmdWarn> = Vec::new();

    let map_dbc_loc = join_path_ignoring_casing(workspace_path, "DBFilesClient/Map.dbc")
        .ok_or("Missing Map.dbc file")?;

    let map_dbc = load_map_dbc_from_path(map_dbc_loc.str())?;

    let map_row = map_dbc.rows
        .iter()
        .find(|map| map.id == map_id)
        .ok_or(format!("Map with id {} not found", map_id))?;

    results.push(map_dbc_loc);

    let maps_folder = join_path_ignoring_casing(
        workspace_path,
        "World/Maps",
    ).ok_or("Missing World/Maps folder in workspace")?;

    // to(maybe)do: these could be a warning.
    let wdt_file_path = get_wdt_path(&maps_folder, map_row)
        .ok_or("Missing Map WDT file")?;
    let wdl_file_path = get_wdl_path(&maps_folder, map_row)
        .ok_or("Missing Map WDL file")?;

    results.push(wdl_file_path);
    results.push(wdt_file_path);

    for adt_entry in find_files_by_extension(maps_folder, 2, ".adt") {
        let adt_path = adt_entry.into_path();
        results.push(adt_path.clone());

        let adt = AdtFile::from_path(adt_path)?;

        add_wow_dep(
            workspace_path,
            adt.mtex.0,
            &mut results,
            &mut warns,
        );

        let added_wmos = add_wow_dep(
            workspace_path,
            adt.mwmo.0,
            &mut results,
            &mut warns,
        );

        add_m2_type_wow_dep(
            workspace_path,
            adt.mmdx.0,
            &mut results,
            &mut warns,
        );

        for wmo_path in added_wmos {
            let wmo = WmoFile::from_path(wmo_path.str())?;
            add_wow_dep(
                workspace_path,
                wmo.root.motx.0,
                &mut results,
                &mut warns,
            );
            add_m2_type_wow_dep(
                workspace_path,
                wmo.root.modn.0,
                &mut results,
                &mut warns,
            );
        }
    }


    Ok(ResolveMapAssetsCmdResult {
        warns,
        results,
    })
}

fn add_wow_dep(
    workspace_root: &Path,
    wow_dependency_list: Vec<String>,
    results: &mut Vec<PathBuf>,
    warns: &mut Vec<ResolveMapAssetsCmdWarn>,
) -> Vec<PathBuf> {
    let mut added = Vec::new();
    for dependency in wow_dependency_list {
        if let Some(path) = join_path_ignoring_casing(workspace_root, dependency.as_str()) {
            added.push(path.clone());
            results.push(path)
        } else {
            warns.push(MISSING(dependency));
        }
    }
    added
}


// m2/mdx's are a bit "special"
fn add_m2_type_wow_dep(
    workspace_root: &Path,
    wow_dependency_list: Vec<String>,
    results: &mut Vec<PathBuf>,
    warns: &mut Vec<ResolveMapAssetsCmdWarn>,
) -> Vec<PathBuf> {
    let mut added = Vec::new();

    fn on_found(
        path: PathBuf,
        added: &mut Vec<PathBuf>,
        results: &mut Vec<PathBuf>,
    ) {
        added.push(path.clone());
        results.push(path.clone());

    }

    for dependency in wow_dependency_list {
        if let Some(path) = join_path_ignoring_casing(workspace_root, dependency.as_str()) {
            on_found(path, &mut added, results);
        } else {
            let mut found = false;

            if dependency.ends_with("mdx") || dependency.ends_with("MDX") {
                // retry, replacing extension to .m2
                let dependency = dependency.replace(".mdx", ".m2")
                    .replace(".MDX", ".M2");
                if let Some(path) = join_path_ignoring_casing(workspace_root, dependency.as_str()) {
                    found = true;
                    on_found(path, &mut added, results);
                }
            } else if dependency.ends_with("m2") || dependency.ends_with("M2") {
                // retry, replacing extension to .mdx
                let dependency = dependency.replace(".M2", ".MDX")
                    .replace(".m2", ".mdx");
                if let Some(path) = join_path_ignoring_casing(workspace_root, dependency.as_str()) {
                    found = true;
                    on_found(path, &mut added, results);
                }
            }

            if !found {
                warns.push(MISSING(dependency));
            }
        }
    }


    added
}


fn join_path_ignoring_casing(
    base: &Path,
    join: &str,
) -> Option<PathBuf> {
    let parts: Vec<&str> = join.split(&['/', '\\'][..]).collect();
    let mut buf = PathBuf::new();
    buf.push(base.clone());

    for part in parts {
        let part = part.to_uppercase();
        if let Ok(read_dir) = read_dir(&buf) {
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

    Some(buf)
}

trait PathBufUtils {
    fn str(&self) -> &str;
}

impl PathBufUtils for PathBuf {
    fn str(&self) -> &str {
        self.to_str().expect("Failed to convert PathBuf -> &str. Invalid FileSystem path characters?")
    }
}

fn find_files_by_extension<P: AsRef<Path>>(
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

fn get_wdt_path(maps_folder: &PathBuf, map_row: &MapDbcRow) -> Option<PathBuf> {
    let loc = maps_folder.join(format!("{0}/{0}.wdt", map_row.internal_name));
    if loc.exists() {
        Some(loc)
    } else {
        None
    }
}

fn get_wdl_path(maps_folder: &PathBuf, map_row: &MapDbcRow) -> Option<PathBuf> {
    let loc = maps_folder.join(format!("{0}/{0}.wdl", map_row.internal_name));
    if loc.exists() {
        Some(loc)
    } else {
        None
    }
}