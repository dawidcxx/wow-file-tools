use crate::formats::chunk::{Chunk, ChunkVecUtils, ChunkMver, ChunkMphd};
use crate::common::R;


#[derive(Debug)]
pub struct WdtFile {
    pub mver: ChunkMver,
    pub mphd: ChunkMphd,
    // incomplete
}

impl WdtFile {
    pub fn from_path(path: &str) -> R<WdtFile> {
        let chunks = Chunk::from_path(path)?;
        Ok(WdtFile::new(chunks))
    }

    fn new(chunks: Vec<Chunk>) -> WdtFile {
        let mver = chunks.get_mver_chunk();
        let mphd = chunks.get_mphd_chunk();
        WdtFile {
            mver,
            mphd,
        }
    }
}
