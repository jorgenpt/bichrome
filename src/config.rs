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
    pub pattern: Pattern,
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
            if profile_selector.pattern.is_match(&url) {
                return Some(&profile_selector.profile);
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
