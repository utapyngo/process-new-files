use std::process::Command;

pub struct Processor {
    pub command: String,
}

impl Processor {
    pub fn process(&self, files: &Vec<String>) {
        let mut command = self.prepare_command();
        for file in files {
            command.arg(file);
        }
        command.spawn().expect("Failed to process files");
    }

    fn prepare_command(&self) -> Command {
        let mut command_line = self.command.split_whitespace();
        if let Some(first) = command_line.next() {
            let mut command = Command::new(first);
            for arg in command_line {
                command.arg(arg);
            }
            command
        } else {
            panic!("Command is empty");
        }
    }
}
