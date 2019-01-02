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
use reqwest::Error;
use std::fs;
use std::fs::File;
use std::io::Write;

const PACKAGE_LIST: &'static str = "https://aur.archlinux.org/packages.gz";

pub struct Aur {}

impl Aur {
    pub fn check_for_data_dir(&self) {
        if let Some(data_local_dir) = dirs::data_local_dir() {
            let config_path = data_local_dir.as_path().join("midna");

            if !config_path.exists() {
                fs::create_dir(config_path.as_path()).unwrap();
            }
        }
    }

    pub fn update_package_list(&self) -> Result<String, Error> {
        let body = reqwest::get(self::PACKAGE_LIST)?.text()?;

        if let Some(data_local_dir) = dirs::data_local_dir() {
            let config_path = data_local_dir.as_path().join("midna/package_list");

            let mut package_list = File::create(config_path.as_path()).unwrap();

            package_list.write_all(body.as_bytes()).unwrap();
        }

        Ok(body)
    }
}
