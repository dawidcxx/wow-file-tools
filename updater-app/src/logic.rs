use std::{path::PathBuf, sync::Mutex};

use nwg::{MessageButtons, MessageIcons};
use serde::{Deserialize, Serialize};

use crate::api::ReleaseDetails;
use lazy_static::lazy_static;

lazy_static! {
    pub static ref WOW_EXE_PATH: Mutex<Option<PathBuf>> = Mutex::new(None);
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Config {
    pub current_release: u64,
    pub latest_release: u64,
    pub installed: bool,
    pub prior_realmlist: Option<String>,
}

impl Default for Config {
    fn default() -> Self {
        Config {
            current_release: 0,
            latest_release: 0,
            installed: false,
            prior_realmlist: None,
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
            }
        },
        Err(_) => {
            println!("Failed to read arenacraft.cfg, using default config");
            Config::default()
        }
    }
}

pub fn write_arenacraft_cfg(config: &Config) {
    println!("Updating arenacraft.cfg: {:?}", config);
    let cfg = serde_json::to_string(&config);
    match cfg {
        Ok(cfg) => match std::fs::write("arenacraft.cfg", cfg) {
            Ok(_) => {
                println!("arenacraft.cfg written successfully");
            }
            Err(err) => {
                println!("Failed to write arenacraft.cfg: {:?}", err);
            }
        },
        Err(err) => {
            println!("Failed to serialize arenacraft.cfg: {:?}", err);
        }
    }
}

pub fn check_has_wow_exe(cwd: &std::path::PathBuf) {
    let candidates = vec![cwd.join("Wow.exe"), cwd.join("Wow64.exe")];
    let found = candidates.iter().find(|p| p.exists());

    match found {
        Some(wow_exe_path) => {
            WOW_EXE_PATH.lock().unwrap().replace(wow_exe_path.clone());
        }
        None => {
            nwg::message(&nwg::MessageParams {
                title: "Error",
                content: "This program must be run from your World of Warcraft directory\n\nPlease copy the program to your World of Warcraft directory and run it from there",
                buttons: MessageButtons::Ok,
                icons: MessageIcons::Error,
            });
            nwg::stop_thread_dispatch();
        }
    }
}

pub fn get_wow_exe() -> PathBuf {
    let wow_exe = WOW_EXE_PATH.lock().unwrap();
    wow_exe.clone().unwrap()
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
