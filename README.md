# process-new-files

process-new-files is a Rust-based application designed to monitor directories
for new files and perform actions based on the new files detected.
It allows for a high degree of customization through command line arguments and a settings file.

## Features

- Monitor multiple directories for new files.
- Specify file extensions to watch for.
- Override settings using command line arguments.
- Perform actions on new files detected.

## Usage

To use process-new-files, you need to specify the settings in a YAML file or through command line arguments.
The settings include the directories to monitor, the file extensions to watch for, and the command to execute
when new files are detected.

Here is an example of how to use process-new-files with command line arguments:

```bash
cargo run -- -D /path/to/directory -E txt -c "echo New file detected"
```

In this example, process-new-files will monitor the directory `/path/to/directory` for new `txt` files.
When a new file is detected, it will execute the command `echo New file detected`.

## Command Line Arguments

- `-D`, `--directories`: Specify the directories to monitor.
- `-E`, `--extensions`: Specify the file extensions to watch for.
- `-c`, `--command`: Specify the command to execute when new files are detected.
- `-d`, `--delay`: Specify the delay between checks for new files.
- `-f`, `--file_list`: Specify the file to save the list of detected files.
- `-P`, `--prefixes`: Specify the prefixes to remove from the file names in the file list.

## Installation

To install process-new-files, you need to have Rust and Cargo installed on your system. You can then clone the
repository and build the project with Cargo:

```bash
git clone https://github.com/utapyngo/process-new-files.git
cd process-new-files
cargo build --release
```

The built binary will be located in the `target/release` directory.

## Contributing

Contributions to process-new-files are welcome. Please open an issue or submit a pull request on GitHub.
