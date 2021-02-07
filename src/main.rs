#![deny(clippy::all)]
#![forbid(unsafe_code)]

mod bichrome_config;
mod chrome_local_state;

use std::env;

fn choose_profile<'a>(config: &'a bichrome_config::Configuration, url: &str) -> Option<&'a String> {
    for profile_selector in &config.profile_selection {
        for pattern in &profile_selector.patterns {
            if pattern.matches(&url) {
                return Some(&profile_selector.profile);
            }
        }
    }

    config
        .profile_selection
        .last()
        .map(|selector| &selector.profile)
}

fn main() {
    let _chrome_local_state_path =
        "/Users/jorgenpt/Library/Application Support/Google/Chrome/Local State";
    let chrome_local_state_path =
        r"C:\Users\jorgenpt\AppData\Local\Google\Chrome\User Data\Local State";
    let profiles =
        chrome_local_state::read_chrome_profiles_from_file(chrome_local_state_path).unwrap();

    println!("profiles {:#?}", profiles);

    let config = bichrome_config::read_config_from_file("bichrome_config.json").unwrap();
    println!("config {:#?}", config);

    for url in env::args() {
        if let Some(profile_name) = choose_profile(&config, &url) {
            println!("{:#?}: {:#?}", url, profile_name);
        }
    }
}
