#![allow(dead_code)]

use log::trace;
use webextension_pattern::Pattern;

use crate::{
    chrome_local_state::{self, read_profiles_from_file},
    os::get_chrome_local_state_path,
};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs::File;
use std::io::BufReader;
use std::path::Path;
use thiserror::Error;
use url::Url;

#[derive(Error, Debug)]
pub enum Error {
    #[error("could not read configuration file")]
    InvalidFile(#[source] std::io::Error),
    #[error("could not parse configuration file")]
    InvalidJson(#[source] serde_json::Error),
    #[error("could not find declaration of profile {0}")]
    MissingProfile(String),
    #[error("unable to retrieve path for Chrome's Local State")]
    CantLocateChromeLocalState,
    #[error("unable to parse Chrome's Local State")]
    CantParseChromeLocalState(#[source] chrome_local_state::Error),
    #[error("no profile in Chrome's Local State matched domain '{0}' specified in config")]
    InvalidHostedDomain(String),
    #[error("no profile in Chrome's Local State matched name '{0}' specified in config")]
    InvalidProfileName(String),
    #[error("failed to parse received url {0:?}")]
    InvalidUrlPassedIn(String, #[source] url::ParseError),
}

type Result<T> = std::result::Result<T, Error>;

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(untagged)]
pub enum ChromeProfile {
    ByName {
        #[serde(rename = "profile")]
        name: String,
    },
    ByHostedDomain {
        hosted_domain: String,
    },
    None {},
}

impl ChromeProfile {
    pub fn get_argument(&self) -> Result<Option<String>> {
        let local_state_path =
            get_chrome_local_state_path().ok_or(Error::CantLocateChromeLocalState)?;
        let profiles =
            read_profiles_from_file(local_state_path).map_err(Error::CantParseChromeLocalState)?;
        trace!("Found Chrome profiles: {profiles:?}");

        match self {
            ChromeProfile::ByName { name } => {
                if let Some(profile) = profiles.profile_by_name(name) {
                    Ok(Some(format!("--profile-directory={}", profile)))
                } else {
                    Err(Error::InvalidProfileName(name.to_owned()))
                }
            }
            ChromeProfile::ByHostedDomain { hosted_domain } => {
                let matching_profiles = profiles.profiles_by_hosted_domain(hosted_domain);
                if matching_profiles.is_empty() {
                    Err(Error::InvalidHostedDomain(hosted_domain.to_owned()))
                } else {
                    Ok(Some(format!(
                        "--profile-directory={}",
                        matching_profiles[0].clone()
                    )))
                }
            }
            ChromeProfile::None {} => Ok(None),
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(untagged)]
pub enum EdgeProfile {
    ByName {
        #[serde(rename = "profile")]
        name: String,
    },
    None {},
}

impl EdgeProfile {
    pub fn get_argument(&self) -> Result<Option<String>> {
        match self {
            EdgeProfile::ByName { name } => Ok(Some(format!("--profile-directory={}", name))),
            EdgeProfile::None {} => Ok(None),
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(tag = "browser")]
pub enum Browser {
    Chrome(ChromeProfile),
    Firefox,
    OsDefault,
    Edge(EdgeProfile),
    Safari,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ProfilePattern {
    pub profile: String,
    pub pattern: Pattern,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Configuration {
    pub default_profile: Option<String>,
    pub profiles: HashMap<String, Browser>,
    pub profile_selection: Vec<ProfilePattern>,
}

impl Configuration {
    pub fn empty() -> Configuration {
        Configuration {
            default_profile: None,
            profiles: HashMap::new(),
            profile_selection: Vec::new(),
        }
    }

    pub fn read_from_file<P: AsRef<Path>>(path: P) -> Result<Configuration> {
        let file = File::open(path).map_err(Error::InvalidFile)?;
        let reader = BufReader::new(file);
        let configuration = serde_json::from_reader(reader).map_err(Error::InvalidJson)?;
        Ok(configuration)
    }

    fn get_profile(&self, profile_name: &str) -> Result<&Browser> {
        for (profile, browser) in &self.profiles {
            if profile == profile_name {
                return Ok(browser);
            }
        }

        Err(Error::MissingProfile(profile_name.to_string()))
    }

    /// Find the best matching browser profile for the given URL.
    pub fn choose_browser(&self, url: &str) -> Result<Browser> {
        let url = Url::parse(url).map_err(|err| Error::InvalidUrlPassedIn(url.to_string(), err))?;

        for profile_selector in &self.profile_selection {
            if profile_selector.pattern.is_match(&url) {
                return self
                    .get_profile(&profile_selector.profile)
                    .map(|b| b.clone());
            }
        }

        // If there's a default_profile, use that, otherwise default to a Chrome without profiles.
        if let Some(default_profile) = &self.default_profile {
            self.get_profile(default_profile).map(|b| b.clone())
        } else {
            Ok(Browser::Chrome(ChromeProfile::None {}))
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
struct Template {
    profiles: HashMap<String, String>,
    configuration: Configuration,
}
