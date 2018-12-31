/**
 * Copyright 2019 Alexander Kluth <deralex@cpan.org>
 *
 * Licensed under the Apache License, Version 2.0 (the "License");
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 *   http://www.apache.org/licenses/LICENSE-2.0
 *
 * Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
 */
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
