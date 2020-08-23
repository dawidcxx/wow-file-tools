use serde::{Deserialize, Serialize};
use crate::common::R;
use std::path::{Path};
use crate::byte_utils::VecUtils;


#[derive(Debug, Serialize, Deserialize)]
struct M2Array<T> {
    pub size: u32,
    pub offset: u32,
    pub elements: Vec<T>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct M2Particle {
    pub particle_id: u32,
    pub flags: u32,
    // pub pos: [f32; 3],
    // pub bone: u16,
    // pub texture: u16,
    // pub geometry_model_filename: M2Array<char>,
    // pub recursion_model_filename: M2Array<char>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct M2File {
    pub magic: String,
    pub version: u32,
    pub name: String,
    pub textures: Vec<String>,
    pub particles: Vec<M2Particle>,
}

impl M2File {
    pub fn from_path<P: AsRef<Path>>(path: P) -> R<M2File> {
        let v = path.as_ref().to_path_buf();

        let data = std::fs::read(v.clone())?;

        M2File::from_bytes(data)
            .map_err(|e| {
                format!("Failed to read M2 file. {}", e).into()
            })
    }

    fn from_bytes(bytes: Vec<u8>) -> R<M2File> {
        let magic = bytes.get_string(0, 4)?;
        let version = bytes.get_u32(4)?;
        let name_offset = bytes.get_u32(12)?;
        let name = bytes.get_string_null_terminated(name_offset as usize)?;

        let n_textures = bytes.get_u32(80)?;
        let text_offset = bytes.get_u32(84)?;
        let mut texture_builder = Vec::with_capacity(n_textures as usize);

        for i in 0..n_textures {
            let name_offset = bytes.get_u32((text_offset + (12 * (i + 1)) + (i * 4)) as usize)?;
            let name = bytes.get_string_null_terminated(name_offset as usize);
            if let Ok(name) = name {
                // a lot of garbage names.. filtering them out for now.
                if name.ends_with("blp") || name.ends_with("BLP") {
                    texture_builder.push(name);
                }
            }
        }

        const PARTICLE_SIZE: u32 = 456;

        let n_particles = bytes.get_u32(0x128)?;
        let particle_offset = bytes.get_u32(0x12C)?;
        let mut particle_builder = Vec::with_capacity(n_particles as usize);

        for i in 0..n_particles {
            let offset = (particle_offset + (i * PARTICLE_SIZE)) as usize;
            let particle_id = bytes.get_u32(offset)?;
            let flags = bytes.get_u32(offset + 4)?;
            particle_builder.push(M2Particle {
                particle_id,
                flags,
            });
        }


        Ok(M2File {
            magic,
            version,
            name,
            textures: texture_builder,
            particles: particle_builder
        })
    }
}