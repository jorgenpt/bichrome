#![deny(clippy::all)]
#![forbid(unsafe_code)]

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
    let config = bichrome_config::read_config_from_file("bichrome_config.json").unwrap();
    println!("config {:#?}", config);

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
