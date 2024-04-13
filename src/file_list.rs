use std::collections::HashSet;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

use serde::{Deserialize, Serialize};

use crate::processor::Processor;
use crate::settings::remove_prefix;

#[derive(Debug, Serialize, Deserialize)]
pub struct FileList {
    pub files: HashSet<String>,
    pub file_name: String,
    pub prefixes: Vec<String>,
}

impl FileList {
    pub fn load(file_name: &str, prefixes: &[String]) -> Self {
        let files = match File::open(file_name) {
            Ok(file) => {
                let reader = BufReader::new(file);
                reader
                    .lines()
                    .collect::<Result<HashSet<_>, _>>()
                    .unwrap_or_default()
            }
            Err(_) => HashSet::new(),
        };
        Self {
            files,
            file_name: file_name.to_string(),
            prefixes: prefixes.to_owned(),
        }
    }

    pub fn save(&self) {
        let mut file = File::create(&self.file_name).expect("Unable to create file list");
        let mut new_files = self.files.iter().collect::<Vec<&String>>();
        new_files.sort_by_key(|f| remove_prefix(f, &self.prefixes));
        let all_lines = new_files
            .iter()
            .map(|f| f.to_string())
            .collect::<Vec<String>>()
            .join("\n");
        file.write_all(all_lines.as_bytes())
            .expect("Unable to write to file list");
    }

    pub fn update(&mut self, new_files: &Vec<String>, processor: &Processor) -> bool {
        let mut changed = false;
        let mut new_files_to_process = Vec::new();
        for file in new_files {
            if self.files.insert(file.clone()) {
                println!("New file: {}", file);
                new_files_to_process.push(file.clone());
                changed = true;
            }
        }
        if !new_files_to_process.is_empty() {
            processor.process(&new_files_to_process);
        }
        changed
    }
}
