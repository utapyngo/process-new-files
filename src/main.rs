use std::thread;
use std::time::Duration;

use clap::Parser;

use crate::settings::remove_prefix;
use directory::Directory;
use file_list::FileList;
use processor::Processor;
use settings::load_settings;

mod directory;
mod file_list;
mod processor;
mod settings;

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
    let processor = Processor {
        command: settings.command,
    };

    loop {
        thread::sleep(Duration::from_secs(settings.delay));
        let mut new_files = Vec::new();
        for directory in &settings.directories {
            let directory = Directory {
                path: directory.clone(),
                extensions: settings.extensions.clone(),
            };
            new_files.extend(directory.get_files());
        }
        new_files.sort_by_key(|f| remove_prefix(f, &settings.prefixes));
        if file_list.update(&new_files, &processor) {
            file_list.save();
        }
    }
}
