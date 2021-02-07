use serde::{Deserialize, Serialize};

use std::collections::HashMap;
use std::error::Error;
use std::fs::File;
use std::io::BufReader;
use std::path::Path;

#[derive(Serialize, Deserialize, Debug)]
pub struct ChromeProfile {
    pub hosted_domain: String,
}

#[derive(Serialize, Deserialize)]
struct ProfilesData {
    info_cache: HashMap<String, ChromeProfile>,
}

#[derive(Serialize, Deserialize)]
struct State {
    profile: ProfilesData,
}

pub fn read_profiles_from_file<P: AsRef<Path>>(
    path: P,
) -> Result<HashMap<String, ChromeProfile>, Box<dyn Error>> {
    let file = File::open(path)?;
    let reader = BufReader::new(file);
    let state: State = serde_json::from_reader(reader)?;

    Ok(state.profile.info_cache)
}
