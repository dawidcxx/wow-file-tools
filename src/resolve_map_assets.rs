use std::path::{Path, PathBuf};
use crate::common::R;
use crate::{ResolveMapAssetsCmd};
use crate::formats::dbc::dbc::load_map_dbc_from_path;
use crate::formats::dbc::map::MapDbcRow;
use walkdir::{WalkDir, DirEntry};
use crate::formats::adt::AdtFile;
use std::fs::read_dir;
use serde::{Serialize, Deserialize};
use crate::formats::wmo::{WmoFile};
use std::collections::HashSet;
use std::iter::FromIterator;
use std::fs;
use crate::formats::m2::M2File;

#[derive(Debug, Serialize, Deserialize)]
pub enum ResolveMapAssetsCmdWarn {
    MISSING(String),
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ResolveMapAssetsCmdResult {
    pub warns: Vec<ResolveMapAssetsCmdWarn>,
    pub dependencies: Vec<PathBuf>,
}

pub fn resolve_map_assets(
    cmd: &ResolveMapAssetsCmd
) -> R<ResolveMapAssetsCmdResult> {
    let mut dependencies: Vec<PathBuf> = Vec::new();
    let mut map_peer_dependencies: Vec<PathBuf> = Vec::new();
    let mut warns: Vec<ResolveMapAssetsCmdWarn> = Vec::new();
    let mut map_peer_warns: Vec<ResolveMapAssetsCmdWarn> = Vec::new();

    let workspace_root = Path::new(cmd.workspace.as_str());

    let map_dbc = load_map_dbc_from_path(
        workspace_root.join("DBFilesClient/Map.dbc").to_str().unwrap()
    )?;

    let map_dbc_row: MapDbcRow = map_dbc.rows.into_iter()
        .find(|it| it.id == cmd.map_id as u32)
        .map_or(Err(format!("Map with given id {} not found", cmd.map_id).into()) as R<MapDbcRow>, Ok)?;
    let map_folder = get_maps_map_folder(workspace_root, &map_dbc_row)?;

    add_wdt_dep(&mut dependencies, &map_folder, &map_dbc_row)?;
    add_wdl_dep(&mut dependencies, &mut warns, &map_dbc_row, &map_folder);

    // helper lambda to add a peer dependency
    let mut add_peer_dep = |origin: &str, marker: &str, asset_list: &Vec<String>| {
        for asset in asset_list {
            let asset = normalize_path(asset);
            let dependency_path_opt = join_path_ignoring_casing(workspace_root, &asset);
            if dependency_path_opt.is_some() {
                let dependency_path = dependency_path_opt.unwrap();
                map_peer_dependencies.push(dependency_path);
            } else {
                let msg = format!(
                    "In file '{}' field '{}' with value '{}' could not be resolved on the disk.",
                    origin, marker, asset
                );
                map_peer_warns.push(ResolveMapAssetsCmdWarn::MISSING(msg));
            }
        }
    };

    for adt_entry in find_file_by_extension(&map_folder, 2, ".adt") {
        let adt_path = adt_entry.path();
        dependencies.push(adt_path.to_path_buf());

        let adt = AdtFile::from_path(adt_path)?;

        add_peer_dep("ADT", "mtex", &adt.mtex.0);
        add_peer_dep("ADT", "mwmo", &adt.mwmo.0);
        add_peer_dep("ADT", "mmdx", &gen_m2_alternatives_from_mdx(workspace_root, &adt.mmdx.0));

        for wmo in adt.mwmo.0 {
            let wmo = normalize_path(&wmo);
            if let Some(wmo_path) = join_path_ignoring_casing(workspace_root, &wmo) {
                let wmo = WmoFile::from_path(wmo_path.to_str().unwrap())?;
                dependencies.push(wmo_path);
                add_peer_dep("WMO", "motx", &wmo.root.motx.0);
                add_peer_dep("WMO", "modn", &wmo.root.modn.0);
                add_peer_dep("WMO", "modn", &gen_m2_alternatives_from_mdx(workspace_root, &wmo.root.modn.0));
            } else {
                let msg = format!(
                    "ADT {} wmo chunk {} could not be found on the disk.",
                    adt_path.to_string_lossy(), wmo
                );
                warns.push(ResolveMapAssetsCmdWarn::MISSING(msg));
            }
        }
    }
    // let cpy = map_peer_dependencies.clone();
    //
    // let m2s: Vec<String> = cpy
    //     .iter()
    //     .filter(|it| it.extension().unwrap().eq("m2") || it.extension().unwrap().eq("M2"))
    //     .map(|p| M2File::from_path(p))
    //     .filter_map(|it| it.ok())
    //     .flat_map(|m2| m2.textures)
    //     .collect();
    //
    // add_peer_dep("M2", "m2", &vec![]);
    //

    dependencies.append(&mut map_peer_dependencies);
    warns.append(&mut map_peer_warns);


    verify_dependencies(&mut dependencies)?;

    prune_garbage(workspace_root, &dependencies);

    Ok(ResolveMapAssetsCmdResult {
        warns,
        dependencies,
    })
}

// sometimes we got mdx's as dependencies
// but .m2's will also work.
fn gen_m2_alternatives_from_mdx(
    workspace_root: &Path,
    mdxs: &Vec<String>,
) -> Vec<String> {
    let mut join = Vec::with_capacity(mdxs.len() * 2);
    // let mut blps = Vec::new();
    let mut patched = mdxs
        .iter()
        .filter(|v| v.ends_with("MDX"))
        .map(|v| v.replace("MDX", "M2"))
        .map(|v| v.replace("mdx", "m2"))
        .collect();

    // for x in patched.iter() {
    //     if let Some(dep) = join_path_ignoring_casing(workspace_root, x) {
    //         if let Ok(m2) = M2File::from_path(dep) {
    //             blps.push(&mut m2.textures.clone())
    //         }
    //     }
    // }

    join.append(&mut patched);
    join.append(&mut mdxs.clone());
    join
}

fn prune_garbage(
    workspace_root: &Path,
    dependencies: &Vec<PathBuf>,
) {
    let dep_cache: HashSet<PathBuf> = HashSet::from_iter(dependencies
        .iter()
        .map(|it| {
            fs::canonicalize(it).unwrap()
        })
    );

    for entry in WalkDir::new(workspace_root)
        .into_iter()
        .filter_map(|e| e.ok()) {
        if entry.metadata().unwrap().is_dir() {
            // ignore dirs
            continue;
        }

        let path = fs::canonicalize(entry.path()).unwrap();

        if !dep_cache.contains(&path) {
            // println!("PISSING {:?}", path);
        }
    }
}

fn verify_dependencies(dependencies: &Vec<PathBuf>) -> R<()> {
    for dependency in dependencies {
        if !dependency.exists() {
            let msg = format!("Program Error. Dependency {:?} not found on the disk. ", dependency);
            return Err(msg.into());
        }
    }
    Ok(())
}

fn add_wdt_dep(dependencies: &mut Vec<PathBuf>, map_folder: &PathBuf, map_dbc_row: &MapDbcRow) -> R<()> {
    let wdt_file = get_wdt_file(&map_folder, map_dbc_row)?;
    dependencies.push(wdt_file);
    Ok(())
}

fn get_maps_map_folder(
    workspace_root: &Path,
    map_dbc_row: &MapDbcRow,
) -> R<PathBuf> {
    let mut buf = PathBuf::new();
    buf.push(workspace_root);
    buf.push("World/Maps");
    buf.push(map_dbc_row.internal_name.clone());

    if !buf.exists() {
        let msg = format!(
            "It seems like the workspace is lacking a World/Maps folder"
        );
        return Err(msg.into());
    }

    Ok(buf)
}

fn add_wdl_dep(
    dependencies: &mut Vec<PathBuf>,
    warns: &mut Vec<ResolveMapAssetsCmdWarn>,
    map_row: &MapDbcRow,
    map_folder: &PathBuf,
) {
    let wdl_file_name = get_wdl_file_name(&map_row);
    let wdl_path = join_path_ignoring_casing(map_folder, &wdl_file_name);
    if let Some(wdl_path) = wdl_path {
        dependencies.push(wdl_path)
    } else {
        let msg = format!(
            "WDL {} could not be found on the disk",
            wdl_file_name
        );
        warns.push(ResolveMapAssetsCmdWarn::MISSING(msg));
    }
}

fn get_wdt_file(
    map_folder: &Path,
    map_dbc_row: &MapDbcRow,
) -> R<PathBuf> {
    let wdt_file_name = get_wdt_file_name(&map_dbc_row);
    let path = join_path_ignoring_casing(map_folder, &wdt_file_name);

    if let Some(path) = path {
        Ok(path)
    } else {
        let msg = format!(
            "{} WDT not found",
            wdt_file_name
        );
        Err(msg.into())
    }
}

fn normalize_path(dep: &String) -> String {
    dep
        .replace(r"\", "/")
        .to_uppercase()
}

fn get_wdt_file_name(map_row: &MapDbcRow) -> String {
    let mut f_name = String::with_capacity(map_row.internal_name.len() + 4);
    f_name.push_str(map_row.internal_name.as_str());
    f_name.push_str(".wdt");
    f_name
}

fn get_wdl_file_name(map_row: &MapDbcRow) -> String {
    let mut f_name = String::with_capacity(map_row.internal_name.len() + 4);
    f_name.push_str(map_row.internal_name.as_str());
    f_name.push_str(".wdl");
    f_name
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



fn join_path_ignoring_casing(
    base: &Path,
    join: &String,
) -> Option<PathBuf> {
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

    Some(buf)
}