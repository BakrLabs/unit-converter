pub struct History {
    commands: Vec<String>,
}

impl History {
    pub fn new() -> Self {
        Self {
            commands: Vec::new(),
        }
    }

    pub fn add(&mut self, command: String) {
        self.commands.push(command);
    }

    pub fn show(&self) {
        if self.commands.is_empty() {
            println!("No history");
            return;
        }

        for (index, command) in self.commands.iter().enumerate() {
            println!("{}: {}", index + 1, command);
        }
    }
}
