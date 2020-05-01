use crate::formats::chunk::*;
use serde::{Deserialize, Serialize};
use crate::common::R;

#[derive(Debug, Serialize, Deserialize)]
pub struct AdtFile {
    pub mver: ChunkMver,
    pub mhdr: ChunkMhdr,
    pub mcin: Vec<ChunkMcin>,
    pub mtex: ChunkMtex,
    pub mmdx: ChunkMmdx,
    pub mmid: ChunkMmid,
    pub mwmo: ChunkMwmo,
    pub mwid: ChunkMwid,
    pub mddf: ChunkMddf,
}

impl AdtFile {
    pub fn from_path(path: &str) -> R<AdtFile> {
        let chunks = Chunk::from_path(path)?;
        Ok(AdtFile::new(chunks))
    }

    fn new(chunks: Vec<Chunk>) -> AdtFile {
        let mver = chunks.get_mver_chunk();
        let mhdr = chunks.get_mhdr();
        let mcin = chunks.get_mcin();
        let mtex = chunks.get_mtex();
        let mmdx = chunks.get_mmdx();
        let mmid = chunks.get_mmid();
        let mwmo = chunks.get_mwmo();
        let mwid = chunks.get_mwid();
        let mddf = chunks.get_mddf();
        AdtFile {
            mver,
            mhdr,
            mcin,
            mtex,
            mmdx,
            mmid,
            mwmo,
            mwid,
            mddf,
        }
    }
}