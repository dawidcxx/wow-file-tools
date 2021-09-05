#![feature(drain_filter)]

pub mod byte_utils;
mod command_handler;
pub mod common;
pub mod formats;
pub mod mpq;
pub mod proxy;

use crate::command_handler::dbc_join::handle_dbc_join;
use crate::command_handler::mpq::handle_mpq_command;
use crate::command_handler::resolve_map_assets::handle_resolve_map_assets;
use crate::command_handler::view::handle_view_command;

use crate::common::R;

use clap::Clap;
use command_handler::proxy::handle_proxy_command;
use serde::ser::SerializeStruct;
use serde::{Serialize, Serializer};

fn main() {
    let root_cmd = RootCmd::parse();
    let cmd_result = handle_cmd(root_cmd).map_err(SerializedError);

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
        Cmd::ResolveMapAssets(cmd) => handle_resolve_map_assets(cmd)?,
        Cmd::DbcJoin(cmd) => handle_dbc_join(cmd)?,
        Cmd::Mpq { cmd } => handle_mpq_command(cmd)?,
        Cmd::Proxy(cmd) => handle_proxy_command(&cmd.host, &cmd.username, &cmd.password)?,
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
    Proxy(ProxyCmd),
}

#[derive(Clap)]
#[clap(about = "A set of MPQ related tools")]
pub enum MpqToolCmd {
    View(MpqToolCmdView),
    Extract(MpqToolCmdExtract),
    ExtractTree(MpqToolCmdExtractTree),
    Pack(MpqToolCmdPack),
}

#[derive(Clap)]
#[clap(about = "Create a proxy server and inspect traffic")]
pub struct ProxyCmd {
    #[clap(short = 't', long = "host")]
    host: String,

    #[clap(short = 'u', long = "username")]
    username: String,

    #[clap(short = 'p', long = "password")]
    password: String,
}

#[derive(Clap)]
#[clap(about = "Get the list of files contained in this archive")]
pub struct MpqToolCmdView {
    #[clap(short = 'a', long = "archive")]
    archive_path: String,
}

#[derive(Clap)]
#[clap(about = "Adds a file to the given archive")]
pub struct MpqToolCmdPack {
    #[clap(short = 'a', long = "archive")]
    archive_path: String,

    #[clap(short = 'f', long = "file")]
    file: String,

    #[clap(short = 'd', long = "destination", about = "Path in the MPQ")]
    dest: String,
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

struct SerializedError(anyhow::Error);

impl Serialize for SerializedError {
    fn serialize<S>(&self, serializer: S) -> Result<<S as Serializer>::Ok, <S as Serializer>::Error>
    where
        S: Serializer,
    {
        // 3 is the number of fields in the struct.
        let mut state = serializer.serialize_struct("ProgramErr", 2)?;
        let chain: Vec<String> = self.0.chain().map(|e| e.to_string()).collect();

        state.serialize_field("error", &self.0.to_string())?;
        state.serialize_field("chain", &chain)?;

        state.end()
    }
}
