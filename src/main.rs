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
extern crate git2;
extern crate reqwest;
extern crate serde_json;

mod aur;

use clap::{App, Arg, SubCommand};
use colored::*;
use serde_json::Value;

fn main() {
    let matches = App::new("midna")
        .version("0.1.0")
        .about("Alternative AUR package helper/manager")
        .subcommand(SubCommand::with_name("update").about("Update local AUR package list"))
        .subcommand(
            SubCommand::with_name("search")
                .about("Search for package in AUR package list")
                .arg(
                    Arg::with_name("package_name")
                        .value_name("PACKAGE_NAME")
                        .takes_value(true)
                        .required(true),
                ),
        )
        .subcommand(
            SubCommand::with_name("install")
                .about("Install package from AUR")
                .arg(
                    Arg::with_name("package_name")
                        .value_name("PACKAGE_NAME")
                        .takes_value(true)
                        .required(true),
                ),
        )
        .get_matches();

    let aur = aur::Aur {};

    aur.check_for_data_dir();

    if let Some(_matches) = matches.subcommand_matches("update") {
        println!(" {}\t{}", "Updating".bold().yellow(), "AUR package list...");
        match aur.update_package_list() {
            Ok(_list) => println!(" {}\t{}", "Updated".bold().green(), "AUR package list."),
            Err(e) => println!("{}", e),
        };
    } else if let Some(cmd) = matches.subcommand_matches("search") {
        let results: Value = aur.search_package(cmd.value_of("package_name").unwrap());

        for i in 0..results["results"].as_array().unwrap().len() {
            println!(
                " {}/{} {}",
                "aur".bold().cyan(),
                results["results"][i]["Name"]
                    .as_str()
                    .unwrap()
                    .bold()
                    .white(),
                results["results"][i]["Version"]
                    .as_str()
                    .unwrap()
                    .bold()
                    .green()
            );

            if let Some(desc) = results["results"][i]["Description"].as_str() {
                println!("    {}", desc);
            }
        }
    } else if let Some(cmd) = matches.subcommand_matches("install") {
        let package_name = cmd.value_of("package_name").unwrap();

        println!(
            " {}\t{}",
            "Installing ".bold().green(),
            package_name.bold().white()
        );

        aur.install_package(package_name);
    } else {
        println!(
            "{}",
            "No command given. Try 'midna update' or 'midna search $PACKAGE_NAME'."
                .bold()
                .red()
        );
    }
}
