extern crate clap;
extern crate colored;
extern crate reqwest;

use clap::{App, Arg, SubCommand};
use colored::*;
use reqwest::get;

fn main() {
    let matches = App::new("midna")
        .version("0.1.0")
        .about("Alternative AUR package helper/manager")
        .arg(
            Arg::with_name("sync")
                .short("S")
                .long("Sync")
                .value_name("PACKAGE_NAME")
                .help("Install package from AUR or official repositories")
                .takes_value(true)
                .required(true),
        )
        .get_matches();

    println!(
        "{} {}",
        " Searching ".bold().green(),
        matches.value_of("sync").unwrap()
    );
}
