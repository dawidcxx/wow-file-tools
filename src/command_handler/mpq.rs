use crate::mpq::add_file_to_mpq;
use crate::mpq::extract_file_from_mpq;
use crate::mpq::extract_file_from_mpq_to_path;
use crate::mpq::extract_mpq_tree;
use crate::mpq::view_mpq;
use crate::{common::R, MpqToolCmd};

pub fn handle_mpq_command(cmd: &MpqToolCmd) -> R<Box<dyn erased_serde::Serialize>> {
    let res: Box<dyn erased_serde::Serialize> = match cmd {
        MpqToolCmd::View(view_cmd) => Box::new(view_mpq(&view_cmd.archive_path)?),
        MpqToolCmd::Extract(extract_cmd) => match &extract_cmd.target_path {
            Some(target_path) => {
                let result = extract_file_from_mpq_to_path(
                    &extract_cmd.archive_path,
                    &extract_cmd.archive_file_path,
                    target_path,
                )?;
                Box::new(result)
            }
            None => {
                let result = extract_file_from_mpq(
                    &extract_cmd.archive_path,
                    &extract_cmd.archive_file_path,
                )?;
                Box::new(result)
            }
        },
        MpqToolCmd::ExtractTree(cmd) => {
            Box::new(extract_mpq_tree(&cmd.archive_path, &cmd.tree, &cmd.dest)?)
        }
        MpqToolCmd::Pack(cmd) => {
            Box::new(add_file_to_mpq(&cmd.archive_path, &cmd.file, &cmd.dest)?)
        }
    };
    Ok(res)
}
