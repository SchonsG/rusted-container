use std::process::Command;

pub struct Command {
    command: String,
    args: Vec<String>,
}

impl Command {
    pub fn new(command: &str) -> Self {
        Self {
            command: command.to_string(),
            args: Vec::new(),
        }
    }

    pub fn arg(mut self, arg: &str) -> Self {
        self.args.push(arg.to_string());
        return self
    }

    pub fn execute(self) -> String {
        let output = Command::new(self.command)
            .args(self.args)
            .output()
            .expect("Failed to execute command");

        return String::from_utf8_lossy(&output.stdout).to_string();
    }
}
