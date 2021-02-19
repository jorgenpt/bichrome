#![allow(dead_code)]

use webextension_pattern::Pattern;

use crate::{chrome_local_state::read_profiles_from_file, os::get_chrome_local_state_path};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::error::Error as StdError;
use std::fmt;
use std::fs::File;
use std::io::BufReader;
use std::path::Path;
use std::result::Result as StdResult;
use url::Url;

#[derive(Debug, Clone, PartialEq)]
pub enum ErrorKind {
    MissingProfile,
    CantLocateChromeLocalState,
    InvalidHostedDomain,
    InvalidUrlPassedIn(url::ParseError),
}

#[derive(Debug)]
pub struct Error {
    /// Formated error message
    pub message: String,
    /// The type of error
    pub kind: ErrorKind,
}

impl Error {
    fn new(kind: ErrorKind, message: String) -> Self {
        Error {
            kind,
            message: message,
        }
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.message)
    }
}
impl StdError for Error {}

type Result<T> = StdResult<T, Error>;

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
    pub fn get_argument(&self) -> StdResult<Option<String>, Box<dyn StdError>> {
        match self {
            ChromeProfile::ByName { name } => Ok(Some(format!("--profile-directory={}", name))),
            ChromeProfile::ByHostedDomain { hosted_domain } => {
                let local_state_path = get_chrome_local_state_path().ok_or(Error::new(
                    ErrorKind::CantLocateChromeLocalState,
                    format!("unable to retrieve path for Chrome's Local State"),
                ))?;
                let profiles = read_profiles_from_file(local_state_path)?;
                let matching_profiles = profiles.get_profiles(hosted_domain);
                if matching_profiles.is_empty() {
                    Err(Box::new(Error::new(
                        ErrorKind::InvalidHostedDomain,
                        format!(
                            "no profile in Chrome's Local State matched '{}' specified in config",
                            hosted_domain
                        ),
                    )))
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
#[serde(tag = "browser")]
pub enum Browser {
    Chrome(ChromeProfile),
    Firefox,
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

    pub fn read_from_file<P: AsRef<Path>>(path: P) -> StdResult<Configuration, Box<dyn StdError>> {
        let file = File::open(path)?;
        let reader = BufReader::new(file);
        let configuration = serde_json::from_reader(reader)?;
        Ok(configuration)
    }

    fn get_profile(&self, profile_name: &str) -> Result<&Browser> {
        for (profile, browser) in &self.profiles {
            if profile == profile_name {
                return Ok(browser);
            }
        }

        Err(Error::new(
            ErrorKind::MissingProfile,
            format!("could not find declaration of profile {}", &profile_name),
        ))
    }

    /// Find the best matching browser profile for the given URL.
    pub fn choose_browser(&self, url: &str) -> Result<Browser> {
        let url = Url::parse(url).map_err(|err| {
            Error::new(
                ErrorKind::InvalidUrlPassedIn(err),
                format!("could not parse provided URL {}", url),
            )
        })?;

        for profile_selector in &self.profile_selection {
            if profile_selector.pattern.is_match(&url) {
                return self
                    .get_profile(&profile_selector.profile)
                    .map(|b| b.clone());
            }
        }

        // If there's a default_profile, use that, otherwise default to a Chrome without profiles.
        if let Some(default_profile) = &self.default_profile {
            self.get_profile(&default_profile).map(|b| b.clone())
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
