use crate::{ResolveMapAssetsCmd, common::{err, R}};
use crate::formats::adt::AdtFile;
use crate::formats::dbc::dbc::{load_loading_screens_dbc_from_path, load_map_dbc_from_path};
use crate::formats::dbc::map::MapDbcRow;
use crate::formats::m2::M2File;
use crate::formats::mdx::MdxFile;
use crate::formats::wdl::WdlFile;
use crate::formats::wmo::WmoFile;
use crate::resolve_map_assets::ResolveMapAssetsCmdWarn::{AdtParseErr, Missing, MissingDbcEntry};
use anyhow::Context;
use serde::{Deserialize, Serialize};
use std::collections::HashSet;
use std::fs;
use std::fs::{read_dir, File};
use std::io::{BufRead, BufReader};
use std::iter::FromIterator;
use std::path::{Path, PathBuf};
use walkdir::{DirEntry, WalkDir};

#[derive(Debug, Serialize, Deserialize)]
pub struct ResolveMapAssetsCmdResult {
    pub warns: Vec<ResolveMapAssetsCmdWarn>,
    pub results: HashSet<PathBuf>,
    pub misc: ResolveMapAssetsCmdResultMisc,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum ResolveMapAssetsCmdWarn {
    Missing(String),
    FileParseFail(String),
    FailedToRemoveFile(String),
    AdtParseErr(PathBuf),
    MissingDbcEntry(String),
    MissingMiniMapFolder,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ResolveMapAssetsCmdResultMisc {
    pub mcnk_area_id_entries: HashSet<ResolveMapAssetsAreaIdEntry>,
}

#[derive(Debug, Serialize, Deserialize, Hash, Eq, PartialEq)]
pub struct ResolveMapAssetsAreaIdEntry {
    pub area_id: u32,
    pub map_id: u32,
    pub map_name: String,
}

pub fn handle_resolve_map_assets_cmd(cmd: &ResolveMapAssetsCmd) -> R<ResolveMapAssetsCmdResult> {
    resolve_map_assets(
        Path::new(cmd.workspace.as_str()),
        &cmd.map_id,
        cmd.prune_unused,
    )
}


fn resolve_map_assets(
    workspace_path: &Path,
    map_ids: &Vec<u32>,
    should_prune_workspace: bool,
) -> R<ResolveMapAssetsCmdResult> {
    if !workspace_path.exists() {
        return err(format!(
            "Error: workspace '{:?}' not found on the file system.",
            workspace_path
        ));
    }

    let mut results_builder = Vec::new();
    let mut warns: Vec<ResolveMapAssetsCmdWarn> = Vec::new();
    let mut mcnk_area_id_entries = HashSet::new();

    let map_dbc_loc = join_path_ignoring_casing(workspace_path, "DBFilesClient/Map.dbc")
        .context("Missing Map.dbc file")?;

    let map_dbc = load_map_dbc_from_path(map_dbc_loc.str())?;

    results_builder.push(map_dbc_loc);

    for map_id in map_ids {
        let map_row = map_dbc
            .rows
            .iter()
            .find(|map| map.id == *map_id)
            .context(format!("Map with id {} not found", map_id))?;

        let maps_folder = join_path_ignoring_casing(
            workspace_path,
            format!("World/Maps/{}", map_row.internal_name).as_str(),
        )
        .context("Missing World/Maps folder in workspace")?;

        // to(maybe)do: these could be a warning.
        let wdt_file_path = get_wdt_path(&maps_folder, map_row).context("Missing Map WDT file")?;
        let wdl_file_path = get_wdl_path(&maps_folder, map_row).context("Missing Map WDL file")?;

        let wdl = WdlFile::from_path(&wdl_file_path)?;

        add_wow_dep(workspace_path, wdl.mwmo.0, &mut results_builder, &mut warns);

        results_builder.push(wdl_file_path);
        results_builder.push(wdt_file_path);

        for adt_entry in find_files_by_extension(maps_folder, 2, ".adt") {
            let adt_path = adt_entry.into_path();
            results_builder.push(adt_path.clone());

            let adt = AdtFile::from_path(adt_path.clone());

            if adt.is_err() {
                warns.push(AdtParseErr(adt_path.clone()));
                break;
            }

            let adt = adt.unwrap();

            for mcnk in &adt.mcnk.items {
                mcnk_area_id_entries.insert(ResolveMapAssetsAreaIdEntry {
                    area_id: mcnk.area_id,
                    map_id: map_id.clone(),
                    map_name: map_row.internal_name.clone(),
                });
            }

            add_wow_dep(workspace_path, adt.mtex.0, &mut results_builder, &mut warns);

            let added_wmos =
                add_wow_dep(workspace_path, adt.mwmo.0, &mut results_builder, &mut warns);

            add_m2_type_wow_dep(workspace_path, adt.mmdx.0, &mut results_builder, &mut warns);

            for wmo_path in added_wmos {
                let wmo = WmoFile::from_path(wmo_path.str())?;
                add_wow_dep(
                    workspace_path,
                    wmo.root.motx.0,
                    &mut results_builder,
                    &mut warns,
                );
                add_m2_type_wow_dep(
                    workspace_path,
                    wmo.root.modn.0,
                    &mut results_builder,
                    &mut warns,
                );

                results_builder.append(wmo.loaded_group_files.clone().as_mut())
            }
        }

        find_and_add_tileset_blps(&mut results_builder);

        find_and_add_minimap_blps(&workspace_path, map_row, &mut results_builder, &mut warns);

        find_and_add_loading_screen_blp(workspace_path, &map_row, &mut results_builder, &mut warns);
    }

    let results: HashSet<PathBuf> = HashSet::from_iter(
        results_builder
            .iter()
            .map(|it| fs::canonicalize(it).expect("Invalid path encountered")),
    );

    if should_prune_workspace {
        prune_workspace(workspace_path, &results, &mut warns);
    }

    Ok(ResolveMapAssetsCmdResult {
        warns,
        results,
        misc: ResolveMapAssetsCmdResultMisc {
            mcnk_area_id_entries,
        },
    })
}

fn find_and_add_minimap_blps(
    workspace_root: &Path,
    map_ref: &MapDbcRow,
    results: &mut Vec<PathBuf>,
    warns: &mut Vec<ResolveMapAssetsCmdWarn>,
) {
    let mini_map_folder = match vec!["TILESET/Textures/Minimap", "Textures/Minimap"]
        .iter()
        .filter_map(|it| join_path_ignoring_casing(workspace_root, it))
        .nth(0)
    {
        None => {
            warns.push(ResolveMapAssetsCmdWarn::MissingMiniMapFolder);
            return;
        }
        Some(it) => it,
    };

    let md5_translate_file =
        match join_path_ignoring_casing(mini_map_folder.as_ref(), "md5translate.trs") {
            None => {
                warns.push(ResolveMapAssetsCmdWarn::Missing(
                    "TILESET/Textures/Minimap/md5translate.trs".to_string(),
                ));
                return;
            }
            Some(f) => f,
        };

    let md5_translate_file = match File::open(md5_translate_file) {
        Ok(f) => f,
        Err(err) => {
            warns.push(ResolveMapAssetsCmdWarn::FileParseFail(format!(
                "Failed to parse 'md5translate.trs' reason: {}",
                err
            )));
            return;
        }
    };

    for line in BufReader::new(md5_translate_file)
        .lines()
        .filter_map(|it| it.ok())
    {
        if line
            .to_uppercase()
            .starts_with(&map_ref.internal_name.to_uppercase())
        {
            let blp = match line.split("\t").last() {
                None => {
                    warns.push(ResolveMapAssetsCmdWarn::FileParseFail(format!(
                        "'md5translate.trs' failed to parse line: {} ",
                        line
                    )));
                    continue;
                }
                Some(l) => l,
            };

            if let Some(blp_path) = join_path_ignoring_casing(mini_map_folder.as_ref(), blp) {
                results.push(blp_path);
            } else {
                warns.push(Missing(format!("TILESET/Textures/Minimap/{}", blp)))
            }
        }
    }
}

fn find_and_add_tileset_blps(results: &mut Vec<PathBuf>) {
    fn try_find_blp_s_dep(blp_root: &Path, file_name: &str) -> Option<PathBuf> {
        let lower_case = blp_root.join(format!("{}_s.blp", file_name));
        if lower_case.exists() {
            return Some(lower_case);
        }
        let upper_case = blp_root.join(format!("{}_s.BLP", file_name));
        if upper_case.exists() {
            return Some(upper_case);
        }
        return None;
    }

    let mut builder = Vec::new();

    for blp_dependency in results
        .iter()
        .filter_map(|dependency| dependency.extension().map(|ext| (dependency, ext)))
        .filter(|(_, ext)| (*ext).eq("blp") || (*ext).eq("BLP"))
        .map(|(dependency, _)| dependency)
    {
        let blp_dependency_str = blp_dependency.str();

        if !blp_dependency_str.to_uppercase().contains("TILESET") {
            // Only TILESET's exhibit this weird behavior.
            continue;
        }

        let blp_root = blp_dependency
            .parent()
            .expect("Expected BLP file to have a parent");
        let file_name = blp_dependency
            .file_name()
            .expect("Expected BLP file to have a filename")
            .to_str()
            .expect("BLP filename parse error")
            .split(".")
            .nth(0)
            .expect("BLP filename parse error");

        if let Some(blp_s) = try_find_blp_s_dep(blp_root, file_name) {
            builder.push(blp_s);
        }
    }
    results.append(&mut builder);
}

fn prune_workspace(
    workspace_root: &Path,
    dependencies: &HashSet<PathBuf>,
    warns: &mut Vec<ResolveMapAssetsCmdWarn>,
) {
    for entry in WalkDir::new(workspace_root)
        .into_iter()
        .filter_map(|e| e.ok())
    {
        if entry.metadata().unwrap().is_dir() {
            // ignore dirs
            continue;
        }

        let workspace_file = fs::canonicalize(entry.path()).expect("Invalid path encountered");

        if !dependencies.contains(&workspace_file) {
            // println!("To be trashed: {:?}", workspace_file);
            if let Err(e) = fs::remove_file(&workspace_file) {
                let msg = format!(
                    "Failed to delete '{}' reason: '{}'",
                    workspace_file.str(),
                    e
                );
                warns.push(ResolveMapAssetsCmdWarn::FailedToRemoveFile(msg));
            }
        }
    }
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
            warns.push(Missing(dependency));
        }
    }
    added
}

fn add_m2_dependencies(m2_root: &Path, file_name: &str, results: &mut Vec<PathBuf>) {
    let mut skin_it = 0;
    loop {
        let skin_file = m2_root.join(format!("{}0{}.skin", file_name, skin_it));
        skin_it += 1;
        if skin_file.exists() {
            results.push(skin_file);
        } else {
            break;
        }
    }
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
        workspace_root: &Path,
        path: PathBuf,
        added: &mut Vec<PathBuf>,
        results: &mut Vec<PathBuf>,
        warns: &mut Vec<ResolveMapAssetsCmdWarn>,
    ) {
        added.push(path.clone());
        results.push(path.clone());

        if let Some(ext) = path.extension() {
            if ext.eq("m2") || ext.eq("M2") {
                // handle m2's.
                if let Ok(m2_file) = M2File::from_path(path.clone()) {
                    add_m2_type_wow_dep(workspace_root, m2_file.textures, results, warns);
                    let file_stem = path
                        .file_stem()
                        .unwrap()
                        .to_str()
                        .expect("Failed to parse filename of a M2");
                    let m2_root = path.parent().unwrap();
                    add_m2_dependencies(m2_root, file_stem, results);
                } else {
                    let msg = format!("Failed to parse m2 '{}'", path.str());
                    warns.push(ResolveMapAssetsCmdWarn::FileParseFail(msg));
                }
            } else if ext.eq("mdx") || ext.eq("MDX") {
                if let Ok(mdx_file) = MdxFile::from_path(path.clone()) {
                    add_m2_type_wow_dep(workspace_root, mdx_file.texs.texture_list, results, warns);
                } else {
                    let msg = format!("Failed to parse m2 '{}'", path.str());
                    warns.push(ResolveMapAssetsCmdWarn::FileParseFail(msg));
                }
            }
        }
    }

    for dependency in wow_dependency_list {
        if let Some(path) = join_path_ignoring_casing(workspace_root, dependency.as_str()) {
            on_found(workspace_root, path, &mut added, results, warns);
        } else {
            let mut found = false;

            if dependency.ends_with("mdx") || dependency.ends_with("MDX") {
                // retry, replacing extension to .m2
                let dependency = dependency.replace(".mdx", ".m2").replace(".MDX", ".M2");
                if let Some(path) = join_path_ignoring_casing(workspace_root, dependency.as_str()) {
                    found = true;
                    on_found(workspace_root, path, &mut added, results, warns);
                }
            } else if dependency.ends_with("m2") || dependency.ends_with("M2") {
                // retry, replacing extension to .mdx
                let dependency = dependency.replace(".M2", ".MDX").replace(".m2", ".mdx");
                if let Some(path) = join_path_ignoring_casing(workspace_root, dependency.as_str()) {
                    found = true;
                    on_found(workspace_root, path, &mut added, results, warns);
                }
            }

            if !found {
                warns.push(Missing(dependency));
            }
        }
    }

    added
}

fn find_and_add_loading_screen_blp(
    workspace_path: &Path,
    map_dbc: &MapDbcRow,
    mut results: &mut Vec<PathBuf>,
    mut warns: &mut Vec<ResolveMapAssetsCmdWarn>,
) {
    if let Some(loading_screen_dbc_path) =
        join_path_ignoring_casing(workspace_path, "DBFilesClient/LoadingScreens.dbc")
    {
        let loading_screens_dbc = load_loading_screens_dbc_from_path(loading_screen_dbc_path.str())
            .expect("LoadingScreens.dbc parse error");
        if let Some(loading_screen_dbc_row) = loading_screens_dbc
            .rows
            .iter()
            .find(|row| row.id == map_dbc.loading_screen_ref_id)
        {
            add_wow_dep(
                workspace_path,
                vec![loading_screen_dbc_row.path.clone()],
                &mut results,
                &mut warns,
            );
        } else {
            warns.push(MissingDbcEntry(format!(
                "DBFilesClient/LoadingScreens.dbc  map_dbc.loading_screen_ref_id = {}",
                map_dbc.loading_screen_ref_id
            )))
        }
    } else {
        warns.push(Missing("DBFilesClient/LoadingScreens.dbc".to_string()));
    }
}

fn join_path_ignoring_casing(base: &Path, join: &str) -> Option<PathBuf> {
    let parts: Vec<&str> = join.split(&['/', '\\'][..]).collect();
    let mut buf = PathBuf::new();
    buf.push(base.clone());

    for part in parts {
        let part = part.to_uppercase();
        if let Ok(read_dir) = read_dir(&buf) {
            let next = read_dir.filter_map(|it| it.ok()).find(|dir_entry| {
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
            panic!(
                "#join_path_ignoring_casing should not arrive at a invalid path: {:?}",
                buf
            );
        }
    }

    Some(buf)
}

trait PathBufUtils {
    fn str(&self) -> &str;
}

impl PathBufUtils for PathBuf {
    fn str(&self) -> &str {
        self.to_str()
            .expect("Failed to convert PathBuf -> &str. Invalid FileSystem path characters?")
    }
}

fn find_files_by_extension<P: AsRef<Path>>(
    path: P,
    depth: usize,
    extension: &str,
) -> Vec<DirEntry> {
    let extension = extension.to_string().to_uppercase();
    find_file_by_predicate(
        path,
        depth,
        Box::new(move |entry| {
            let curr_file_name = entry.file_name().to_str().unwrap().to_uppercase();
            curr_file_name.ends_with(&extension)
        }),
    )
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
        if metadata.is_err() {
            continue;
        }
        let metadata = metadata.unwrap();
        if metadata.is_dir() {
            continue;
        }
        if predicate.call((&entry,)) {
            res.push(entry);
        }
    }
    res
}

fn get_wdt_path(maps_folder: &PathBuf, map_row: &MapDbcRow) -> Option<PathBuf> {
    join_path_ignoring_casing(
        maps_folder,
        format!("{0}.wdt", map_row.internal_name).as_str(),
    )
}

fn get_wdl_path(maps_folder: &PathBuf, map_row: &MapDbcRow) -> Option<PathBuf> {
    join_path_ignoring_casing(
        maps_folder,
        format!("{0}.wdl", map_row.internal_name).as_str(),
    )
}
