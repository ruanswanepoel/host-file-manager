use crate::config;
use crate::host::Host;
use std::fs;
use std::io::prelude::*;

pub fn read_hosts() -> Vec<Host> {
    // Read the hosts file
    let data = std::fs::read_to_string(config::HOSTS_FILE_PATH).expect("Unable to read file");

    let mut hosts: Vec<Host> = vec![];

    for (n, l) in data.lines().enumerate() {
        if (l.starts_with("#") || l.is_empty()) {
            continue;
        }
        let parts: Vec<&str> = l.split(" ").collect();
        let host = Host::new(hosts.len(), parts[0].to_string(), parts[1].to_string(), n);
        hosts.push(host);
    }

    hosts
}

pub fn add_host(host: Host) {
    let mut file = fs::OpenOptions::new()
        .append(true)
        .open(config::HOSTS_FILE_PATH)
        .expect("Unable to open file.");

    if let Err(e) = writeln!(file, "\n{}", host.to_string()) {
        eprintln!("Could not write to file: {}", e);
    }
}

pub fn remove_host(id: usize) {
    let hosts: Vec<Host> = read_hosts();
    let host: &Host = hosts.iter().filter(|h| h.id == id).collect::<Vec<&Host>>()[0];

    let data = std::fs::read_to_string(config::HOSTS_FILE_PATH).expect("Unable to read file");
    let mut lines: Vec<&str> = data.lines().collect();

    lines.remove(host.line_number);

    std::fs::write(config::HOSTS_FILE_PATH, lines.join("\n"));
}
