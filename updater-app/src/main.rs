#![cfg_attr(
    all(
      target_os = "windows",
      not(debug_assertions),
    ),
    windows_subsystem = "windows"
  )]

/*!
    A very simple application that show your name in a message box.
    See `basic` for the version without the derive macro
*/

extern crate native_windows_derive as nwd;
extern crate native_windows_gui as nwg;

use std::{
    cell::RefCell,
    collections::VecDeque,
    thread::{self},
};

use nwd::NwgUi;
use nwg::NativeUi;
use stormlib::MpqArchive;

mod api;
mod logic;
mod mpq_writer_thread;

const REALMLIST: &'static str = r#"set realmlist 157.90.144.252"#;

#[derive(Default, NwgUi)]
pub struct UpdaterApp {
    api_ref: api::Api,
    compute: RefCell<Option<thread::JoinHandle<anyhow::Result<VecDeque<String>>>>>,

    #[nwg_resource]
    embed: nwg::EmbedResource,

    #[nwg_control(size: (640, 480), position: (0, 0), title: "Updater App", flags: "WINDOW|VISIBLE", icon: None)]
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
    #[nwg_events( OnButtonClick: [UpdaterApp::play] )]
    play_button: nwg::Button,

    #[nwg_control( position: (10, 70), size: (620, 50))]
    update_progress: nwg::ProgressBar,

    #[nwg_control(text: "Install", font: Some(&data.btn_font), position: (10, 70), size: (620, 50))]
    #[nwg_events( OnButtonClick: [UpdaterApp::update_or_install] )]
    update_or_install_button: nwg::Button,

    #[nwg_control(text: "Disable", font: Some(&data.btn_font), position: (10, 130), size: (620, 50))]
    #[nwg_events( OnButtonClick: [UpdaterApp::enable_or_disable] )]
    enable_or_disable_button: nwg::Button,

    #[nwg_control(text: "Uninstall", font: Some(&data.btn_font), position: (10, 190), size: (620, 50))]
    #[nwg_events( OnButtonClick: [UpdaterApp::uninstall] )]
    uninstall_button: nwg::Button,

    #[nwg_control(text: "Loading..", font: Some(&data.txt_font), position: (10, 250), size: (620, 50))  ]
    current_status_label: nwg::Label,
}

impl UpdaterApp {
    fn on_mount(&self) {
        let icon = self.embed.icon(2, None);
        self.window.set_icon(icon.as_ref());
        self.update_progress.set_visible(false);
        let cwd = std::env::current_dir().expect("Failed to get current directory");
        logic::check_has_wow_exe(&cwd);
        logic::check_has_data_dir(&cwd);
        mpq_writer_thread::close();

        let config = logic::read_updater_app_cfg();

        if config.installed {
            self.update_or_install_button.set_text("Update");
            self.play_button.set_enabled(true);
            self.uninstall_button.set_enabled(true);
            self.current_status_label.set_text(&format!(
                "Current version: v{}",
                0.1 * config.current_release as f32
            ));
            self.enable_or_disable_button.set_enabled(true);
        } else {
            self.enable_or_disable_button.set_enabled(false);
            self.update_or_install_button.set_text("Install");
            self.play_button.set_enabled(false);
            self.uninstall_button.set_enabled(false);
            self.current_status_label
                .set_text("Press the Install button to continue");
        }

        if config.disabled {
            self.enable_or_disable_button.set_text("Enable");
            self.update_or_install_button.set_enabled(false);
        } else {
            self.enable_or_disable_button.set_text("Disable");
            self.update_or_install_button.set_enabled(true);
        }
    }

    fn on_exit(&self) {
        nwg::stop_thread_dispatch();
        mpq_writer_thread::exit();
    }

    fn enable_or_disable(&self) {
        let mut config = logic::read_updater_app_cfg();
        if config.disabled {
            // run enable
            let mpq = logic::get_mpq_path();
            let disabled_mpq = mpq.with_file_name("patch-A.mpq");
            std::fs::rename(mpq, disabled_mpq).expect("Failed to enable CustomMods");
            let prior_realmlist = logic::set_realmlist_content(&REALMLIST);
            if let Some(realmlist) = prior_realmlist {
                config.prior_realmlist = Some(realmlist);
            }
        } else {
            // run disable
            let mpq = logic::get_mpq_path();
            let disabled_mpq = mpq.with_file_name(".disabled.mpq");
            std::fs::rename(mpq, disabled_mpq).expect("Failed to disable CustomMods");
            if let Some(ref realmlist) = config.prior_realmlist {
                logic::set_realmlist_content(&realmlist);
            }

        }
        config.disabled = !config.disabled;
        logic::write_updater_app_cfg(&config);

        self.on_mount();
    }

    fn uninstall(&self) {
        let current_config = logic::read_updater_app_cfg();
        if current_config.installed {
            let _ = std::fs::remove_file(logic::get_mpq_path());
        }
        logic::write_updater_app_cfg(&logic::Config::default());
        self.on_mount();
    }

    fn play(&self) {
        let wow_exe = logic::get_wow_exe();
        std::process::Command::new(wow_exe)
            .spawn()
            .expect("Failed to start WoW");
    }

    fn update_or_install(&self) {
        let config = logic::read_updater_app_cfg();
        if config.installed {
            self.update(config);
        } else {
            self.install(config);
        }
    }

    fn update(&self, mut config: logic::Config) {
        self.current_status_label.set_text("Checking for updates..");
        match self.api_ref.get_releases(config.current_release + 1) {
            Ok(releases) => {
                if releases.is_empty() {
                    println!("No updates found");
                    self.current_status_label.set_text("No updates found");
                    return;
                }
                config.latest_release = releases.last().unwrap().id;
                logic::write_updater_app_cfg(&config);
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

    fn install(&self, mut config: logic::Config) {
        self.current_status_label.set_text("Creating MPQ archive");
        let archive = stormlib::MpqArchive::new("data/patch-A.mpq");
        match archive {
            Ok(archive) => {
                drop(archive); // release the lock on the MPQ file
                self.current_status_label
                    .set_text("MPQ archive initialized");
                config.installed = true;
                config.current_release = config.latest_release;
                logic::write_updater_app_cfg(&config);
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
        mpq_writer_thread::open();
        let sender = self.notice.sender();
        let api = self.api_ref.clone();

        self.compute.borrow_mut().replace(thread::spawn(move || {
            let file_path = files_to_download
                .pop_front()
                .expect("Called download_updates on empty queue?");
            println!("Downloading file: {}", file_path);

            let file_content = api.get_file(&file_path)?;
            println!("Downloaded file: {}", file_path);

           mpq_writer_thread::write_file(&file_path, file_content);
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
                            let mut cfg = logic::read_updater_app_cfg();
                            cfg.current_release = cfg.latest_release;
                            logic::write_updater_app_cfg(&cfg);
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
    nwg::init().expect("Failed to init Native Windows GUI");
    nwg::Font::set_global_family("Segoe UI").expect("Failed to set default font");
    let _app = UpdaterApp::build_ui(Default::default()).expect("Failed to build UI");
    nwg::dispatch_thread_events();
}
