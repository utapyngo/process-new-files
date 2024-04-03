use std::collections::HashSet;
use std::fs;
use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;
use std::process::Command;
use std::thread;
use std::time::Duration;
use clap::Parser;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
struct Settings {
    #[serde(default = "default_delay")]
    delay: u64,
    directories: Vec<String>,
    command: String,
    extensions: Vec<String>,
    #[serde(default = "default_file_list")]
    file_list: String,
    #[serde(default = "default_prefixes")]
    prefixes: Vec<String>,
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

struct Processor {
    command: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct FileList {
    files: HashSet<String>,
    file_name: String,
    prefixes: Vec<String>,
}

struct Directory {
    path: String,
    extensions: Vec<String>,
}

impl Processor {
    fn process(&self, files: &Vec<String>) {
        let mut command = self.prepare_command();
        for file in files {
            command.arg(file);
        }
        command.spawn().expect("Failed to process files");
    }

    fn prepare_command(&self) -> Command {
        let mut command_line = self.command.split_whitespace();
        let command = if let Some(first) = command_line.next() {
            let mut command = Command::new(first);
            for arg in command_line {
                command.arg(arg);
            }
            command
        } else {
            panic!("Command is empty");
        };
        command
    }
}

impl FileList {
    fn load(file_name: &str, prefixes: &Vec<String>) -> Self {
        let files = match File::open(file_name) {
            Ok(file) => {
                let reader = BufReader::new(file);
                reader.lines().collect::<Result<HashSet<_>, _>>().unwrap_or_default()
            }
            Err(_) => HashSet::new(),
        };
        Self { files, file_name: file_name.to_string(), prefixes: prefixes.clone() }
    }

    fn save(&self) {
        let mut file = File::create(&self.file_name).expect("Unable to create file list");
        let mut new_files = self.files.iter().collect::<Vec<&String>>();
        new_files.sort_by_key(|f| remove_prefix(f, &self.prefixes));
        let all_lines = new_files.iter().map(|f| f.to_string()).collect::<Vec<String>>().join("\n");
        file.write_all(all_lines.as_bytes()).expect("Unable to write to file list");
    }

    fn update(&mut self, new_files: &Vec<String>, processor: &Processor) -> bool {
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

impl Directory {
    fn get_files(&self) -> Vec<String> {
        let mut files = Vec::new();
        if let Ok(entries) = fs::read_dir(&self.path) {
            for entry in entries {
                if let Ok(entry) = entry {
                    let path = entry.path();
                    if path.is_file() {
                        if let Some(extension) = path.extension() {
                            if self.extensions.contains(&extension.to_str().unwrap().to_string()) {
                                files.append(&mut vec![path.to_str().unwrap().to_string()]);
                            }
                        }
                    } else if path.is_dir() {
                        let sub_files = Directory { path: path.to_str().unwrap().to_string(), extensions: self.extensions.clone() }.get_files();
                        files.extend(sub_files);
                    }
                }
            }
        }
        files
    }
}

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(short, long, default_value = "settings.yaml")]
    settings: String,
    #[arg(short = 'c', long)]
    command: Option<String>,
    #[arg(short = 'd', long)]
    delay: Option<u64>,
    #[arg(short = 'D', long)]
    directories: Vec<String>,
    #[arg(short = 'E', long)]
    extensions: Vec<String>,
    #[arg(short = 'f', long)]
    file_list: Option<String>,
    #[arg(short = 'P', long)]
    prefixes: Vec<String>,
}

fn main() {
    let args = Args::parse();
    println!("Loading settings from {}", args.settings);
    let mut settings = load_settings(&args.settings);
    if let Some(command) = args.command {
        settings.command = command;
    }
    if let Some(delay) = args.delay {
        settings.delay = delay;
    }
    if !args.directories.is_empty() {
        settings.directories = args.directories;
    }
    if !args.extensions.is_empty() {
        settings.extensions = args.extensions;
    }
    if let Some(file_list) = args.file_list {
        settings.file_list = file_list;
    }
    if !args.prefixes.is_empty() {
        settings.prefixes = args.prefixes;
    }
    let mut file_list = FileList::load(&settings.file_list, &settings.prefixes);
    let processor = Processor { command: settings.command };

    loop {
        thread::sleep(Duration::from_secs(settings.delay));
        let mut new_files = Vec::new();
        for directory in &settings.directories {
            let directory = Directory { path: directory.clone(), extensions: settings.extensions.clone() };
            new_files.extend(directory.get_files());
        }
        new_files.sort_by_key(|f| remove_prefix(f, &settings.prefixes));
        if file_list.update(&new_files, &processor) {
            file_list.save();
        }
    }
}

fn remove_prefix(s: &str, prefixes: &[String]) -> String {
    let mut s = s.split('/').last().unwrap().to_string();
    for prefix in prefixes {
        s = s.replace(prefix, "");
    }
    s
}

fn load_settings(file_name: &str) -> Settings {
    let settings_file = File::open(file_name).expect("Unable to open settings file");
    serde_yaml::from_reader(settings_file).expect("Unable to parse settings file")
}