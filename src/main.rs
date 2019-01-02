/*!
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
extern crate dirs;
extern crate reqwest;

mod aur;

use clap::{App, SubCommand};
use colored::*;

fn main() {
    let matches = App::new("midna")
        .version("0.1.0")
        .about("Alternative AUR package helper/manager")
        .subcommand(SubCommand::with_name("update").about("Update local AUR package list"))
        .get_matches();

    let aur = aur::Aur {};

    aur.check_for_data_dir();

    if let Some(_matches) = matches.subcommand_matches("update") {
        println!(" {}\t{}", "Updating".bold().yellow(), "AUR package list...");
        match aur.update_package_list() {
            Ok(_list) => println!(" {}\t{}", "Updated".bold().green(), "AUR package list."),
            Err(e) => println!("{}", e),
        };
    } else {
        println!(
            "{}",
            "No command given. Try 'midna update' or 'midna install'."
                .bold()
                .red()
        );
    }
}
