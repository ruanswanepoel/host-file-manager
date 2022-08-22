
fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() == 1 {
        // TODO: Display help
        println!("Not enough arguments");
        std::process::exit(1);
    }
    handle_args(args);
}

fn handle_args(args: Vec<String>) {
    let _result = match args[1].as_str() {
        "list" => list_hosts(),
        _ => {
            println!("Unknown command");
            std::process::exit(1);
        }
    };
}

fn list_hosts() {
    // Read the hosts file
    let data = std::fs::read_to_string("C:/Windows/System32/drivers/etc/hosts").expect("Unable to read file");
    // Filter out comments and empty lines
    let lines = data.lines().filter(|l| !l.starts_with("#") && !l.is_empty());
    // Print contents
    println!("{}", lines.collect::<Vec<&str>>().join("\n"));
}
