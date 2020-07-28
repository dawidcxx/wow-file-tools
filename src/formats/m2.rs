use serde::{Deserialize, Serialize};
use crate::common::R;
use std::path::{Path, PathBuf};
use crate::byte_utils::VecUtils;

#[derive(Debug, Serialize, Deserialize)]
pub struct M2File {
    pub magic: String,
    pub version: u32,
    pub name: String,
    pub textures: Vec<String>,
    pub replaceable_textures: Vec<String>,
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

        let mut trap = false;
        for i in 1..=n_textures {
            let name_offset = bytes.get_u32((text_offset + (12 * i)) as usize)?;
            let name = bytes.get_string_null_terminated(name_offset as usize);
            if let Ok(name) = name {
                let namec = name.clone();
                // a lot of garbage names.. filtering them out for now.
                if name.ends_with("blp") || name.ends_with("BLP") {
                    texture_builder.push(name);
                }
                if namec.to_uppercase().contains("ULDUMSANDBLOWING01.BLP") {
                    trap = true;
                }
            }
            if trap {
                // println!("trap: {:?}", bytes.get_string_null_terminated(name_offset as usize))
            }
        }

        let n_replaceable_textures = bytes.get_u32(80)?;
        let replaceable_textures_offset = bytes.get_u32(84)?;
        let mut replaceable_texture_builder = Vec::with_capacity(n_replaceable_textures as usize);

        for i in 1..=n_replaceable_textures {
            let offset = bytes.get_u32((replaceable_textures_offset + (12 * i)) as usize)?;
            let texture_name = bytes.get_string_null_terminated(offset as usize);
            if let Ok(name) = texture_name {
                // a lot of garbage names.. filtering them out for now.
                if name.ends_with("blp") || name.ends_with("BLP") {
                    replaceable_texture_builder.push(name);
                }
            }
        }

        // if texture_builder.contains(&"WORLD\\EXPANSION03\\DOODADS\\ULDUM\\BLOWINGSAND\\ULDUMSANDBLOWING01.BLP".to_string()) {
        //     println!("offset: {} n_texts {}", text_offset, n_textures);
        //     let ofsTexUnits = bytes.get_u32(0x8b)?;
        //     println!("ofsTexUnits {}", ofsTexUnits);
        // }

        Ok(M2File {
            magic,
            version,
            name,
            textures: texture_builder,
            replaceable_textures: replaceable_texture_builder,
        })
    }
}