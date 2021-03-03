use std::{path::PathBuf, vec};

#[derive(Debug, Clone)]
pub struct MpqPath {
    is_dir: bool,
    components: Vec<String>,
}

impl MpqPath {
    pub fn from_string(src: &String) -> Option<Self> {
        let src_normalized = src.trim().replace("/", "\\");

        if src_normalized.is_empty() {
            return None;
        }

        if src_normalized == "\\" {
            return Some(MpqPath {
                is_dir: true,
                components: vec![],
            });
        }

        let components: Vec<String> = src_normalized
            .replace("/", "\\")
            .split("\\")
            .filter(|it| !it.is_empty())
            .map(|it| it.to_uppercase())
            .collect();

        if components.len() == 0 {
            return None;
        }

        let leaf_component = components.last().unwrap();

        // infer from the name is it's a file/directory.
        let is_dir = match leaf_component.split(".").count() {
            1 => {
                // doesn't have an extension, ...probably a directory
                // expect when it's a listfile.
                leaf_component.to_uppercase().eq("(LISTFILE)")
            }
            2 => false,
            _ => panic!("Malformed MPQ file path"),
        };
        return Some(MpqPath {
            components,
            is_dir: is_dir,
        });
    }

    pub fn is_dir(&self) -> bool {
        self.is_dir
    }
    pub fn is_file(&self) -> bool {
        !self.is_dir
    }
    pub fn is_root(&self) -> bool {
        self.components.len() == 0
    }
    pub fn to_string_path(&self) -> String {
        self.components.join("\\")
    }

    pub fn to_relative_path_buf(&self) -> PathBuf {
        let mut pb = PathBuf::with_capacity(self.components.len());
        pb.push(".");
        self.components.iter().for_each(|c| pb.push(c));
        return pb;
    }

    pub fn parent(&self) -> Self {
        let components = match self.components.len() {
            0 => panic!("MPQPath: Tried to obtain the parent of a root node"),
            1 => vec![],
            _ => self
                .components
                .iter()
                .take(self.components.len() - 1)
                .cloned()
                .collect(),
        };
        return MpqPath {
            components,
            is_dir: true,
        };
    }
}

pub struct MpqPathUtil;

impl MpqPathUtil {
    pub fn matching(entry: MpqPath, matches_source: Vec<MpqPath>) -> Vec<MpqPath> {
        let mut matching_entries = matches_source.clone();
        let mut index = 0;
        while let Some(curr) = entry.components.get(index) {
            matching_entries.drain_filter(|other_path| {
                if let Some(other) = other_path.components.get(index) {
                    if curr == other {
                        false
                    } else {
                        true
                    }
                } else {
                    true
                }
            });
            index += 1;
        }
        return matching_entries;
    }
}
