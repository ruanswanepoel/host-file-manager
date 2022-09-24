use std::fs;
use std::io::prelude::*;
use crate::host::Host;

pub fn read_hosts() -> Vec<Host> {
    
    // Read the hosts file
    // let data = fs::read_to_string("./test_host_file").expect("Unable to read file");
    let data = std::fs::read_to_string("C:/Windows/System32/drivers/etc/hosts").expect("Unable to read file");

    // Filter out comments and empty lines
    let lines = data.lines().filter(|l| !l.starts_with("#") && !l.is_empty());

    // Parse each line into a Host struct
    let hosts: Vec<Host> = lines.enumerate().map(|(i, l)| {
        let parts: Vec<&str> = l.split(" ").collect();
        Host::new(i, parts[0].to_string(), parts[1].to_string())
    }).collect();

    hosts

}

pub fn add_host(host: Host) {

    let mut file = fs::OpenOptions::new()
        .append(true)
        // .open("./test_host_file")
        .open("C:/Windows/System32/drivers/etc/hosts")
        .expect("Unable to open file.");

    if let Err(e) = writeln!(file, "{}", host.to_string()) {
        eprintln!("Could not write to file: {}", e);
    }

}
