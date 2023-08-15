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
    name: Option<String>,
    shortcut_name: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ProfilesData {
    info_cache: HashMap<String, ChromeProfile>,
}

impl ProfilesData {
    pub fn profiles_by_hosted_domain(&self, hosted_domain: &str) -> Vec<&String> {
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
    pub fn profile_by_name(&self, name: &str) -> Option<&str> {
        // Prefer direct profile name matches
        if let Some((profile_name, _)) = self.info_cache.get_key_value(name) {
            return Some(profile_name);
        }

        let matched_name = Some(name.to_owned());
        let found = self.info_cache.iter().find(|(_, profile)| {
            profile.name == matched_name || profile.shortcut_name == matched_name
        });

        if let Some((profile_name, _)) = found {
            Some(profile_name)
        } else {
            None
        }
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
