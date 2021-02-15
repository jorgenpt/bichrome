#![allow(dead_code)]

use regex::Regex;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::convert::TryFrom;
use std::error::Error;
use std::fmt;
use std::fs::File;
use std::io::BufReader;
use std::path::Path;

#[derive(Debug)]
pub struct MissingProfileError(String);
impl fmt::Display for MissingProfileError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "profile not declared in config: {}", self.0)
    }
}
impl Error for MissingProfileError {}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(untagged)]
pub enum ChromeProfile {
    ByName { name: String },
    ByHostedDomain { hosted_domain: String },
    None {},
}

impl ChromeProfile {
    pub fn get_argument(&self) -> Option<String> {
        match self {
            ChromeProfile::ByName { name } => Some(format!("--profile-directory={}", name)),
            ChromeProfile::ByHostedDomain { hosted_domain: _ } => {
                panic!("not implemented"); // TODO Implement lookup through chrome_local_state
            }
            ChromeProfile::None {} => None,
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

#[serde(try_from = "String", into = "String")]
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Pattern {
    raw: String,
    compiled: Regex,
}

impl Pattern {
    pub fn is_match(&self, url: &str) -> bool {
        self.compiled.is_match(url)
    }
}

impl Into<String> for Pattern {
    fn into(self) -> String {
        self.raw
    }
}

impl TryFrom<String> for Pattern {
    type Error = regex::Error;

    fn try_from(raw: String) -> Result<Self, Self::Error> {
        let compiled = Regex::new(&raw)?;
        Ok(Pattern { compiled, raw })
    }
}

impl fmt::Display for Pattern {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:#?}", self.raw)
    }
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

    pub fn read_from_file<P: AsRef<Path>>(path: P) -> Result<Configuration, Box<dyn Error>> {
        let file = File::open(path)?;
        let reader = BufReader::new(file);
        let configuration = serde_json::from_reader(reader)?;
        Ok(configuration)
    }

    fn get_profile(&self, profile_name: &str) -> Result<&Browser, MissingProfileError> {
        for (profile, browser) in &self.profiles {
            if profile == profile_name {
                return Ok(browser);
            }
        }

        Err(MissingProfileError(profile_name.to_string()))
    }

    /// Find the best matching browser profile for the given URL.
    pub fn choose_browser(&self, url: &str) -> Result<Browser, MissingProfileError> {
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
