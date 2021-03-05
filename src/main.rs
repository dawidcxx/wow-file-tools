#![feature(backtrace)]
#![feature(fn_traits)]
#![feature(drain_filter)]

pub mod byte_utils;
pub mod common;
pub mod formats;
pub mod mpq;
mod resolve_map_assets;
mod view_command;

use crate::common::R;

use crate::formats::dbc::join::spell::get_spells_join;
use crate::formats::dbc::join::talents::get_talents_join;
use crate::mpq::{
    extract_file_from_mpq, extract_file_from_mpq_to_path, extract_mpq_tree, view_mpq,
};
use crate::resolve_map_assets::ResolveMapAssetsCmdResult;
use crate::view_command::handle_view_command;
use clap::Clap;
use serde::ser::SerializeStruct;
use serde::{Serialize, Serializer};
use std::error::Error;
use std::path::Path;

fn main() {
    let root_cmd = RootCmd::parse();
    let cmd_result = handle_cmd(root_cmd).map_err(|error| ProgramErr { error });

    std::process::exit(match cmd_result {
        Err(e) => {
            let json = serde_json::to_string_pretty(&e).unwrap();
            eprintln!("{}", json);
            727
        }
        _ => 0,
    })
}

fn handle_cmd(root_cmd: RootCmd) -> R<()> {
    let mut result: Box<dyn erased_serde::Serialize> = match &root_cmd.cmd {
        Cmd::View(v) => handle_view_command(v)?,
        Cmd::ResolveMapAssets(cmd) => Box::new(handle_resolve_map_assets_cmd(cmd)?),
        Cmd::DbcJoin(cmd) => match cmd.join {
            AggregateViewCmdChoice::SPELLS => {
                Box::new(get_spells_join(&cmd.dbc_folder, &cmd.record_id)?)
            }
            AggregateViewCmdChoice::TALENTS => {
                Box::new(get_talents_join(&cmd.dbc_folder, &cmd.record_id)?)
            }
        },
        Cmd::Mpq { cmd } => match cmd {
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
        },
    };

    if root_cmd.no_result {
        result = Box::new("")
    }

    if root_cmd.compact {
        serde_json::to_writer(std::io::stdout(), &result)?;
    } else {
        serde_json::to_writer_pretty(std::io::stdout(), &result)?;
    }

    Ok(())
}

fn handle_resolve_map_assets_cmd(cmd: &ResolveMapAssetsCmd) -> R<ResolveMapAssetsCmdResult> {
    resolve_map_assets::resolve_map_assets(
        Path::new(cmd.workspace.as_str()),
        &cmd.map_id,
        cmd.prune_unused,
    )
}

#[derive(Clap)]
#[clap(version = "1.0", author = "ArenaCraft")]
pub struct RootCmd {
    #[clap(
        short = 'c',
        long = "compact",
        about = "Output JSON will no longer be pretty printed"
    )]
    compact: bool,

    #[clap(
        long = "no-result",
        about = "Don't output any result, useful for testing"
    )]
    no_result: bool,

    #[clap(subcommand)]
    cmd: Cmd,
}

#[derive(Clap)]
pub enum Cmd {
    View(ViewCmd),
    ResolveMapAssets(ResolveMapAssetsCmd),
    DbcJoin(DbcJoinCmd),
    Mpq {
        #[clap(subcommand)]
        cmd: MpqToolCmd,
    },
}

#[derive(Clap)]
#[clap(about = "A set of MPQ related tools")]
pub enum MpqToolCmd {
    View(MpqToolCmdView),
    Extract(MpqToolCmdExtract),
    ExtractTree(MpqToolCmdExtractTree),
}

#[derive(Clap)]
#[clap(about = "Get the list of files contained in this archive")]
pub struct MpqToolCmdView {
    #[clap(short = 'a', long = "archive")]
    archive_path: String,
}

#[derive(Clap)]
#[clap(
    about = "Extract a single file from the archive, default prints it to std-out as a hex encoded json string"
)]
pub struct MpqToolCmdExtract {
    #[clap(short = 'a', long = "archive")]
    archive_path: String,

    #[clap(
        short = 'f',
        long = "file",
        about = "The archive path of the file to retrieve"
    )]
    archive_file_path: String,

    #[clap(
        short = 't',
        long = "target",
        about = "Create this file and write retrieved contents to it"
    )]
    target_path: Option<String>,
}

#[derive(Clap)]
#[clap(about = "Get the list of files contained in this archive")]
pub struct MpqToolCmdExtractTree {
    #[clap(short = 'a', long = "archive")]
    pub archive_path: String,

    #[clap(short = 't', long = "tree")]
    pub tree: String,

    #[clap(short = 'd', long = "dest")]
    pub dest: String,
}

#[derive(Clap)]
#[clap(about = "View given file as JSON")]
pub struct ViewCmd {
    #[clap(short = 'f', long = "file")]
    file: String,
}

#[derive(Clap)]
#[clap(about = "Resolve all map dependencies")]
pub struct ResolveMapAssetsCmd {
    #[clap(short = 'w', long = "workspace")]
    workspace: String,

    #[clap(short = 'm', long = "map-ids")]
    map_id: Vec<u32>,

    #[clap(
        short = 'p',
        long = "prune-unused",
        about = "Remove unneeded files within the workspace"
    )]
    prune_unused: bool,
}

#[derive(Clap)]
#[clap(about = "Show a joined view of multiple dbc files")]
pub struct DbcJoinCmd {
    #[clap(short = 'd', long = "dbc-folder")]
    dbc_folder: String,

    #[clap(
        short = 'j',
        long = "join-name",
        about = "join to display, one of: SPELLS, TALENTS"
    )]
    join: AggregateViewCmdChoice,

    #[clap(short = 'r', long = "record-id")]
    record_id: Option<u32>,
}

enum AggregateViewCmdChoice {
    SPELLS,
    TALENTS,
}

impl std::str::FromStr for AggregateViewCmdChoice {
    type Err = &'static str;
    fn from_str(s: &str) -> Result<Self, &'static str> {
        match s.to_uppercase().as_str() {
            "SPELLS" => Ok(Self::SPELLS),
            "TALENTS" => Ok(Self::TALENTS),
            _ => Err("Must be one of ( SPELLS, TALENTS )\n"),
        }
    }
}

struct ProgramErr {
    error: Box<dyn Error>,
}

impl Serialize for ProgramErr {
    fn serialize<S>(&self, serializer: S) -> Result<<S as Serializer>::Ok, <S as Serializer>::Error>
    where
        S: Serializer,
    {
        // 3 is the number of fields in the struct.
        let mut state = serializer.serialize_struct("ProgramErr", 3)?;
        state.serialize_field("error", &self.error.to_string())?;
        state.serialize_field(
            "backtrace",
            &self.error.backtrace().map(|b| format!("{:?}", b)),
        )?;
        state.end()
    }
}
