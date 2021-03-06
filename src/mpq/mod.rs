pub mod mpq_path;

use crate::common::{err, R};
use crate::mpq::mpq_path::*;
use anyhow::Context;
use serde::{Deserialize, Serialize};
use std::io::Write;
use std::path::PathBuf;
use std::{fs::OpenOptions, time::Instant};
use stormlib::{MpqArchive, MpqFile};

#[derive(Debug, Serialize, Deserialize)]
pub struct ViewMpqResult(Vec<String>);

pub fn view_mpq(mpq_path: &String) -> R<ViewMpqResult> {
    let mpq_path = validated_mpq_path(mpq_path)?;
    let mut mpq = MpqArchive::from_path_readonly(mpq_path.to_str().unwrap())?;
    let file_list = mpq.get_file_list()?;
    Ok(ViewMpqResult(file_list))
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ExtractMpqFileResult {
    pub content: String,
}

pub fn extract_file_from_mpq(mpq_path: &String, mpq_file_name: &String) -> R<ExtractMpqFileResult> {
    let mpq_path = validated_mpq_path(mpq_path)?;
    let mut mpq = MpqArchive::from_path_readonly(mpq_path.to_str().unwrap())?;
    let file = retrieve_mpq_file(mpq_file_name, &mut mpq)?;
    let file_bytes = file.read_as_vec()?;
    Ok(ExtractMpqFileResult {
        content: hex::encode(file_bytes),
    })
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MpqExtractFileResult {
    pub written_to: String,
}

pub fn extract_file_from_mpq_to_path(
    mpq_path: &String,
    mpq_file_name: &String,
    target_path: &String,
) -> R<MpqExtractFileResult> {
    let mpq_path = validated_mpq_path(mpq_path)?;
    let target_path = {
        let path = PathBuf::from(target_path);
        // do some check on the path
        if path.is_dir() {
            if !path.exists() {
                return err(format!(
                    "Given target-path directory '{}' must exist",
                    path.display()
                ));
            }
        } else {
            let parent = path.parent().context(format!(
                "Given target-path '{}' must have a valid parent directory",
                path.display()
            ))?;
            if !parent.exists() {
                return err(format!(
                    "Given target-path '{}' must have a valid parent directory",
                    path.display()
                ));
            }
            if path.exists() {
                return err(format!(
                    "Given target-path '{}' already exist",
                    path.display()
                ));
            }
        }
        path
    };

    let mut mpq = MpqArchive::from_path_readonly(mpq_path.to_str().unwrap())?;
    let mpq_file = retrieve_mpq_file(mpq_file_name, &mut mpq)?;

    let dest_path = if target_path.is_dir() {
        let file_name = mpq_file.get_file_name()?;
        println!("{}", file_name);
        let contents = mpq_file.read_as_vec()?;
        let dest_path = target_path.join(file_name);
        let mut write_file = std::fs::File::create(&dest_path)?;
        write_file.write(contents.as_slice())?;
        dest_path
    } else {
        let dest_path = target_path;
        let mut write_file = std::fs::File::create(&dest_path)?;
        let contents = mpq_file.read_as_vec()?;
        write_file.write(contents.as_slice())?;
        dest_path
    };

    let result = MpqExtractFileResult {
        written_to: dest_path.to_str().unwrap().to_string(),
    };

    return Ok(result);
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MpqExtractTreeResult {
    pub extraced_files: Vec<PathBuf>,
}

pub fn extract_mpq_tree(
    mpq_path: &String,
    mpq_tree: &String,
    target: &String,
) -> R<MpqExtractTreeResult> {
    let mpq_path = validated_mpq_path(mpq_path)?;
    let mpq_path_str = mpq_path.to_str().unwrap();
    let target_path = PathBuf::from(target);

    if !target_path.exists() || !target_path.is_dir() {
        return err(format!(
            "Given target path '{}' must exist and be a directory",
            target_path.display()
        ));
    }

    let mpq_tree_path = MpqPath::from_string(mpq_tree).unwrap();
    let mut archive = MpqArchive::from_path_readonly(mpq_path_str)?;
    let file_list: Vec<MpqPath> = archive
        .get_file_list()
        .context("Failed to get the MPQ's (listfile)")?
        .into_iter()
        .filter_map(|it| MpqPath::from_string(&it))
        .collect();

    let matching_files = MpqPathUtil::matching(mpq_tree_path, file_list);

    create_directories(&matching_files, &target_path)?;

    let mut extraced_files = Vec::with_capacity(matching_files.len());

    for mpq_file_path in matching_files {
        let mpq_file = archive.get_file(mpq_file_path.to_string_path().as_str())?;
        let bytes = mpq_file.read_as_vec()?;
        let dest = target_path.join(mpq_file_path.to_relative_path_buf());
        let mut file = OpenOptions::new()
            .read(true)
            .write(true)
            .create(true)
            .open(dest.as_path())?;
        file.write_all(bytes.as_slice())?;
        extraced_files.push(dest);
    }

    return Ok(MpqExtractTreeResult { extraced_files });
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AddFileToMpqResult {
    pub elapsed_ms: f64,
}

pub fn add_file_to_mpq(
    mpq_path: &String,
    file_to_add: &String,
    mpq_save_path: &String,
) -> R<AddFileToMpqResult> {
    let start = Instant::now();
    let mpq_path = validated_mpq_path(mpq_path)?;
    let mut archive = MpqArchive::from_path(mpq_path.to_string_lossy().as_ref())?;

    let file_path_buf = PathBuf::from(file_to_add);
    if !file_path_buf.exists() {
        return err(format!(
            "File '{}' not found on the file system",
            file_to_add
        ));
    }
    if !file_path_buf.is_file() {
        return err(format!(
            "File '{}' must be a file, not a directory",
            file_to_add
        ));
    }
    if file_path_buf.extension().is_none() {
        return err(format!("File '{}' must have a extension", file_to_add));
    }

    let file_name = file_path_buf
        .file_name()
        .with_context(|| format!("Failed to extract file_name of: '{}'", file_to_add))?
        .to_string_lossy()
        .to_string();

    let mpq_save_path_dir = MpqPath::from_string(mpq_save_path)
        .with_context(|| format!("Invalid mpq path save location given '{}'", mpq_save_path))?;

    if !mpq_save_path_dir.is_dir() {
        return err(format!(
            "Mpq save path must be a directory '{}'",
            mpq_save_path_dir.to_string()
        ));
    }

    let save_at = mpq_save_path_dir.push(&file_name);

    archive
        .add_file(&file_path_buf, &save_at.to_string())
        .with_context(|| {
            format!(
                "Failed to add file '{}' to archieve '{}'",
                file_path_buf.display(),
                mpq_path.display()
            )
        })?;

    return Ok(AddFileToMpqResult {
        elapsed_ms: (start.elapsed().as_micros() as f64) / 1000.0,
    });
}

// internal utils

fn create_directories(matching_files: &Vec<MpqPath>, target_path: &PathBuf) -> R<()> {
    let directories: Vec<PathBuf> = matching_files
        .iter()
        .map(|path| {
            // makes sure we are only dealing with directories
            if path.is_file() {
                path.parent()
            } else {
                path.clone()
            }
        })
        // prepend the target path
        .map(|it| target_path.join(it.to_relative_path_buf()))
        .collect();
    for dir in directories {
        std::fs::create_dir_all(dir)?;
    }
    Ok(())
}

fn validated_mpq_path(mpq_path: &String) -> R<PathBuf> {
    let mpq_path = PathBuf::from(mpq_path);
    if !mpq_path.exists() {
        return err(format!("Could not find MPQ '{}'", mpq_path.display()));
    }
    extension_check(&mpq_path)?;
    Ok(mpq_path)
}

fn extension_check(mpq_path: &PathBuf) -> R<()> {
    let msg = format!("Must be a .mpq or .MPQ file");
    match mpq_path.extension() {
        None => {
            return err(msg);
        }
        Some(ext) => {
            let ext = ext.to_str().unwrap().to_uppercase();
            if ext != "MPQ" {
                return err(msg);
            }
        }
    };
    return Ok(());
}

fn retrieve_mpq_file<'a>(mpq_file_name: &String, mpq: &'a mut MpqArchive) -> R<&'a MpqFile> {
    let file = mpq
        .get_file(mpq_file_name.as_str())
        .with_context(|| format!("Failed to retrieve MPQ file, file: `{}`", mpq_file_name))?;
    Ok(file)
}
