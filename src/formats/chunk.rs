use std::str::from_utf8;
use std::fs::File;
use std::io::Read;
use crate::byte_utils::*;
use std::convert::TryInto;
use serde::{Serialize, Deserialize};
use crate::common::R;

#[derive(Debug, Serialize, Deserialize)]
pub struct Chunk {
    pub id: [u8; 4],
    pub size: u32,
    pub data: Vec<u8>,
}

impl Chunk {
    pub fn get_id_as_string(&self) -> String { from_utf8(&self.id).unwrap().chars().rev().collect() }

    pub fn from_path(path: &str) -> R<Vec<Chunk>> {
        let mut f = File::open(path)?;
        let file_size = f.metadata()?.len() as usize;
        let mut buffered_file = Vec::with_capacity(file_size);
        f.read_to_end(&mut buffered_file)?;

        let mut offset: usize = 0;
        let mut builder: Vec<Chunk> = Vec::new();

        loop {
            let id: [u8; 4] = buffered_file[offset..offset + 4].try_into()?;
            let size = buffered_file[offset + 4..offset + 8].to_vec().get_u32(0)? as usize;
            let data = buffered_file[offset + 8..offset + 8 + size].to_vec();

            offset += 8 + size;
            builder.push(Chunk {
                id,
                size: size as u32,
                data,
            });

            if offset == buffered_file.len() {
                break;
            }
        }

        Ok(builder)
    }
}

pub trait ChunkVecUtils {
    fn get_chunk_of_type(&self, chunk_type: &str) -> &Chunk;
    fn get_mver_chunk(&self) -> ChunkMver;
    fn get_mphd_chunk(&self) -> ChunkMphd;
    fn get_mhdr(&self) -> ChunkMhdr;
    fn get_mcin(&self) -> Vec<ChunkMcin>;
    fn get_mtex(&self) -> ChunkMtex;
    fn get_mmdx(&self) -> ChunkMmdx;
    fn get_mmid(&self) -> ChunkMmid;
    fn get_mwmo(&self) -> ChunkMwmo;
}

impl ChunkVecUtils for Vec<Chunk> {
    fn get_chunk_of_type(&self, chunk_type: &str) -> &Chunk {
        self.iter().find(|it| it.get_id_as_string() == chunk_type.to_owned()).unwrap()
    }

    fn get_mver_chunk(&self) -> ChunkMver {
        ChunkMver::from_chunk(self.get_chunk_of_type("MVER"))
    }

    fn get_mphd_chunk(&self) -> ChunkMphd {
        ChunkMphd::from_chunk(self.get_chunk_of_type("MPHD"))
    }

    fn get_mhdr(&self) -> ChunkMhdr {
        ChunkMhdr::from_chunk(self.get_chunk_of_type("MHDR"))
    }

    fn get_mcin(&self) -> Vec<ChunkMcin> { ChunkMcin::from_chunk(self.get_chunk_of_type("MCIN")) }

    fn get_mtex(&self) -> ChunkMtex { ChunkMtex::from_chunk(self.get_chunk_of_type("MTEX")) }

    fn get_mmdx(&self) -> ChunkMmdx { ChunkMmdx::from_chunk(self.get_chunk_of_type("MMDX")) }

    fn get_mmid(&self) -> ChunkMmid { ChunkMmid::from_chunk(self.get_chunk_of_type("MMID")) }

    fn get_mwmo(&self) -> ChunkMwmo { ChunkMwmo::from_chunk(self.get_chunk_of_type("MWMO")) }
}


#[derive(Debug, Serialize, Deserialize)]
pub struct ChunkMver {
    pub map_version: u32,
}

impl ChunkMver {
    pub fn from_chunk(chunk: &Chunk) -> ChunkMver {
        assert_eq!(chunk.get_id_as_string(), "MVER");
        assert_eq!(chunk.size, 4);
        ChunkMver {
            map_version: chunk.data.get_u32(0).unwrap()
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ChunkMphd {
    pub flags: u32,
    pub something: u32,
    pub unused: [u8; 6],
}

impl ChunkMphd {
    pub fn from_chunk(c: &Chunk) -> ChunkMphd {
        assert_eq!(c.get_id_as_string(), "MPHD");
        assert_eq!(c.size, 32);
        ChunkMphd {
            flags: c.data.get_u32(0).unwrap(),
            something: c.data.get_u32(4).unwrap(),
            unused: c.data[8..14].try_into().unwrap(),
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ChunkMhdr {
    pub flags: u32,
    pub offs_mcin: u32,
    pub offs_mtex: u32,
    pub offs_mmdx: u32,
    pub offs_mmid: u32,
    pub offs_mwmo: u32,
    pub offs_mwid: u32,
    pub offs_mddf: u32,
    pub offs_modf: u32,
}

impl ChunkMhdr {
    pub fn from_chunk(c: &Chunk) -> ChunkMhdr {
        assert_eq!(c.get_id_as_string(), "MHDR");
        assert_eq!(c.size, 64);
        ChunkMhdr {
            flags: c.data.get_u32(0).unwrap(),
            offs_mcin: c.data.get_u32(4).unwrap(),
            offs_mtex: c.data.get_u32(8).unwrap(),
            offs_mmdx: c.data.get_u32(12).unwrap(),
            offs_mmid: c.data.get_u32(16).unwrap(),
            offs_mwmo: c.data.get_u32(20).unwrap(),
            offs_mwid: c.data.get_u32(24).unwrap(),
            offs_mddf: c.data.get_u32(28).unwrap(),
            offs_modf: c.data.get_u32(32).unwrap(),
        }
    }
}


#[derive(Debug, Serialize, Deserialize)]
pub struct ChunkMcin {
    pub offs_mcnk: u32,
    pub size: u32,
    pub flags: u32,
    pub async_id: u32,
}

impl ChunkMcin {
    pub fn from_chunk(c: &Chunk) -> Vec<ChunkMcin> {
        assert_eq!(c.get_id_as_string(), "MCIN");
        assert_eq!(c.size, 4096);
        c.data.chunks(16)
            .map(|chunk| {
                let chunk = chunk.to_vec();
                ChunkMcin {
                    offs_mcnk: chunk.get_u32(0).unwrap(),
                    size: chunk.get_u32(4).unwrap(),
                    flags: chunk.get_u32(8).unwrap(),
                    async_id: chunk.get_u32(12).unwrap(),
                }
            })
            .collect()
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ChunkMtex(Vec<String>);

impl ChunkMtex {
    pub fn from_chunk(c: &Chunk) -> ChunkMtex {
        assert_eq!(c.get_id_as_string(), "MTEX");
        ChunkMtex(c.data.get_null_terminated_strings().unwrap())
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ChunkMmdx(Vec<String>);

impl ChunkMmdx {
    pub fn from_chunk(c: &Chunk) -> ChunkMmdx {
        assert_eq!(c.get_id_as_string(), "MMDX");
        ChunkMmdx(c.data.get_null_terminated_strings().unwrap())
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ChunkMmid(Vec<u32>);

impl ChunkMmid {
    pub fn from_chunk(c: &Chunk) -> ChunkMmid {
        assert_eq!(c.get_id_as_string(), "MMID");
        let offsets = c.data.chunks(4)
            .map(|it| it.to_vec().get_u32(0).unwrap())
            .collect();
        ChunkMmid(offsets)
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ChunkMwmo(Vec<String>);

impl ChunkMwmo {
    pub fn from_chunk(c: &Chunk) -> ChunkMwmo {
        assert_eq!(c.get_id_as_string(), "MWMO");
        ChunkMwmo(c.data.get_null_terminated_strings().unwrap())
    }
}