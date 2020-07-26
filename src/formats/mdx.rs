use serde::{Deserialize, Serialize};
use crate::common::R;
use std::path::Path;
use crate::byte_utils::VecUtils;
use crate::formats::chunk::{Chunk, ChunkVecUtils};


#[derive(Debug, Serialize, Deserialize)]
pub struct MdxFile {
    pub texs: ChunkTexs,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ChunkTexs {
    pub texture_list: Vec<String>,
}

impl MdxFile {
    pub fn from_path<P: AsRef<Path>>(path: P) -> R<MdxFile> {
        let _chunks = Chunk::from_path(path)?;
        todo!("Mdx file handling is not implemented yet")
    }

    // fn from_bytes(chunks: Vec<Chunk>) -> R<MdxFile> {
    //     let texs = chunks.get_texs()?;
    //     Ok(MdxFile {
    //         texs,
    //     })
    // }
}


trait ChunkExt {
    fn get_texs(&self) -> R<ChunkTexs>;
}

impl ChunkExt for Vec<Chunk> {
    fn get_texs(&self) -> R<ChunkTexs> {
        ChunkTexs::from_chunk(self.get_chunk_of_type("TEXS"))
    }
}

impl ChunkTexs {
    fn from_chunk(chunk: &Chunk) -> R<ChunkTexs> {
        let strs = chunk.data.get_null_terminated_strings()?;
        println!("{:?}", strs);
        Ok(ChunkTexs {
            texture_list: vec![]
        })
    }
}