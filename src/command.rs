use crate::disp;

pub struct Command {
    pub name: String,
    pub args: Vec<String>,
}

impl Command {
    pub fn new(name: String, args: Vec<String>) -> Command {
        Command {
            name,
            args,
        }
    }

    pub fn execute(&self) {
        match self.name.as_str() {
            "list" => disp::list_hosts(),
            _ => {
                println!("Unknown command");
                std::process::exit(1);
            }
        }
    }
}
