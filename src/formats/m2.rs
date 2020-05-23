use serde::{Deserialize, Serialize};
use crate::common::R;
use std::path::Path;
use crate::byte_utils::VecUtils;

#[derive(Debug, Serialize, Deserialize)]
pub struct M2File {
    pub magic: String,
    pub version: u32,
    pub name: String,
    pub textures: Vec<String>,
}

impl M2File {
    pub fn from_path<P: AsRef<Path>>(path: P) -> R<M2File> {
        let data = std::fs::read(path)?;
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

        let textures = if n_textures > 0 {
            let text_offset = bytes.get_u32(84)?;
            let mut builder = Vec::with_capacity(n_textures as usize);
            for i in 1..=n_textures {
                let name_offset = bytes.get_u32((text_offset + (12 * i)) as usize)?;
                let name = bytes.get_string_null_terminated(name_offset as usize)?;
                if name.ne("") { // ignore empty strings..
                    builder.push(name);
                }
            }
            builder
        } else {
            vec![]
        };

        Ok(M2File {
            magic,
            version,
            name,
            textures,
        })
    }
}