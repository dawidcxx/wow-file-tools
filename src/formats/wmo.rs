use crate::common::R;
use crate::formats::chunk::{Chunk, ChunkMver, ChunkVecUtils, ChunkMotx, ChunkMogn, ChunkModn, ChunkMohd};
use serde::{Serialize, Deserialize};
use std::collections::HashSet;
use std::iter::FromIterator;

const ROOT_FILE_CHUNKS: &[&str] = &["MOMT", "MOGI", "MOSB", "MOVV", "MODN"];
const GROUP_FILE_CHUNKS: &[&str] = &["MOGP", "MOPY", "MOVI", "MONR", "MOTV"];

#[derive(Debug, Serialize, Deserialize)]
pub enum WmoFile {
    ROOT(WmoRootFile),
    GROUP(WmoGroupFile),
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WmoRootFile {
    pub mver: ChunkMver,
    pub motx: ChunkMotx,
    pub mohd: ChunkMohd,
    pub momt: (),
    pub mogn: ChunkMogn,
    pub mogi: (),
    pub mosb: (),
    pub mopv: (),
    pub mopt: (),
    pub mopr: (),
    pub movv: (),
    pub movb: (),
    pub molt: (),
    pub mods: (),
    pub modn: ChunkModn,
    pub modd: (),
    pub mfog: (),
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WmoGroupFile {
    pub mver: ChunkMver,
    pub mogp: (),
    pub mopy: (),
    pub movi: (),
    pub movt: (),
    pub monr: (),
    pub motv: (),
    pub moba: (),
    pub molr: Option<()>,
    pub modr: Option<()>,
    pub mobn: Option<()>,
    pub mobr: Option<()>,
    pub mpbv: Option<()>,
    pub mpbp: Option<()>,
    pub mpbi: Option<()>,
    pub mpbg: Option<()>,
    pub mocv: Option<()>,
    pub mliq: Option<()>,
    pub mori: Option<()>,
    pub morb: Option<()>,
}

impl WmoFile {
    pub fn from_path(path: &str) -> R<WmoFile> {
        let chunks = Chunk::from_path(path)?;
        Ok(WmoFile::new(chunks))
    }

    fn new(chunks: Vec<Chunk>) -> WmoFile {
        fn matches_file_type(marker_chunks: &[&str], lookup: &HashSet<String>) -> bool {
            marker_chunks
                .iter()
                .all(|c| {
                    lookup.contains(*c)
                })
        }

        let chunk_names_lookup: HashSet<String> = chunks.iter()
            .map(|v| v.get_id_as_string())
            .collect();

        if matches_file_type(ROOT_FILE_CHUNKS, &chunk_names_lookup) {
            WmoFile::ROOT(WmoRootFile::new(chunks))
        } else if matches_file_type(GROUP_FILE_CHUNKS, &chunk_names_lookup) {
            WmoFile::GROUP(WmoGroupFile::new(chunks))
        } else {
            panic!("WmoFile#new: Cannot create a root or group WMO from given chunks!")
        }
    }
}

impl WmoRootFile {
    fn new(chunks: Vec<Chunk>) -> WmoRootFile {
        let mver = chunks.get_mver_chunk();
        let motx = chunks.get_motx();
        let mohd = chunks.get_mohd();
        let mogn = chunks.get_mogn();
        let modn = chunks.get_modn();

        WmoRootFile {
            mver,
            motx,
            mohd,
            momt: (),
            mogn,
            mogi: (),
            mosb: (),
            mopv: (),
            mopt: (),
            mopr: (),
            movv: (),
            movb: (),
            molt: (),
            mods: (),
            modn,
            modd: (),
            mfog: (),
        }
    }
}

impl WmoGroupFile {
    fn new(chunks: Vec<Chunk>) -> WmoGroupFile {
        let mver = chunks.get_mver_chunk();
        WmoGroupFile {
            mver,
            mogp: (),
            mopy: (),
            movi: (),
            movt: (),
            monr: (),
            motv: (),
            moba: (),
            molr: None,
            modr: None,
            mobn: None,
            mobr: None,
            mpbv: None,
            mpbp: None,
            mpbi: None,
            mpbg: None,
            mocv: None,
            mliq: None,
            mori: None,
            morb: None,
        }
    }
}