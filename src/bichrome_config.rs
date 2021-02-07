use regex::Regex;
use serde::{Deserialize, Serialize};
use std::error::Error;
use std::fmt;
use std::fs::File;
use std::io::BufReader;
use std::path::Path;

#[serde(from = "String", into = "String")]
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

impl From<String> for Pattern {
    fn from(raw: String) -> Self {
        Pattern {
            compiled: Regex::new(&raw).unwrap(),
            raw,
        }
    }
}

impl fmt::Display for Pattern {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:#?}", self.raw)
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ProfilePatterns {
    pub profile: String,
    pub patterns: Vec<Pattern>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Configuration {
    pub profile_selection: Vec<ProfilePatterns>,
}

pub fn read_config_from_file<P: AsRef<Path>>(path: P) -> Result<Configuration, Box<dyn Error>> {
    let file = File::open(path)?;
    let reader = BufReader::new(file);
    let configuration = serde_json::from_reader(reader)?;
    Ok(configuration)
}
