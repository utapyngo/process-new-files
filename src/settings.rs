use std::fs::File;

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Settings {
    #[serde(default = "default_delay")]
    pub delay: u64,
    pub directories: Vec<String>,
    pub command: String,
    pub extensions: Vec<String>,
    #[serde(default = "default_file_list")]
    pub file_list: String,
    #[serde(default = "default_prefixes")]
    pub prefixes: Vec<String>,
}

fn default_delay() -> u64 {
    5
}

fn default_file_list() -> String {
    "file_list.txt".to_string()
}

fn default_prefixes() -> Vec<String> {
    Vec::new()
}

pub fn load_settings(file_name: &str) -> Settings {
    let settings_file = File::open(file_name).expect("Unable to open settings file");
    serde_yaml::from_reader(settings_file).expect("Unable to parse settings file")
}

pub fn remove_prefix(s: &str, prefixes: &[String]) -> String {
    let mut s = s.split('/').last().unwrap().to_string();
    for prefix in prefixes {
        s = s.replace(prefix, "");
    }
    s
}
