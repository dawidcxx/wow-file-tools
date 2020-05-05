#![feature(backtrace)]
#![feature(impl_trait_in_bindings)]

pub mod byte_utils;
pub mod formats;
pub mod common;

use clap::Clap;
use crate::common::R;
use std::path::Path;
use serde::{Serialize, Serializer};
use std::error::Error;
use serde::ser::SerializeStruct;
use crate::formats::adt::AdtFile;
use crate::formats::wmo::{WmoFileVariant, WmoFile};

fn main() {
    let root_cmd = RootCmd::parse();
    let cmd_result = handle_cmd(root_cmd)
        .map_err(|error| ProgramErr { error });

    std::process::exit(match cmd_result {
        Err(e) => {
            let json = serde_json::to_string_pretty(&e).unwrap();
            eprintln!("{}", json);
            727
        }
        _ => 0
    })
}

fn handle_cmd(root_cmd: RootCmd) -> R<()> {
    return match root_cmd.cmd {
        Cmd::View(v) => {
            let file_path = Path::new(&v.file);
            let file_path_str = file_path.to_str().unwrap();

            if !file_path.exists() {
                return Err("This file does not exist".into());
            }

            let extension = match file_path.extension() {
                None => {
                    return Err("Given file has no extension".into());
                }
                Some(v) => {
                    v.to_str().unwrap()
                }
            };

            let view_result = get_view_result(&v, file_path_str, extension)?;


            println!("{}", view_result);

            Ok(())
        }
    };
}

fn get_view_result(
    view_cmd: &ViewCmd,
    file_path_str: &str,
    extension: &str,
) -> R<String> {
    let result = match extension {
        "wmo" => {
            let f = WmoFile::from_path(file_path_str)?;
            serialize_result(view_cmd, f)?
        }
        "adt" => {
            let f = AdtFile::from_path(file_path_str)?;
            serialize_result(view_cmd, f)?
        }
        _ => {
            let err_msg = format!("Unsupported file extension: ({})", extension);
            return Err(err_msg.into());
        }
    };

    Ok(result)
}

fn serialize_result(view_cmd: &ViewCmd, result: impl Serialize) -> R<String> {
    let output_str = if view_cmd.compact {
        serde_json::to_string(&result)?
    } else {
        serde_json::to_string_pretty(&result)?
    };
    Ok(output_str)
}

#[derive(Clap)]
#[clap(version = "1.0", author = "ArenaCraft")]
struct RootCmd {
    #[clap(subcommand)]
    cmd: Cmd,
}

#[derive(Clap)]
enum Cmd {
    View(ViewCmd)
}

#[derive(Clap)]
struct ViewCmd {
    #[clap(short = "f", long = "file")]
    file: String,

    #[clap(short = "c", long = "compact", help = "Output JSON will no longer be pretty printed")]
    compact: bool,
}

struct ProgramErr {
    error: Box<dyn Error>,
}

impl Serialize for ProgramErr {
    fn serialize<S>(&self, serializer: S) -> Result<<S as Serializer>::Ok, <S as Serializer>::Error> where
        S: Serializer {
        // 3 is the number of fields in the struct.
        let mut state = serializer.serialize_struct("ProgramErr", 3)?;
        state.serialize_field("error", &self.error.to_string())?;
        state.serialize_field(
            "backtrace",
            &self.error
                .backtrace()
                .map(|b| format!("{:?}", b)),
        )?;
        state.end()
    }
}