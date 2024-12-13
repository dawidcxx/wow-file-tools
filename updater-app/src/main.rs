/*!
    A very simple application that show your name in a message box.
    See `basic` for the version without the derive macro
*/

extern crate native_windows_derive as nwd;
extern crate native_windows_gui as nwg;

use std::{
    cell::RefCell,
    collections::VecDeque,
    sync::{Arc, Mutex},
    thread::{self, sleep_ms},
    time::Duration,
};

use nwd::NwgUi;
use nwg::NativeUi;
use stormlib::MpqArchive;

mod api;
mod logic;

#[derive(Default, NwgUi)]
pub struct UpdaterApp {
    apiRef: api::Api,
    compute: RefCell<Option<thread::JoinHandle<anyhow::Result<VecDeque<String>>>>>,

    #[nwg_control(size: (640, 480), position: (0, 0), title: "ArenaCraft Launcher", flags: "WINDOW|VISIBLE")]
    #[nwg_events( OnInit: [UpdaterApp::on_mount], OnWindowClose: [UpdaterApp::on_exit])]
    window: nwg::Window,

    #[nwg_control]
    #[nwg_events( OnNotice: [UpdaterApp::on_notice] )]
    notice: nwg::Notice,

    #[nwg_resource(family: "Segoe UI", size: 24)]
    btn_font: nwg::Font,

    #[nwg_resource(family: "Segoe UI", size: 18)]
    txt_font: nwg::Font,

    #[nwg_control(text: "Play", font: Some(&data.btn_font), position: (10, 10), size: (620, 50))]
    play_button: nwg::Button,

    #[nwg_control( position: (10, 70), size: (620, 50))]
    update_progress: nwg::ProgressBar,

    #[nwg_control(text: "Install", font: Some(&data.btn_font), position: (10, 70), size: (620, 50))]
    #[nwg_events( OnButtonClick: [UpdaterApp::update_or_install] )]
    update_or_install_button: nwg::Button,

    #[nwg_control(text: "Uninstall", font: Some(&data.btn_font), position: (10, 130), size: (620, 50))]
    uninstall_button: nwg::Button,

    #[nwg_control(font: Some(&data.txt_font), position: (10, 190), size: (620, 50))  ]
    current_status_label: nwg::Label,
    // #[nwg_control(font: Some(&data.txt_font),
    //               background_color: Some([255, 237, 213]),
    //               position: (0, 460),
    //               size: (640, 30)
    // )]
    // #[nwg_events(MousePressRightUp: [])]
    // status_text: nwg::RichLabel,
}

impl UpdaterApp {
    fn on_mount(&self) {
        self.update_progress.set_visible(false);
        let cwd = std::env::current_dir().expect("Failed to get current directory");
        logic::check_has_wow_exe(&cwd);
        logic::check_has_data_dir(&cwd);

        let config = logic::read_arenacraft_cfg();

        if config.installed {
            self.update_or_install_button.set_text("Update");
            self.play_button.set_enabled(true);
            self.uninstall_button.set_enabled(true);
            self.current_status_label.set_text(&format!(
                "Current version: v{}",
                0.1 * config.current_release as f32
            ));
        } else {
            self.update_or_install_button.set_text("Install");
            self.play_button.set_enabled(false);
            self.uninstall_button.set_enabled(false);
            self.current_status_label
                .set_text("Press the Install button to continue");
        }
    }

    fn on_exit(&self) {
        nwg::stop_thread_dispatch();
    }

    fn uinstall(&self) {}
    fn play(&self) {}

    fn update_or_install(&self) {
        let config = logic::read_arenacraft_cfg();
        if config.installed {
            self.update(config);
        } else {
            self.install(config);
        }
    }

    fn update(&self, config: logic::Config) {
        self.current_status_label.set_text("Checking for updates..");
        match self.apiRef.get_releases(config.current_release) {
            Ok(releases) => {
                self.current_status_label
                    .set_text(format!("Found {} missing update(s)", releases.len()).as_str());
                let files_to_download = logic::compute_files_to_download(releases);
                self.update_progress
                    .set_range(0..files_to_download.len() as u32);
                self.update_progress.set_pos(0);
                self.update_progress.set_step(1);
                self.update_progress.set_visible(true);
                self.update_or_install_button.set_visible(false);
                self.download_updates(VecDeque::from(files_to_download));
            }
            Err(err) => {
                println!("Failed to get updates: {:?}", err);
                self.current_status_label
                    .set_text(format!("Failed to get updates: '{:?}'", err).as_str());
            }
        }
    }

    fn install(&self, config: logic::Config) {
        self.current_status_label.set_text("Creating MPQ archive");
        let archive = stormlib::MpqArchive::new("data/patch-A.mpq");

        match archive {
            Ok(archive) => {
                self.current_status_label
                    .set_text("MPQ archive initialized");
                self.update(config);
            }
            Err(err) => {
                println!("Failed to create MPQ archive: '{:?}'", err);
                self.current_status_label
                    .set_text(format!("Failed to create MPQ archive: {}", err).as_str());
            }
        }
    }

    fn download_updates(&self, mut files_to_download: VecDeque<String>) {
        let sender = self.notice.sender();
        let api = self.apiRef.clone();

        self.compute.borrow_mut().replace(thread::spawn(move || {
            let file_path = files_to_download
                .pop_front()
                .expect("Called download_updates on empty queue?");
            println!("Downloading file: {}", file_path);
        
            let file_content = api.get_file(&file_path)?;
            println!("Downloaded file: {}", file_path);
        
            let mut archive = MpqArchive::from_path("data/patch-A.mpq")?;
            archive.write_file(file_path.as_str(), &file_content)?;
            println!("Added file to MPQ: {}", file_path);


            sender.notice();
            Ok(files_to_download)
        }));
    }

    fn on_notice(&self) {
        println!("Notice received");
        let mut response = self.compute.borrow_mut();

        match response.take() {
            Some(thread) => {
                let unprocessed_files = thread.join().unwrap();
                match unprocessed_files {
                    Ok(unprocessed_files) => {
                        println!("Files to download left: {:?}", unprocessed_files.len());
                        if unprocessed_files.is_empty() {
                            println!("All updates processed");
                            self.update_progress.set_visible(false);
                            self.update_or_install_button.set_visible(true);
                            self.on_mount();
                        } else {
                            drop(response);
                            self.update_progress.set_pos(self.update_progress.pos() + 1);
                            self.download_updates(unprocessed_files);
                            println!("Processing next update");
                        }
                    }
                    Err(err) => {
                        println!("Failed to download updates: {:?}", err);
                        self.current_status_label
                            .set_text(format!("Failed to download updates: '{:?}'", err).as_str());
                    }
                }
            }
            None => {}
        }
      
    }
}

fn main() {
    // only for testing
    unsafe { nwg::set_dpi_awareness() }

    nwg::init().expect("Failed to init Native Windows GUI");
    nwg::Font::set_global_family("Segoe UI").expect("Failed to set default font");
    let _app = UpdaterApp::build_ui(Default::default()).expect("Failed to build UI");

    nwg::dispatch_thread_events();
}
