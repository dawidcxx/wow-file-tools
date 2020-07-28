use crate::formats::chunk::{Chunk, ChunkVecUtils, ChunkMver, ChunkMphd, ChunkMain, ChunkMwmo, ChunkModf};
use crate::common::R;
use serde::{Serialize, Deserialize};
use std::path::Path;

#[derive(Debug, Serialize, Deserialize)]
pub struct WdtFile {
    pub mver: ChunkMver,
    pub mphd: ChunkMphd,
    pub main: ChunkMain,
    pub mwmo: ChunkMwmo,
    pub modf: Option<ChunkModf>,
}

impl WdtFile {
    pub fn from_path<P: AsRef<Path>>(path: P) -> R<WdtFile> {
        let chunks = Chunk::from_path(path)?;
        WdtFile::new(chunks)
    }

    fn new(chunks: Vec<Chunk>) -> R<WdtFile> {
        let mver = chunks.get_mver_chunk()?;
        let mphd = chunks.get_mphd_chunk();
        let main = chunks.get_main();
        let mwmo = chunks.get_mwmo();
        let modf = chunks.get_modf();
        Ok(WdtFile {
            mver,
            mphd,
            main,
            mwmo,
            modf,
        })
    }
}
