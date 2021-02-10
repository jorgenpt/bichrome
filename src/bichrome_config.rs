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

pub fn read_config_from_file<P: AsRef<Path>>(path: P) -> Result<Configuration, Box<dyn Error>> {
    let file = File::open(path)?;
    let reader = BufReader::new(file);
    let configuration = serde_json::from_reader(reader)?;
    Ok(configuration)
}

#[derive(Debug, Clone)]
pub struct MissingProfileMappingError {
    pub profile: String,
}

impl fmt::Display for MissingProfileMappingError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "invalid first item to double")
    }
}

impl Error for MissingProfileMappingError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        None
    }
}

pub fn generate_config<P: AsRef<Path>>(
    template_path: P,
    path: P,
    domain_profile_mapping: &HashMap<String, String>,
) -> Result<(), Box<dyn Error>> {
    let template_file = File::open(template_path)?;
    let reader = BufReader::new(template_file);
    let template: Template = serde_json::from_reader(reader)?;
    let mut configuration = template.configuration;
    for mut profile_selector in &mut configuration.profile_selection {
        let hosted_domain = template
            .profiles
            .get(&profile_selector.profile)
            .ok_or(MissingProfileMappingError {
                profile: profile_selector.profile.to_owned(),
            })?
            .to_owned();

        profile_selector.profile = domain_profile_mapping
            .iter()
            .find(|(_, v)| **v == hosted_domain)
            .ok_or(MissingProfileMappingError {
                profile: profile_selector.profile.to_owned(),
            })?
            .0
            .to_owned();
    }

    let file = File::create(path)?;
    let writer = BufWriter::new(file);
    serde_json::to_writer(writer, &configuration)?;

    Ok(())
}
