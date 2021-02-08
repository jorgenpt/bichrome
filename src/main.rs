#![deny(clippy::all)]
#![forbid(unsafe_code)]
// We use the console subsystem in debug builds, but use the Windows subsystem in release
// builds so we don't have to allocate a console and pop up a command line window.
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
#![cfg_attr(debug_assertions, windows_subsystem = "console")]

mod bichrome_config;
mod chrome_local_state;

use std::env;
use std::process::{Command, Stdio};

// Chrome Local State:
//  - macOS path: "/Users/jorgenpt/Library/Application Support/Google/Chrome/Local State";
//  - Windows path: r"C:\Users\jorgenpt\AppData\Local\Google\Chrome\User Data\Local State";

fn choose_profile<'a>(config: &'a bichrome_config::Configuration, url: &str) -> Option<&'a String> {
    for profile_selector in &config.profile_selection {
        for pattern in &profile_selector.patterns {
            if pattern.is_match(&url) {
                return Some(&profile_selector.profile);
            }
        }
    }

    None
}

const CHROME_EXE_PATH: &str = r"C:\Program Files (x86)\Google\Chrome\Application\chrome.exe";

fn main() {
    // TODO: Use profiles + a distribution config to generate bichrome_config if it doesn't exist.
    // TODO: Read profile from %localappdata%.
    let config = bichrome_config::read_config_from_file("bichrome_config.json");
    let config = match config {
        Ok(config) => {
            println!("loaded config {:#?}", config);
            config
        }
        Err(e) => {
            println!("failed to parse config: {:?}", e);
            println!("opening URLs without profile");
            bichrome_config::Configuration::empty()
        }
    };

    for url in env::args().skip(1) {
        let mut chrome = Command::new(CHROME_EXE_PATH);
        chrome
            .stdout(Stdio::null())
            .stdin(Stdio::null())
            .stderr(Stdio::null());

        if let Some(profile_name) = choose_profile(&config, &url) {
            chrome.arg(format!("--profile-directory={}", profile_name));
        }

        chrome.arg(url).spawn().unwrap();
    }
}
