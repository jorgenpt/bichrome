use crate::chrome_local_state;
use regex::Regex;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::convert::TryFrom;
use std::error::Error;
use std::fmt;
use std::fs::File;
use std::io::{BufReader, BufWriter};
use std::path::Path;

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
pub struct ProfilePatterns {
    pub profile: String,
    pub patterns: Vec<Pattern>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Configuration {
    pub profile_selection: Vec<ProfilePatterns>,
}

impl Configuration {
    pub fn empty() -> Configuration {
        Configuration {
            profile_selection: Vec::new(),
        }
    }

    pub fn read_from_file<P: AsRef<Path>>(path: P) -> Result<Configuration, Box<dyn Error>> {
        let file = File::open(path)?;
        let reader = BufReader::new(file);
        let configuration = serde_json::from_reader(reader)?;
        Ok(configuration)
    }

    /// Find the best matching Chrome profile for the given URL.
    /// Returns None if there aren't any matching patterns.
    pub fn choose_profile(&self, url: &str) -> Option<&String> {
        for profile_selector in &self.profile_selection {
            for pattern in &profile_selector.patterns {
                if pattern.is_match(&url) {
                    return Some(&profile_selector.profile);
                }
            }
        }
        None
    }
}

#[derive(Serialize, Deserialize, Debug)]
struct Template {
    profiles: HashMap<String, String>,
    configuration: Configuration,
}

pub fn generate_config<P: AsRef<Path>>(
    template_path: P,
    path: P,
    chrome_profiles_data: &chrome_local_state::ProfilesData,
) -> Result<(), Box<dyn Error>> {
    // Load the template config from JSON
    let template: Template = {
        let template_file = File::open(template_path)?;
        let reader = BufReader::new(template_file);
        serde_json::from_reader(reader)?
    };

    // Create a mapping from placeholder profiles in the template to the appropriate
    // profile from our Google Chrome Local State, silently omitting any entries that
    // don't exist in your local state.
    let domain_map: HashMap<&String, &String> = template
        .profiles
        .iter()
        .filter_map(|(placeholder_name, hosted_domain)| {
            let matching_profiles = chrome_profiles_data.get_profiles(&hosted_domain);
            if let Some(first_matching_profile) = matching_profiles.first() {
                Some((placeholder_name, *first_matching_profile))
            } else {
                None
            }
        })
        .collect();

    // Rebuild the profile_selection from the template config to only contain the entries
    // that we have remappings for, and to use the real profile name
    let remapped_profile_selection = template
        .configuration
        .profile_selection
        .iter()
        .filter_map(|profile_selector| {
            if let Some(remapped_profile) = domain_map.get(&profile_selector.profile) {
                Some(ProfilePatterns {
                    profile: remapped_profile.to_string(),
                    patterns: profile_selector.patterns.clone(),
                })
            } else {
                None
            }
        })
        .collect();

    // Finally construct a config that we can serialize to disk
    let configuration = Configuration {
        profile_selection: remapped_profile_selection,
    };

    let file = File::create(path)?;
    let writer = BufWriter::new(file);
    serde_json::to_writer(writer, &configuration)?;

    Ok(())
}
