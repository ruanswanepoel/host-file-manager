use crate::host::Host;

pub fn list_hosts() {
    // Read the hosts file
    let data = std::fs::read_to_string("C:/Windows/System32/drivers/etc/hosts").expect("Unable to read file");
    // Filter out comments and empty lines
    let lines = data.lines().filter(|l| !l.starts_with("#") && !l.is_empty());
    // Parse each line into a Host struct
    // TODO: Move this code to a parser
    let hosts = lines.map(|l| {
        let parts: Vec<&str> = l.split(" ").collect();
        Host::new(parts[0].to_string(), parts[1].to_string())
    });
    // Print contents
    println!("+{:-^17}+{:-^50}+", "", "");
    hosts.for_each(|h| println!("| {:<15} | {:<48} |", h.ip, h.hostname));
    println!("+{:-^17}+{:-^50}+", "", "");
}
