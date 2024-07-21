use std::{collections::HashMap, fs::File, io::Read};

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct UrlPair {
    pub name: String,
    pub old_url: String,
    pub new_url: String,
    pub old_headers: Option<HashMap<String, String>>,
    pub new_headers: Option<HashMap<String, String>>,
}

pub fn read_yaml_file(path: &str) -> Result<Vec<UrlPair>, Box<dyn std::error::Error>> {
    let mut file = File::open(path)?;
    let mut contents: String = String::new();
    file.read_to_string(&mut contents)?;
    let url_pairs: Vec<UrlPair> = serde_yaml::from_str(&contents)?;
    Ok(url_pairs)
}
