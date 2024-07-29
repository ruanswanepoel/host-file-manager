#![allow(unused)]

mod config;
mod disp;
mod host;
mod host_file;

use clap::arg;
use clap::{App, Arg, Command};
use host::Host;

fn main() {
    let matches = App::new("hfm")
        .version(env!("CARGO_PKG_VERSION"))
        .about("A simple hosts file manager")
        .subcommand(
            Command::new("open")
                .alias("o")
                .about("Open your hosts file"),
        )
        .subcommand(
            Command::new("list")
                .alias("ls")
                .about("List your configured hosts"),
        )
        .subcommand(
            Command::new("add")
                .alias("a")
                .about("Add a new host configuration")
                .arg(arg!([ip] "The server ip address").required(true))
                .arg(arg!([hostname] "The host name").required(true)),
        )
        .subcommand(
            Command::new("remove")
                .alias("rm")
                .about("Remove a host configuration")
                .arg(
                    arg!([id] "The id of the host configuration (According to the list command)")
                        .required(true),
                ),
        )
        .get_matches();

    if let Some(matches) = matches.subcommand_matches("open") {
        let result = open::that(config::HOSTS_FILE_PATH);
    }

    if let Some(matches) = matches.subcommand_matches("list") {
        disp::list_hosts();
    }

    if let Some(matches) = matches.subcommand_matches("add") {
        let ip = matches.get_one::<String>("ip").unwrap().clone();
        let hostname = matches.get_one::<String>("hostname").unwrap().clone();
        let host = host::Host::new(0, ip, hostname, 0);
        host_file::add_host(host);
    }

    if let Some(matches) = matches.subcommand_matches("remove") {
        let id = matches
            .get_one::<String>("id")
            .unwrap()
            .parse::<usize>()
            .unwrap();
        host_file::remove_host(id);
    }
}
