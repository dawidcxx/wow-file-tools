use serde::{Deserialize, Serialize};
use crate::common::R;
use std::path::{Path};
use crate::formats::chunk::{Chunk, ChunkVecUtils, ChunkMwmo};

#[derive(Debug, Serialize, Deserialize)]
pub struct WdlFile {
    pub mwmo: ChunkMwmo,
}

impl WdlFile {
    pub fn from_path<P: AsRef<Path>>(path: P) -> R<WdlFile> {
        let chunks = Chunk::from_path(path)?;
        let mwmo = chunks.get_mwmo();
        Ok(WdlFile {
            mwmo,
        })
    }
}

