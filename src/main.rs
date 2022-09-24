#![allow(unused)]

mod host_file;
mod host;
mod disp;

use clap::{App, Arg, Command};
use clap::arg;
use host::Host;


fn main() {

    let matches = App::new("hfm")
        .version(env!("CARGO_PKG_VERSION"))
        .about("A simple hosts file manager")
        .subcommand(
            Command::new("list")
                .about("List your configured hosts")
        )
        .subcommand(
            Command::new("add")
                .about("Add a new host configuration")
                .arg(arg!([ip] "The server ip address").required(true))
                .arg(arg!([hostname] "The host name").required(true))
        )
        .subcommand(
            Command::new("remove")
                .about("Remove a configuration")
                .arg(arg!([id] "The id of the host configuration").required(true))
        )
        .get_matches();

    if let Some(matches) = matches.subcommand_matches("list") {
        disp::list_hosts();
    }

    if let Some(matches) = matches.subcommand_matches("add") {
        let ip = matches.get_one::<String>("ip").unwrap().clone();
        let hostname = matches.get_one::<String>("hostname").unwrap().clone();
        let host = host::Host::new(0, ip, hostname);
        host_file::add_host(host);
    }

    if let Some(matches) = matches.subcommand_matches("remove") {
        let id = matches.get_one::<String>("id").unwrap().clone();
        println!("ID: {}", id);
    }

}
