use nwg::{MessageButtons, MessageIcons};
use serde::{Deserialize, Serialize};

use crate::api::ReleaseDetails;

#[derive(Serialize, Deserialize, Debug)]
pub struct Config {
    pub current_release: u64,
    pub installed: bool,
}

impl Default for Config {
    fn default() -> Self {
        Config {
            current_release: 1,
            installed: false,
        }
    }
}

pub fn read_arenacraft_cfg() -> Config {
    let cfg = std::fs::read_to_string("arenacraft.cfg");
    match cfg {
        Ok(cfg) => match serde_json::from_str(&cfg) {
            Ok(config) => config,
            Err(_) => {
                println!("Failed to parse arenacraft.cfg, using default config");
                Config::default()
            },
        },
        Err(_) => {
            println!("Failed to read arenacraft.cfg, using default config");
            Config::default()
        },
    }
}

pub fn write_arenacraft_cfg(config: &Config) -> std::io::Result<()> {
    let cfg = serde_json::to_string(&config)?;
    std::fs::write("arenacraft.cfg", cfg)
}

pub fn check_has_wow_exe(cwd: &std::path::PathBuf) {
    let candidates = vec![cwd.join("Wow.exe"), cwd.join("Wow64.exe")];
    let found = candidates.iter().any(|p| p.exists());
    if !found {
        nwg::message(&nwg::MessageParams {
            title: "Error",
            content: "This program must be run from your World of Warcraft directory\n\nPlease copy the program to your World of Warcraft directory and run it from there",
            buttons: MessageButtons::Ok,
            icons: MessageIcons::Error,
        });
        nwg::stop_thread_dispatch();
    }
}

pub fn check_has_data_dir(cwd: &std::path::PathBuf) {
    let found = cwd.join("Data").exists();
    if !found {
        nwg::message(&nwg::MessageParams {
            title: "Error",
            content: "This program must be run from your World of Warcraft directory\n\nPlease copy the program to your World of Warcraft directory and run it from there",
            buttons: MessageButtons::Ok,
            icons: MessageIcons::Error,
        });
        nwg::stop_thread_dispatch();
    }
}

pub fn compute_files_to_download(releases: Vec<ReleaseDetails>) -> Vec<String> {
    let mut files_to_download = std::collections::HashSet::new();
    for release in releases {
        for file in release.files {
            files_to_download.insert(file.path);
        }
    }
    return files_to_download.into_iter().collect();
}
