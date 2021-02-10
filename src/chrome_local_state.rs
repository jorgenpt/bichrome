use serde::{Deserialize, Serialize};

use std::collections::HashMap;
use std::error::Error;
use std::fs::File;
use std::io::BufReader;
use std::path::Path;

#[derive(Serialize, Deserialize, Debug)]
pub struct ChromeProfile {
    hosted_domain: String,
}

#[derive(Serialize, Deserialize)]
pub struct ProfilesData {
    info_cache: HashMap<String, ChromeProfile>,
}

impl ProfilesData {
    pub fn get_profiles(&self, hosted_domain: &str) -> Vec<&String> {
        self.info_cache
            .iter()
            .filter_map(|(profile_name, profile)| {
                if profile.hosted_domain == hosted_domain {
                    Some(profile_name)
                } else {
                    None
                }
            })
            .collect()
    }
}

#[derive(Serialize, Deserialize)]
struct State {
    profile: ProfilesData,
}

pub fn read_profiles_from_file<P: AsRef<Path>>(path: P) -> Result<ProfilesData, Box<dyn Error>> {
    let state: State = {
        let file = File::open(path)?;
        let reader = BufReader::new(file);
        serde_json::from_reader(reader)?
    };

    Ok(state.profile)
}
