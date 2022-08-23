mod command;
mod host;
mod disp;

use command::Command;

fn main() {
    let res = run();
    if res.is_err() {
        println!("{}", res.unwrap_err());
        std::process::exit(1);
    }
}

// Returns nothing or error
fn run() -> Result<(), String> {
    let args: Vec<String> = std::env::args().collect();
    if args.len() == 1 {
        // TODO: Display help
        println!("Not enough arguments");
        std::process::exit(1);
    }
    let command = Command::new(args[1].clone(), args[2..].to_vec());
    command.execute();
    Ok(())
}
