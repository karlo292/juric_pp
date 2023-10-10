use std::collections::HashMap;
use std::fs::File;
use std::io::{Stdout, Write};
use std::process::{Command, Stdio};
use std::{thread, time};

use crossterm::{style, style::Color, QueueableCommand};
use regex::Regex;
use reqwest::blocking::Client;

use crate::utilities;

struct AUR {
    base_url: String,
    pkg_url: String,
}

struct ArchLinux {
    aur: AUR,
    base_url: String,
    pkg_url: String,
}

impl ArchLinux {
    fn download(&self, stdout: &mut Stdout, url: &String) {
        utilities::create_temp_folder();
        utilities::cd_to_temp_folder();

        let mut git = Command::new("git");
        git.arg("clone").arg(url);

        git.output().expect("Failed to execute process 'git clone`!");

        print_download_progress(stdout);

        let last_slash_index = url.rfind('/').unwrap_or(0);
        let result = &url[last_slash_index + 1..];
        utilities::cd_to_folder(&result.to_string());

        self.aur.install(stdout);
    }

    fn scrape_package(&self, stdout: &mut Stdout, url: &String) {
        let response = reqwest::blocking::get(url).unwrap().text().unwrap();

        let document = scraper::Html::parse_document(&response);
        let selector = scraper::Selector::parse("div#actionlist>ul>li>a").unwrap();

        for _match in document.select(&selector) {
            let opt = Some(_match.value().attr("href"));
            if let Some(opt_2) = opt {
                self.download(stdout, &opt_2.to_owned().to_string());
            }
        }
    }

    fn scrape_matches(&self, stdout: &mut Stdout, package: &String) {
        let response = reqwest::blocking::get(self.base_url.clone()).unwrap().text().unwrap();

        let document = scraper::Html::parse_document(&response);
        let selector = scraper::Selector::parse("div#exact-matches>table>tbody>tr").unwrap();

        let mut matches: Vec<String> = Vec::new();
        for _match in document.select(&selector) {
            let mut element = _match.text().collect::<Vec<_>>().join(" ");
            element = element.trim().replace("\n", " ");
            matches.push(element);
        }

        if matches.is_empty() {
            print_package_not_found(stdout);
            self.aur.scrape(stdout, package);
            std::process::exit(0);
        }

        let words = matches[0].split_whitespace().collect::<Vec<&str>>();
        let mut word_mapping = HashMap::new();
        for (index, word) in words.iter().enumerate() {
            word_mapping.insert(index.to_string(), word.to_string());
        }

        print_package_info(&word_mapping);

        let mut pkg_url = self.pkg_url.clone();
        pkg_url += &word_mapping["1"];
        pkg_url += "/";
        pkg_url += &word_mapping["0"];
        pkg_url += "/";
        pkg_url += &word_mapping["2"];
        pkg_url += "/";

        self.scrape_package(stdout, &pkg_url);
    }
}

impl AUR {
    fn install(&self, stdout: &mut Stdout) {
        let mut cmd = Command::new("makepkg");
        cmd.args(&["-si"]).stdin(Stdio::piped());
        let mut child = cmd.spawn().expect("Failed to start makepkg");

        let stdin = child.stdin.as_mut().expect("Failed to open stdin");
        let confirmation = "y";
        stdin.write_all(confirmation.as_bytes()).expect("Failed to write password");
        stdin.flush().expect("Failed to flush stdin");

        let status = child.wait().expect("Failed to wait for makepkg");

        print_installation_result(stdout, status.success());
    }

    fn download(&self, stdout: &mut Stdout, url: &String, package: &String) {
        utilities::create_temp_folder();
        utilities::cd_to_temp_folder();
        utilities::create_folder(package);
        utilities::cd_to_folder(package);

        let client = Client::new();
        let mut response = match client.get(url).send() {
            Ok(res) => res,
            Err(err) => {
                eprintln!("Failed to send the GET request: {}", err);
                return;
            }
        };

        if !response.status().is_success() {
            eprintln!("Request failed with status code: {}", response.status());
        }

        let mut output_file = match File::create("PKGBUILD") {
            Ok(file) => file,
            Err(err) => {
                eprintln!("Failed to create the output file: {}", err);
                return;
            }
        };

        if let Err(err) = std::io::copy(&mut response, &mut output_file) {
            eprintln!("Failed to copy response content to the file: {}", err);
        }

        print_download_progress(stdout);
        self.install(stdout);
    }

    fn scrape(&self, stdout: &mut Stdout, package: &String) {
        let response = reqwest::blocking::get(self.base_url.clone()).unwrap().text().unwrap();
        let document = scraper::Html::parse_document(&response);
        let selector = scraper::Selector::parse("div#pkglist-results>form>table>tbody>tr").unwrap();

        let mut matches: Vec<String> = Vec::new();
        for _match in document.select(&selector) {
            let mut element = _match.text().collect::<Vec<_>>().join(" ");
            element = element.trim().replace("\n", " ");
            let starts_with = format!("{} ", package);
            if element.starts_with(&starts_with) {
                matches.push(element);
            }
        }

        if matches.is_empty() {
            print_package_not_found(stdout);
            std::process::exit(0);
        }

        let words = matches[0].split_whitespace().collect::<Vec<&str>>();
        let mut word_mapping = HashMap::new();
        for (index, word) in words.iter().enumerate() {
            word_mapping.insert(index.to_string(), word.to_string());
        }

        print_package_info(&word_mapping);

        let mut pkg_url = String::new();
        pkg_url += &self.pkg_url;
        pkg_url +=
