use anyhow::Context;
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
    pub flags_1: u16,
    pub flags_2: u16,
    pub pos: [f32; 3],
    pub bone: u16,
    pub texture_id: u16,
    pub model_file_name: String,
    pub particle_name: String,
    pub blending_type: u8,
    pub emitter_type: u8,
    pub particle_dbc_color: u16,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct M2File {
    pub magic: String,
    pub version: u32,
    pub name: String,
    pub textures: Vec<String>,
    pub particles: Vec<M2Particle>,
    pub n_particles: u32,
}

impl M2File {
    pub fn from_path<P: AsRef<Path>>(path: P) -> R<M2File> {
        let path = path.as_ref().to_path_buf();
        let data = std::fs::read(path.clone())
            .with_context(|| format!("Failed to m2 file '{}'", path.display()))?;
        M2File::from_bytes(data)
            .context("Failed to read M2 file.")
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

        const PARTICLE_SIZE: u32 = 476;

        let n_particles = bytes.get_u32(0x128)?;
        let particle_offset = bytes.get_u32(0x12C)?;
        let mut particle_builder = Vec::with_capacity(n_particles as usize);

        for i in 0..n_particles {
            let offset = (particle_offset + (i * PARTICLE_SIZE)) as usize;
            let particle_id = bytes.get_u32(offset)?;
            let flags_1 = bytes.get_u16(offset + 4)?;
            let flags_2 = bytes.get_u16(offset + 6)?;
            let pos = {
                let pos_1 = bytes.get_f32(offset + 8)?;
                let pos_2 = bytes.get_f32(offset + 12)?;
                let pos_3 = bytes.get_f32(offset + 16)?;
                [pos_1, pos_2, pos_3]
            };
            let bone = bytes.get_u16(offset + 20)?;
            let texture_id = bytes.get_u16(offset + 22)?;
            let model_file_name = {
                let len = bytes.get_u32(offset + 24)? as usize;
                let offs = bytes.get_u32(offset + 28)? as usize;
                bytes.get_string(offs, len)?
            };
            let particle_name = {
                let len = bytes.get_u32(offset + 32)? as usize;
                let offs = bytes.get_u32(offset + 36)? as usize;
                bytes.get_string(offs, len)?
            };

            let blending_type = bytes.get_byte(offset + 40)?;
            let emitter_type = bytes.get_byte(offset + 41)?;
            let particle_dbc_color = bytes.get_u16(offset + 42)?;

            particle_builder.push(M2Particle {
                particle_id,
                flags_1,
                flags_2,
                pos,
                bone,
                model_file_name,
                particle_name,
                texture_id,
                blending_type,
                emitter_type,
                particle_dbc_color,
            });
        }


        Ok(M2File {
            magic,
            version,
            name,
            textures: texture_builder,
            particles: particle_builder,
            n_particles,
        })
    }
}