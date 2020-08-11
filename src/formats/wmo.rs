use crate::common::R;
use crate::formats::chunk::{Chunk, ChunkMver, ChunkVecUtils, ChunkMotx, ChunkMogn, ChunkModn, ChunkMohd, ChunkMolr};
use serde::{Serialize, Deserialize};
use std::collections::HashSet;
use std::path::{Path, PathBuf};
use crate::byte_utils::VecUtils;

const ROOT_FILE_CHUNKS: &[&str] = &["MOMT", "MOGI", "MOSB", "MOVV", "MODN"];
const GROUP_FILE_CHUNKS: &[&str] = &["MOGP", "MOPY", "MOVI", "MONR", "MOTV"];

#[derive(Debug, Serialize, Deserialize)]
pub struct WmoFile {
    pub root: WmoRootFile,
    pub groups: Vec<WmoGroupFile>,
    pub loaded_group_files: Vec<PathBuf>,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum WmoFileVariant {
    ROOT(WmoRootFile),
    GROUP(WmoGroupFile),
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WmoRootFile {
    pub mver: ChunkMver,
    pub motx: ChunkMotx,
    pub mohd: ChunkMohd,
    pub momt: (),
    pub mogn: ChunkMogn,
    pub mogi: ChunkMogi,
    pub mosb: (),
    pub mopv: (),
    pub mopt: (),
    pub mopr: (),
    pub movv: (),
    pub movb: (),
    pub molt: (),
    pub mods: (),
    pub modn: ChunkModn,
    pub modd: (),
    pub mfog: (),
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WmoGroupFile {
    pub mver: ChunkMver,
    pub mogp: (),
    pub mopy: (),
    pub movi: (),
    pub movt: (),
    pub monr: (),
    pub motv: (),
    pub moba: (),
    pub molr: Option<ChunkMolr>,
    pub modr: Option<()>,
    pub mobn: Option<()>,
    pub mobr: Option<()>,
    pub mpbv: Option<()>,
    pub mpbp: Option<()>,
    pub mpbi: Option<()>,
    pub mpbg: Option<()>,
    pub mocv: Option<()>,
    pub mliq: Option<()>,
    pub mori: Option<()>,
    pub morb: Option<()>,
}

impl WmoFile {
    pub fn from_path(path: &str) -> R<WmoFile> {
        let chunks = Chunk::from_path(path)?;
        let variant = WmoFileVariant::new(chunks)?;
        match variant {
            WmoFileVariant::ROOT(root_file) => {
                // ok we have a root file,
                // load the dependent groups
                let parent_dir = Path::new(path).parent().unwrap();
                let original_file_name = Path::new(path).file_name().unwrap().to_str().unwrap();

                let (groups, loaded_group_files) = WmoFile::get_groups(parent_dir, original_file_name, &root_file);

                Ok(WmoFile {
                    root: root_file,
                    groups,
                    loaded_group_files,
                })
            }
            WmoFileVariant::GROUP(_) => {
                Err("WmoFile#from_path: Given WMO must a be root type WMO.".into())
            }
        }
    }

    fn get_groups(
        parent_path: &Path,
        original_file_name: &str,
        root_file: &WmoRootFile,
    ) -> (Vec<WmoGroupFile>, Vec<PathBuf>) {
        let group_names = WmoRootFile::get_group_names(root_file.mohd.n_groups, original_file_name);
        let mut loaded_group_paths = Vec::new();

        let groups = group_names.iter()
            .map(|group_name| {
                let mut b = PathBuf::new();
                b.push(parent_path);
                b.push(group_name);

                let wmo_group_file = Self::load_group_wmo(b.to_str().unwrap())
                    .expect(format!("Failed to load group WMO {:?}", b).as_str());

                // let wmo_group_file = Self::load_group_wmo(b.to_str().unwrap());
                // if wmo_group_file.is_ok() {
                loaded_group_paths.push(b);
                // }
                wmo_group_file
            })
            // .filter_map(|g| g.ok())
            .collect();
        (groups, loaded_group_paths)
    }

    fn load_group_wmo(path: &str) -> R<WmoGroupFile> {
        let chunks = Chunk::from_path(path)?;
        WmoGroupFile::new(chunks)
    }
}

impl WmoFileVariant {
    fn new(chunks: Vec<Chunk>) -> R<WmoFileVariant> {
        fn matches_file_type(marker_chunks: &[&str], lookup: &HashSet<String>) -> bool {
            marker_chunks
                .iter()
                .all(|c| {
                    lookup.contains(*c)
                })
        }

        let chunk_names_lookup: HashSet<String> = chunks.iter()
            .map(|v| v.get_id_as_string())
            .collect();

        if matches_file_type(ROOT_FILE_CHUNKS, &chunk_names_lookup) {
            Ok(WmoFileVariant::ROOT(WmoRootFile::new(chunks)?))
        } else if matches_file_type(GROUP_FILE_CHUNKS, &chunk_names_lookup) {
            Ok(WmoFileVariant::GROUP(WmoGroupFile::new(chunks)?))
        } else {
            panic!("WmoFile#new: Cannot create a root or group WMO from given chunks!")
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ChunkMogiItem {
    pub flags: u32,
    pub bounding_box: [f32; 6],
    pub name_offset: i32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ChunkMogi(pub Vec<ChunkMogiItem>);

impl WmoRootFile {
    pub fn get_group_names(n_groups: u32, original_file_name: &str) -> Vec<String> {
        let file_name = original_file_name
            .split("/")
            .last()
            .unwrap()
            .split(".wmo")
            .nth(0)
            .unwrap();

        (0..n_groups)
            .map(|index: u32| {
                format!("{}_{:0>3}.wmo", file_name, index)
            })
            .collect()
    }

    fn new(chunks: Vec<Chunk>) -> R<WmoRootFile> {
        let mver = chunks.get_mver_chunk()?;
        let motx = chunks.get_motx();
        let mohd = chunks.get_mohd();
        let mogn = chunks.get_mogn();
        let modn = chunks.get_modn();
        let mogi = chunks.get_mogi();

        Ok(WmoRootFile {
            mver,
            motx,
            mohd,
            momt: (),
            mogn,
            mogi,
            mosb: (),
            mopv: (),
            mopt: (),
            mopr: (),
            movv: (),
            movb: (),
            molt: (),
            mods: (),
            modn,
            modd: (),
            mfog: (),
        })
    }
}

impl WmoGroupFile {
    fn new(chunks: Vec<Chunk>) -> R<WmoGroupFile> {
        let mver = chunks.get_mver_chunk()?;
        let molr = chunks.get_molr();
        Ok(WmoGroupFile {
            mver,
            mogp: (),
            mopy: (),
            movi: (),
            movt: (),
            monr: (),
            motv: (),
            moba: (),
            molr,
            modr: None,
            mobn: None,
            mobr: None,
            mpbv: None,
            mpbp: None,
            mpbi: None,
            mpbg: None,
            mocv: None,
            mliq: None,
            mori: None,
            morb: None,
        })
    }
}

trait WmoChunkExt {
    fn get_mogi(&self) -> ChunkMogi;
}

impl WmoChunkExt for Vec<Chunk> {
    fn get_mogi(&self) -> ChunkMogi { ChunkMogi::from_chunk(self.get_chunk_of_type("MOGI")) }
}

impl ChunkMogi {
    fn from_chunk(chunk: &Chunk) -> ChunkMogi {
        let mapped = chunk.data.chunks(32)
            .map(|segment| {
                let segment = segment.to_vec();
                let flags = segment.get_u32(0).unwrap();
                let b1 = segment.get_f32(4).unwrap();
                let b2 = segment.get_f32(8).unwrap();
                let b3 = segment.get_f32(12).unwrap();
                let b4 = segment.get_f32(16).unwrap();
                let b5 = segment.get_f32(20).unwrap();
                let b6 = segment.get_f32(24).unwrap();
                let bounding_box = [b1, b2, b3, b4, b5, b6];
                let name_offset = segment.get_i32(28).unwrap();
                ChunkMogiItem {
                    flags,
                    bounding_box,
                    name_offset,
                }
            })
            .collect();

        ChunkMogi(mapped)
    }
}

#[cfg(test)]
#[test]
fn wmo_root_get_group_names() {
    let group_names = WmoRootFile::get_group_names(2, "test.wmo");
    assert_eq!(group_names, vec![
        "test_000.wmo",
        "test_001.wmo",
    ]);

    let group_names = WmoRootFile::get_group_names(5, "test00.wmo");
    assert_eq!(group_names, vec![
        "test00_000.wmo",
        "test00_001.wmo",
        "test00_002.wmo",
        "test00_003.wmo",
        "test00_004.wmo",
    ]);

    let group_names = WmoRootFile::get_group_names(5, "WMO/woRlD//whatever//test00.wmo");
    assert_eq!(group_names, vec![
        "test00_000.wmo",
        "test00_001.wmo",
        "test00_002.wmo",
        "test00_003.wmo",
        "test00_004.wmo",
    ]);
}
