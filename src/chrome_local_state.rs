#![allow(dead_code)]

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs::File;
use std::io::BufReader;
use std::path::Path;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error("could not read Chrome Local State")]
    InvalidFile(#[source] std::io::Error),
    #[error("could not parse Chrome Local State")]
    InvalidJson(#[source] serde_json::Error),
}

type Result<T> = std::result::Result<T, Error>;

#[derive(Serialize, Deserialize, Debug)]
pub struct ChromeProfile {
    hosted_domain: String,
}

#[derive(Serialize, Deserialize, Debug)]
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

pub fn read_profiles_from_file<P: AsRef<Path>>(path: P) -> Result<ProfilesData> {
    let state: State = {
        let file = File::open(path).map_err(Error::InvalidFile)?;
        let reader = BufReader::new(file);
        serde_json::from_reader(reader).map_err(Error::InvalidJson)?
    };

    Ok(state.profile)
}
