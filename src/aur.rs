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
use super::log;
use git2::Repository;
use glob::glob;
use reqwest::Error;
use serde_json::Value;
use std::fs;
use std::fs::File;
use std::io::Write;
use std::path::PathBuf;
use std::process::Command;

const DATA_DIRECTORY: &'static str = "midna";
const LOCAL_PACKAGES_LIST: &'static str = "midna/packages_list";
const PACKAGE_LIST: &'static str = "https://aur.archlinux.org/packages.gz";
const AUR_RPC_SEARCH: &'static str = "https://aur.archlinux.org/rpc/?v=5&type=search&arg=";
const AUR_CLONE: &'static str = "https://aur.archlinux.org/";

pub struct Aur {}

impl Aur {
    pub fn check_for_data_dir(&self) {
        if let Some(data_local_dir) = dirs::data_local_dir() {
            let config_path = data_local_dir.as_path().join(self::DATA_DIRECTORY);

            if !config_path.exists() {
                fs::create_dir(config_path.as_path()).unwrap();
            }
        }
    }

    pub fn update_package_list(&self) -> Result<String, Error> {
        let body = reqwest::get(self::PACKAGE_LIST)?.text()?;

        if let Some(data_local_dir) = dirs::data_local_dir() {
            let config_path = data_local_dir.as_path().join(self::LOCAL_PACKAGES_LIST);

            let mut package_list = File::create(config_path.as_path()).unwrap();
            package_list.write_all(body.as_bytes()).unwrap();
        }

        Ok(body)
    }

    pub fn search_package(&self, package_name: &str) -> Value {
        let body = reqwest::get(&format!("{}{}", self::AUR_RPC_SEARCH, package_name))
            .unwrap()
            .text()
            .unwrap();
        let result: Value = serde_json::from_str(&body).unwrap();

        return result;
    }

    pub fn clone_package(&self, package_name: &str) {
        if let Some(data_local_dir) = dirs::data_local_dir() {
            let repo_dir = data_local_dir
                .as_path()
                .join(self::DATA_DIRECTORY)
                .join(package_name);

            if repo_dir.exists() {
                log::warning("Repository already exists, will not clone it.");
                return;
            }
        }

        let _repo = match Repository::clone(
            &format!("{}{}.git", self::AUR_CLONE, package_name),
            &format!(
                "{}/{}",
                self.get_data_dir().unwrap().to_str().unwrap(),
                package_name
            ),
        ) {
            Ok(_repo) => log::info("Successfully cloned", package_name),
            Err(e) => log::error("Failed to clone", package_name, e.message()),
        };
    }

    pub fn install_package(&self, package_name: &str, verbose: bool) -> Result<(), String> {
        log::info("Cloning repository from AUR", "");
        self.clone_package(package_name);

        log::info("Running makepkg --syncdeps in repository directory", "");

        let package_file: String = match self.makepkg(package_name, verbose) {
            Ok(package_file) => package_file,
            Err(e) => return Err(e),
        };

        log::info("Installing package with pacman -U", "");
        log::hint("You will be prompted for your password in order to install the package!");
        self.pacman_install(package_file, verbose);

        Ok(())
    }

    pub fn makepkg(&self, package_name: &str, verbose: bool) -> Result<String, String> {
        let mut makepkg_cmd = Command::new("makepkg");
        makepkg_cmd
            .current_dir(format!(
                "{}/{}",
                self.get_data_dir().unwrap().to_str().unwrap(),
                package_name
            ))
            .arg("--syncdeps");

        if verbose {
            let status = makepkg_cmd.status().expect("Failed to execute 'makepkg'");

            if !status.success() {
                return Err(String::from("nwedwedwedeee"));
            }
        } else {
            let output = makepkg_cmd.output().expect("Failed to execute 'makepkg'");

            if !output.status.success() {
                return Err(String::from("disch nit so jut"));
            }
        }

        let mut package: String = String::from("");

        for entry in glob(&format!(
            "{}/{}/{}-*.pkg.tar.xz",
            self.get_data_dir().unwrap().to_str().unwrap(),
            package_name,
            package_name
        ))
        .unwrap()
        .filter_map(Result::ok)
        {
            package = entry.display().to_string();
        }

        Ok(package.clone())
    }

    pub fn pacman_install(&self, package_file: String, verbose: bool) {
        let mut pacman_cmd = Command::new("sudo");
        pacman_cmd.args(vec!["pacman", "-U", &package_file, "--noconfirm"]);

        if verbose {
            let status = pacman_cmd.status().expect("Failed to execute 'pacman'");

            match status.code() {
                Some(_code) => {}
                None => {}
            }
        } else {
            let output = pacman_cmd.output().expect("Failed to execute 'pacman'");

            match output.status.code() {
                Some(_code) => {}
                None => {}
            }
        }
    }

    // fn search_package_local(package_name: &str) {
    //     let mut file = File::open(
    //         dirs::data_local_dir()
    //             .unwrap()
    //             .as_path()
    //             .join(self::LOCAL_PACKAGES_LIST),
    //     )
    //     .unwrap();

    //     let mut file_content = String::new();
    //     file.read_to_string(&mut file_content).unwrap();

    //     if let Some(_package) = file_content.lines().find(|&p| p == package_name) {
    //         println!("jefunden {} ", package_name);
    //     } else {
    //         println!("hamwa nit jefunde {} ", package_name);
    //     }
    // }

    fn get_data_dir(&self) -> Result<PathBuf, &'static str> {
        match dirs::data_local_dir() {
            Some(data_dir) => Ok(data_dir.as_path().join(self::DATA_DIRECTORY)),
            None => Err("Fatal error."),
        }
    }
}
