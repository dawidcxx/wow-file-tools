use std::path::PathBuf;
use serde::{Serialize, Deserialize};
use crate::common::{R, err};
use stormlib::{MpqArchive, MpqFile};
use std::io::Write;

#[derive(Debug, Serialize, Deserialize)]
pub struct ViewMpqResult(Vec<String>);

pub fn view_mpq(
    mpq_path: &String,
) -> R<ViewMpqResult> {
    let mpq_path = validated_mpq_path(mpq_path)?;
    let mut mpq = MpqArchive::from_path(mpq_path.to_str().unwrap())?;
    let file_list = mpq.get_file_list()?;
    Ok(ViewMpqResult(file_list))
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ExtractMpqFileResult {
    pub content: String,
}

pub fn extract_file_from_mpq(
    mpq_path: &String,
    mpq_file_name: &String,
) -> R<ExtractMpqFileResult> {
    let mpq_path = validated_mpq_path(mpq_path)?;
    let mut mpq = MpqArchive::from_path(mpq_path.to_str().unwrap())?;
    let file = retrieve_mpq_file(mpq_file_name, &mut mpq)?;
    let file_bytes = file.read_as_vec()?;
    Ok(ExtractMpqFileResult {
        content: hex::encode(file_bytes)
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
                return err(format!("Given target-path directory '{}' must exist", path.display()));
            }
        } else {
            let parent = path
                .parent()
                .ok_or(format!("Given target-path '{}' must have a valid parent directory", path.display()))?;
            if !parent.exists() {
                return err(format!("Given target-path '{}' must have a valid parent directory", path.display()));
            }
            if path.exists() {
                return err(format!("Given target-path '{}' already exist", path.display()));
            }
        }
        path
    };

    let mut mpq = MpqArchive::from_path(mpq_path.to_str().unwrap())?;
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
        written_to: dest_path.to_str().unwrap().to_string()
    };

    return Ok(result);
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
    let file = mpq.get_file(mpq_file_name.as_str())
        .map_err(|e| format!("Failed to retrieve MPQ file, reason:  '{}' file: `{}`", e, mpq_file_name))?;
    Ok(file)
}