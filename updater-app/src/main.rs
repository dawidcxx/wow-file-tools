/*!
    A very simple application that show your name in a message box.
    Unlike `basic_d`, this example use layout to position the controls in the window
*/
extern crate native_windows_derive as nwd;
extern crate native_windows_gui as nwg;
mod api;

use api::ReleaseDetails;
use nwd::NwgUi;
use nwg::NativeUi;
use nwg::{
    stretch::{
        geometry::Rect,
        style::{Dimension as D, FlexDirection},
    },
    MessageButtons, MessageIcons,
};
use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use std::str::FromStr;
use std::vec;

const FIFTY_PC: D = D::Percent(0.5);
const PT_10: D = D::Points(10.0);
const PT_5: D = D::Points(5.0);
const PADDING: Rect<D> = Rect {
    start: PT_10,
    end: PT_10,
    top: PT_10,
    bottom: PT_10,
};
const MARGIN: Rect<D> = Rect {
    start: PT_5,
    end: PT_5,
    top: PT_5,
    bottom: PT_5,
};

#[derive(Default, NwgUi)]
pub struct BasicApp {
    #[nwg_control(size: (640, 480), position: (0, 0), title: "MPQ Updater App", flags: "WINDOW|VISIBLE")]
    #[nwg_events( OnWindowClose: [], OnInit: [BasicApp::on_mount] )]
    window: nwg::Window,

    #[nwg_resource(family: "Segoe UI", size: 18)]
    font: nwg::Font,

    #[nwg_layout(parent: window, flex_direction: FlexDirection::Column, padding: PADDING)]
    layout: nwg::FlexboxLayout,
}

impl BasicApp {
    fn on_mount(&self) {
        // Check if program is running in same directory as Wow.exe
        // If not, show error message and exit
        let cwd = std::env::current_dir().expect("Failed to get current directory");
        println!("Running from {:?}", cwd);
        check_has_wow_exe(&cwd);
        check_has_data_dir(&cwd);
        println!("Directory checks passed");

        // Get current release version
        let mut config = read_arenacraft_cfg().unwrap_or_else(|_| Config { current_release: 1 });
        println!("Loaded config: {:?}", config);
        config.current_release = 1;

        let releases = api::Api::default()
            .get_releases(config.current_release)
            .expect("Failed to get releases");

        let archivePath = cwd.join("data/patch-A.MPQ");
        let mut mpq = if archivePath.exists() {
            stormlib::MpqArchive::from_path(archivePath.to_str().unwrap())
                .expect("Failed to open archive")
        } else {
            stormlib::MpqArchive::new(archivePath.to_str().unwrap())
                .expect("Failed to create archive")
        };

        let files_to_download = compute_files_to_download(releases);
        for file in files_to_download {
            let start_time = std::time::Instant::now();
            let file_content = api::Api::default()
                .get_file(&file)
                .expect("Failed to download file");
            println!("Downloaded file: {} in {:?}", file, start_time.elapsed());
            mpq.write_file(file.as_str(), &file_content).expect("Failed to write MPQ");
        }
    }
}

fn main() {
    nwg::init().expect("Failed to init Native Windows GUI");
    nwg::Font::set_global_family("Segoe UI").expect("Failed to set default font");
    let _app = BasicApp::build_ui(Default::default()).expect("Failed to build UI");
    nwg::dispatch_thread_events();
}

#[derive(Serialize, Deserialize, Debug)]
struct Config {
    current_release: u64,
}

fn read_arenacraft_cfg() -> std::io::Result<Config> {
    let cfg = std::fs::read_to_string("arenacraft.cfg")?;
    let config: Config = serde_json::from_str(&cfg)?;
    Ok(config)
}

fn write_arenacraft_cfg(config: &Config) -> std::io::Result<()> {
    let cfg = serde_json::to_string(&config)?;
    std::fs::write("arenacraft.cfg", cfg)
}

fn check_has_wow_exe(cwd: &std::path::PathBuf) {
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

fn check_has_data_dir(cwd: &std::path::PathBuf) {
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

fn compute_files_to_download(releases: Vec<ReleaseDetails>) -> Vec<String> {
    let mut files_to_download = std::collections::HashSet::new();
    for release in releases {
        for file in release.files {
            files_to_download.insert(file.path);
        }
    }
    return files_to_download.into_iter().collect();
}

fn write_file_to_mpq(file: &str, content: Vec<u8>) {}
