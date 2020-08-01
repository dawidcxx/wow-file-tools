use std::str::from_utf8;
use std::fs::File;
use std::io::Read;
use crate::byte_utils::*;
use std::convert::TryInto;
use serde::{Serialize, Deserialize};
use crate::common::{R};
use std::path::Path;

#[derive(Debug, Serialize, Deserialize)]
pub struct Chunk {
    pub id: [u8; 4],
    pub size: u32,
    pub data: Vec<u8>,
}

impl Chunk {
    pub fn get_id_as_string(&self) -> String { from_utf8(&self.id).unwrap().chars().rev().collect() }

    pub fn from_path<P: AsRef<Path>>(path: P) -> R<Vec<Chunk>> {
        let mut f = File::open(path)?;
        let file_size = f.metadata()?.len() as usize;
        let mut buffered_file = Vec::with_capacity(file_size);
        f.read_to_end(&mut buffered_file)?;

        let mut offset: usize = 0;
        let mut builder: Vec<Chunk> = Vec::new();
        let max_size = buffered_file.len();

        loop {
            if offset + 4 > max_size {
                break;
            }
            let id: [u8; 4] = buffered_file[offset..offset + 4].try_into()?;
            let size = Chunk::get_chunk_size(&id, &buffered_file, offset)?;
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

    fn get_chunk_size(chunk_id: &[u8; 4], buffered_file: &Vec<u8>, offset: usize) -> R<usize> {
        // The MOGP chunk breaks our parsing system
        // by having an unexpected size provided in the header..
        // This function adds special casing for a chunk of this type.
        // If any other chunk happens to have a similar issue, just add
        // the correct [ChunkFix] entry into the `const` array [#CHUNK_SIZE_FIXES]
        struct ChunkFix {
            chunk_id: [u8; 4],
            chunk_size_correction: usize,
        }

        const CHUNK_SIZE_FIXES: &[ChunkFix] = &[
            ChunkFix {
                chunk_id: [80, 71, 79, 77], // MOGP
                chunk_size_correction: 68,
            },
            ChunkFix {
                chunk_id: [1, 0, 0, 0], // no clue
                chunk_size_correction: 40,
            }
        ];

        let requires_fix = CHUNK_SIZE_FIXES
            .iter()
            .find(|&it| it.chunk_id.ends_with(chunk_id));

        match requires_fix {
            Some(fix) => {
                // use correction
                Ok(fix.chunk_size_correction)
            }
            None => {

                // usual size extraction, 99% of the chunks.
                let size = buffered_file[offset + 4..offset + 8].to_vec().get_u32(0)? as usize;
                Ok(size)
            }
        }
    }
}

pub trait ChunkVecUtils {
    fn get_chunk_of_type_optionally(&self, chunk_type: &str) -> Option<&Chunk>;
    fn get_chunk_of_type(&self, chunk_type: &str) -> &Chunk;
    fn get_chunk_of_type_checked(&self, chunk_type: &str) -> R<&Chunk>;
    fn get_mver_chunk(&self) -> R<ChunkMver>;
    fn get_mphd_chunk(&self) -> ChunkMphd;
    fn get_main(&self) -> ChunkMain;
    fn get_modf(&self) -> Option<ChunkModf>;
    fn get_mhdr(&self) -> ChunkMhdr;
    fn get_mcin(&self) -> Vec<ChunkMcin>;
    fn get_mtex(&self) -> ChunkMtex;
    fn get_mmdx(&self) -> ChunkMmdx;
    fn get_mmid(&self) -> ChunkMmid;
    fn get_mwmo(&self) -> ChunkMwmo;
    fn get_mwid(&self) -> ChunkMwid;
    fn get_mddf(&self) -> ChunkMddf;
    fn get_motx(&self) -> ChunkMotx;
    fn get_mogn(&self) -> ChunkMogn;
    fn get_modn(&self) -> ChunkModn;
    fn get_mohd(&self) -> ChunkMohd;
}

impl ChunkVecUtils for Vec<Chunk> {
    fn get_chunk_of_type_optionally(&self, chunk_type: &str) -> Option<&Chunk> {
        self.iter().find(|it| it.get_id_as_string() == chunk_type.to_owned())
    }

    fn get_chunk_of_type(&self, chunk_type: &str) -> &Chunk {
        self.get_chunk_of_type_optionally(chunk_type)
            .expect(format!("Failed to find chunk of type {}", chunk_type).as_str())
    }

    fn get_chunk_of_type_checked(&self, chunk_type: &str) -> R<&Chunk> {
        self.get_chunk_of_type_optionally(chunk_type)
            .ok_or(format!("Failed to find chunk of type {}", chunk_type).into())
    }

    fn get_mver_chunk(&self) -> R<ChunkMver> {
        let chunk = self.get_chunk_of_type_checked("MVER")?;
        ChunkMver::from_chunk(chunk)
    }

    fn get_mphd_chunk(&self) -> ChunkMphd {
        ChunkMphd::from_chunk(self.get_chunk_of_type("MPHD"))
    }

    fn get_main(&self) -> ChunkMain { ChunkMain::from_chunk(self.get_chunk_of_type("MAIN")) }

    fn get_modf(&self) -> Option<ChunkModf> { self.get_chunk_of_type_optionally("MODF").and_then(ChunkModf::from_chunk) }

    fn get_mhdr(&self) -> ChunkMhdr {
        ChunkMhdr::from_chunk(self.get_chunk_of_type("MHDR"))
    }

    fn get_mcin(&self) -> Vec<ChunkMcin> { ChunkMcin::from_chunk(self.get_chunk_of_type("MCIN")) }

    fn get_mtex(&self) -> ChunkMtex { ChunkMtex::from_chunk(self.get_chunk_of_type("MTEX")) }

    fn get_mmdx(&self) -> ChunkMmdx { ChunkMmdx::from_chunk(self.get_chunk_of_type("MMDX")) }

    fn get_mmid(&self) -> ChunkMmid { ChunkMmid::from_chunk(self.get_chunk_of_type("MMID")) }

    fn get_mwmo(&self) -> ChunkMwmo { ChunkMwmo::from_chunk(self.get_chunk_of_type("MWMO")) }

    fn get_mwid(&self) -> ChunkMwid { ChunkMwid::from_chunk(self.get_chunk_of_type("MWID")) }

    fn get_mddf(&self) -> ChunkMddf { ChunkMddf::from_chunk(self.get_chunk_of_type("MDDF")) }

    fn get_motx(&self) -> ChunkMotx { ChunkMotx::from_chunk(self.get_chunk_of_type("MOTX")) }

    fn get_mogn(&self) -> ChunkMogn { ChunkMogn::from_chunk(self.get_chunk_of_type("MOGN")) }

    fn get_modn(&self) -> ChunkModn { ChunkModn::from_chunk(self.get_chunk_of_type("MODN")) }

    fn get_mohd(&self) -> ChunkMohd { ChunkMohd::from_chunk(self.get_chunk_of_type("MOHD")) }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ChunkMver {
    pub map_version: u32,
}

impl ChunkMver {
    pub fn from_chunk(chunk: &Chunk) -> R<ChunkMver> {
        assert_eq!(chunk.get_id_as_string(), "MVER");
        assert_eq!(chunk.size, 4);
        let map_version = chunk.data.get_u32(0);
        map_version.map(|map_version|
            ChunkMver {
                map_version,
            }
        )
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
pub struct ChunkMain(pub Vec<ChunkMainItem>);

#[derive(Debug, Serialize, Deserialize)]
pub struct ChunkMainItem {
    pub flags: u32,
    pub area: u32,
}

impl ChunkMain {
    pub fn from_chunk(c: &Chunk) -> ChunkMain {
        assert_eq!(c.get_id_as_string(), "MAIN");
        assert_eq!(c.size, 32768);
        let items = c.data.chunks(8)
            .map(|c| {
                let c = c.to_vec();
                let flags = c.get_u32(0).unwrap();
                let area = c.get_u32(4).unwrap();
                ChunkMainItem { flags, area }
            })
            .collect();

        ChunkMain(items)
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ChunkModf {
    id: u32,
    dynamic_id: u32,
    pos: [f32; 3],
    orientation: [f32; 3],
    bounding_box: [f32; 6],
    flags: u16,
    doodad_set_index: u16,
    name_set: u16,
    padding: u16,
}

impl ChunkModf {
    pub fn from_chunk(c: &Chunk) -> Option<ChunkModf> {
        assert_eq!(c.get_id_as_string(), "MODF");
        assert!([64, 0].contains(&c.size), "MODF size should either be 64 or 0.");

        if c.size == 0 {
            return None;
        }

        let id = c.data.get_u32(0).unwrap();
        let dynamic_id = c.data.get_u32(4).unwrap();
        let pos_x = c.data.get_f32(8).unwrap();
        let pos_y = c.data.get_f32(12).unwrap();
        let pos_z = c.data.get_f32(16).unwrap();
        let pos = [pos_x, pos_y, pos_z];
        let orient_x = c.data.get_f32(20).unwrap();
        let orient_y = c.data.get_f32(24).unwrap();
        let orient_z = c.data.get_f32(28).unwrap();
        let orientation = [orient_x, orient_y, orient_z];
        let b1 = c.data.get_f32(32).unwrap();
        let b2 = c.data.get_f32(36).unwrap();
        let b3 = c.data.get_f32(40).unwrap();
        let b4 = c.data.get_f32(44).unwrap();
        let b5 = c.data.get_f32(48).unwrap();
        let b6 = c.data.get_f32(52).unwrap();
        let bounding_box = [b1, b2, b3, b4, b5, b6];
        let flags = c.data.get_u16(56).unwrap();
        let doodad_set_index = c.data.get_u16(58).unwrap();
        let name_set = c.data.get_u16(60).unwrap();
        let padding = c.data.get_u16(62).unwrap();
        Some(ChunkModf {
            id,
            dynamic_id,
            pos,
            orientation,
            bounding_box,
            flags,
            doodad_set_index,
            name_set,
            padding,
        })
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
pub struct ChunkMtex(pub Vec<String>);

impl ChunkMtex {
    pub fn from_chunk(c: &Chunk) -> ChunkMtex {
        assert_eq!(c.get_id_as_string(), "MTEX");
        ChunkMtex(c.data.get_null_terminated_strings().unwrap())
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ChunkMmdx(pub Vec<String>);

impl ChunkMmdx {
    pub fn from_chunk(c: &Chunk) -> ChunkMmdx {
        assert_eq!(c.get_id_as_string(), "MMDX");
        ChunkMmdx(c.data.get_null_terminated_strings().unwrap())
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ChunkMmid(pub Vec<u32>);

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
pub struct ChunkMwmo(pub Vec<String>);

impl ChunkMwmo {
    pub fn from_chunk(c: &Chunk) -> ChunkMwmo {
        assert_eq!(c.get_id_as_string(), "MWMO");
        ChunkMwmo(c.data.get_null_terminated_strings().unwrap())
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ChunkMwid(pub Vec<u32>);

impl ChunkMwid {
    pub fn from_chunk(c: &Chunk) -> ChunkMwid {
        assert_eq!(c.get_id_as_string(), "MWID");
        assert_eq!(c.size % 4, 0);
        let offsets = c.data.chunks(4)
            .map(|it| it.to_vec().get_u32(0).unwrap())
            .collect();
        ChunkMwid(offsets)
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ChunkMddfItem {
    pub mmid_entry: u32,
    pub unique_id: u32,
    pub position: [f32; 3],
    pub rotation: [f32; 3],
    pub scale: u16,
    pub flags: u16,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ChunkMddf(pub Vec<ChunkMddfItem>);

impl ChunkMddf {
    pub fn from_chunk(c: &Chunk) -> ChunkMddf {
        assert_eq!(c.get_id_as_string(), "MDDF");
        assert_eq!(c.size % 36, 0);

        let items: Vec<ChunkMddfItem> = c.data.chunks(36).map(|data| {
            let data = data.to_vec();
            let mmid_entry = data.get_u32(0).unwrap();
            let unique_id = data.get_u32(4).unwrap();
            let pos_x = data.get_f32(8).unwrap();
            let pos_y = data.get_f32(12).unwrap();
            let pos_z = data.get_f32(16).unwrap();
            let position = [pos_x, pos_y, pos_z];
            let rot_x = data.get_f32(20).unwrap();
            let rot_y = data.get_f32(24).unwrap();
            let rot_z = data.get_f32(28).unwrap();
            let rotation = [rot_x, rot_y, rot_z];
            let scale = data.get_u16(32).unwrap();
            let flags = data.get_u16(34).unwrap();
            ChunkMddfItem {
                mmid_entry,
                unique_id,
                position,
                rotation,
                scale,
                flags,
            }
        }).collect();

        ChunkMddf(items)
    }
}


#[derive(Debug, Serialize, Deserialize)]
pub struct ChunkMotx(pub Vec<String>);

impl ChunkMotx {
    pub fn from_chunk(c: &Chunk) -> ChunkMotx {
        assert_eq!(c.get_id_as_string(), "MOTX");
        let strings = c.data.get_null_terminated_strings()
            .unwrap()
            .into_iter()
            .filter(|it| !it.is_empty())
            .collect();

        ChunkMotx(strings)
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ChunkMogn(pub Vec<String>);

impl ChunkMogn {
    pub fn from_chunk(c: &Chunk) -> ChunkMogn {
        assert_eq!(c.get_id_as_string(), "MOGN");
        let strings = c.data.get_null_terminated_strings()
            .unwrap_or(vec![])
            .into_iter()
            .filter(|it| !it.is_empty())
            .collect();
        ChunkMogn(strings)
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ChunkModn(pub Vec<String>);

impl ChunkModn {
    pub fn from_chunk(c: &Chunk) -> ChunkModn {
        assert_eq!(c.get_id_as_string(), "MODN");
        let strings = c.data.get_null_terminated_strings()
            .unwrap_or(vec![])
            .into_iter()
            .filter(|it| !it.is_empty())
            .collect();
        ChunkModn(strings)
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ChunkMohd {
    pub n_materials: u32,
    pub n_groups: u32,
    pub n_ports: u32,
    pub n_lights: u32,
    pub n_models: u32,
    pub n_doodads: u32,
    pub n_doodad_sets: u32,
    pub ambient_color: u32,
    pub wmo_id: u32,
    pub bounding_box_corner_1: [f32; 3],
    pub bounding_box_corner_2: [f32; 3],
}

impl ChunkMohd {
    pub fn from_chunk(c: &Chunk) -> ChunkMohd {
        assert_eq!(c.get_id_as_string(), "MOHD");
        ChunkMohd {
            n_materials: c.data.get_u32(0).unwrap(),
            n_groups: c.data.get_u32(4).unwrap(),
            n_ports: c.data.get_u32(8).unwrap(),
            n_lights: c.data.get_u32(12).unwrap(),
            n_models: c.data.get_u32(16).unwrap(),
            n_doodads: c.data.get_u32(20).unwrap(),
            n_doodad_sets: c.data.get_u32(24).unwrap(),
            ambient_color: c.data.get_u32(28).unwrap(),
            wmo_id: c.data.get_u32(32).unwrap(),
            bounding_box_corner_1: {
                let b1 = c.data.get_f32(36).unwrap();
                let b2 = c.data.get_f32(40).unwrap();
                let b3 = c.data.get_f32(44).unwrap();
                [b1, b2, b3]
            },
            bounding_box_corner_2: {
                let b1 = c.data.get_f32(48).unwrap();
                let b2 = c.data.get_f32(52).unwrap();
                let b3 = c.data.get_f32(56).unwrap();
                [b1, b2, b3]
            },
        }
    }
}