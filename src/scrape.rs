#![allow(unused_assignments)]

use std::collections::HashMap;
use std::io::{Stdout, Write};
use std::process::{Command, Stdio};
use std::{thread, time};

use crossterm::{style, style::Color, QueueableCommand};
use regex::Regex;

use std::fs::File;
use std::io::copy;
use reqwest::blocking::Client;

use crate::utilities;

fn download_aur(stdout: &mut Stdout, url: &String, package: &String) {
    /* Create directories */
    utilities::create_temp_folder();
    utilities::cd_to_temp_folder();

    utilities::create_folder(package);
    utilities::cd_to_folder(package);

    /* Download PKGBUILD file */

    // Create a reqwest client.
    let client = Client::new();

    // Send a GET request to the URL.
    let mut response = match client.get(url).send() {
        Ok(res) => res,
        Err(err) => {
            eprintln!("Failed to send the GET request: {}", err);
            return;
        }
    };

    // Check if the request was successful (HTTP status 200 OK).
    if !response.status().is_success() {
        eprintln!("Request failed with status code: {}", response.status());
    }

    // Open a file to save the downloaded content.
    let mut output_file = match File::create("PKGBUILD") {
        Ok(file) => file,
        Err(err) => {
            eprintln!("Failed to create the output file: {}", err);
            return;
        }
    };

    // Copy the response content to the output file.
    if let Err(err) = copy(&mut response, &mut output_file) {
        eprintln!("Failed to copy response content to the file: {}", err);
    }

    /* Install package */

    stdout.queue(style::SetForegroundColor(Color::Cyan)).expect("Failed to set foreground color!");
    println!("Files for installation have been downloaded!");

    stdout.queue(style::SetForegroundColor(Color::Rgb { r: (232), g: (149), b: (187) })).expect("Failed to set foreground color!");
    std::io::stdout().flush().unwrap();
    print!("Do you want to install them? \\ Wait (5 sec) - ");

    stdout.queue(style::SetForegroundColor(Color::Green)).expect("Failed to set foreground color!");
    std::io::stdout().flush().unwrap();
    print!("YES");

    stdout.queue(style::SetForegroundColor(Color::Rgb { r: (232), g: (149), b: (187) })).expect("Failed to set foreground color!");
    std::io::stdout().flush().unwrap();
    print!(" | Ctrl-C - ");

    stdout.queue(style::SetForegroundColor(Color::Red)).expect("Failed to set foreground color!");
    println!("NO");

    stdout.queue(style::SetForegroundColor(Color::Reset)).expect("Failed to reset foreground color!");
    println!("");

    let mut i: u16 = 0;
    while i < 5 {
        thread::sleep(time::Duration::from_secs(1));
        
        std::io::stdout().flush().unwrap();
        print!("█");

        i = i + 1
    }

    println!("");

    // TODO - make better installation "process"
}

fn search_aur(stdout: &mut Stdout, package: &String) {
    /* Generate URL */
    let mut url: String = String::new();

    url += "https://aur.archlinux.org/packages?O=0&SeB=nd&K=";
    url += &package;
    url += "&outdated=&SB=p&SO=d&PP=50&submit=Go";

    /* Download AUR page */
    let response = reqwest::blocking::get(
        url,
    )
    .unwrap()
    .text()
    .unwrap();

    /* Web-scrape downloaded page */
    let document = scraper::Html::parse_document(&response);
    let selector = scraper::Selector::parse("div#pkglist-results>form>table>tbody>tr").unwrap();

    let mut matches: Vec<String> = Vec::new();
    for _match in document.select(&selector) {
        let mut element = _match.text().collect::<Vec<_>>().join(" ");
        element = element.trim().replace("\n", " ");

        let mut starts_with: String = package.to_string();
        starts_with += " ";

        if element.starts_with(&starts_with) {
            matches.push(element);
        }
    }

    /* If there are no matches, then package is not found */
    if matches.len() == 0 {
        stdout.queue(style::SetForegroundColor(Color::Red)).expect("Failed to set foreground color!");
        println!("Package not found!");
        stdout.queue(style::SetForegroundColor(Color::Reset)).expect("Failed to set/reset foreground color!");

        std::process::exit(0);
    }

    /* Split "keys" */
    let re = Regex::new(r"\s{2,}").unwrap();
    let words: Vec<&str> = re.split(matches[0].as_str()).collect();

    let mut word_mapping = HashMap::new();
    for (index, word) in words.iter().enumerate() {
        word_mapping.insert((index).to_string(), word.to_string());
    }

    /* Print package information */
    stdout.queue(style::SetForegroundColor(Color::Green)).expect("Failed to set foreground color!");
    println!("Package found!");
    stdout.queue(style::SetForegroundColor(Color::Reset)).expect("Failed to set/reset foreground color!");

    let key = "0";
    if let Some(value) = word_mapping.get(key) {
        println!("    Name: {}", value);
    } else {
        println!("    Key not found: {} (name)!", key);
    }

    let key = "1";
    if let Some(value) = word_mapping.get(key) {
        println!("    Version: {}", value);
    } else {
        println!("    Key not found: {} (Version)!", key);
    }

    let key = "2";
    if let Some(value) = word_mapping.get(key) {
        println!("    Votes: {}", value);
    } else {
        println!("    Key not found: {} (Votes)!", key);
    }

    let key = "3";
    if let Some(value) = word_mapping.get(key) {
        println!("    Popularity: {}", value);
    } else {
        println!("    Key not found: {} (Popularity)!", key);
    }

    let key = "4";
    if let Some(value) = word_mapping.get(key) {
        println!("    Description: {}", value);
    } else {
        println!("    Key not found: {} (Description)!", key);
    }

    let key = "5";
    if let Some(value) = word_mapping.get(key) {
        println!("    Maintainer: {}", value);
    } else {
        println!("    Key not found: {} (Maintainer)!", key);
    }

    let key = "6";
    if let Some(value) = word_mapping.get(key) {
        println!("    Last Updated: {}", value);
    } else {
        println!("    Key not found: {} (Last Updated)!", key);
    }

    println!("");

    /* Generate URL for package page */
    let mut pkg_url: String = String::new();

    pkg_url = "https://aur.archlinux.org/cgit/aur.git/plain/PKGBUILD?h=".to_string();
    pkg_url += word_mapping.get("0").unwrap();

    download_aur(stdout, &pkg_url, package);
}

fn download_arch_package(stdout: &mut Stdout, url: &String) {
    utilities::create_temp_folder();
    utilities::cd_to_temp_folder();

    let mut git = Command::new("git");
    git.arg("clone")
        .arg(url);

    git.output().expect("Failed to execute process 'git clone`!");

    println!("");

    stdout.queue(style::SetForegroundColor(Color::Cyan)).expect("Failed to set foreground color!");
    println!("Files for installation have been downloaded!");

    stdout.queue(style::SetForegroundColor(Color::Rgb { r: (232), g: (149), b: (187) })).expect("Failed to set foreground color!");
    std::io::stdout().flush().unwrap();
    print!("Do you want to install them? \\ Wait (5 sec) - ");

    stdout.queue(style::SetForegroundColor(Color::Green)).expect("Failed to set foreground color!");
    std::io::stdout().flush().unwrap();
    print!("YES");

    stdout.queue(style::SetForegroundColor(Color::Rgb { r: (232), g: (149), b: (187) })).expect("Failed to set foreground color!");
    std::io::stdout().flush().unwrap();
    print!(" | Ctrl-C - ");

    stdout.queue(style::SetForegroundColor(Color::Red)).expect("Failed to set foreground color!");
    println!("NO");

    stdout.queue(style::SetForegroundColor(Color::Reset)).expect("Failed to reset foreground color!");
    println!("");

    let mut i: u16 = 0;
    while i < 5 {
        thread::sleep(time::Duration::from_secs(1));
        
        std::io::stdout().flush().unwrap();
        print!("█");

        i = i + 1
    }

    println!("");

    if let Some(last_slash_index) = url.rfind('/') {
        let result = &url[last_slash_index + 1..];
        utilities::cd_to_folder(&result.to_string());
    }

    // Thanks to ChatGPT for following code

    // Create new command
    let mut cmd = Command::new("makepkg");
    
    // Specify the arguments
    cmd.args(&["-si"]);

    // Set up stdin to capture input
    cmd.stdin(Stdio::piped());

    // Spawn the command process
    let mut child = cmd.spawn().expect("Failed to start makepkg");

    // Prepare to send confirmation
    let stdin = child.stdin.as_mut().expect("Failed to open stdin");
    let confirmation = "y";

    // Write the password to the process's stdin
    stdin.write_all(confirmation.as_bytes()).expect("Failed to write password");
    stdin.flush().expect("Failed to flush stdin");

    // Wait for the command to finish
    let status = child.wait().expect("Failed to wait for makepkg");

    if status.success() {
        stdout.queue(style::SetForegroundColor(Color::Green)).expect("Failed to set foreground color!");
        println!("Package has been successfully installed!");
    } else {
        stdout.queue(style::SetForegroundColor(Color::Red)).expect("Failed to set foreground color!");
        println!("Package installation failed!");
    }
}

fn scrape_arch_package(stdout: &mut Stdout, url: &String) {
    let response = reqwest::blocking::get(
        url,
    )
    .unwrap()
    .text()
    .unwrap();

    let document = scraper::Html::parse_document(&response);
    let selector = scraper::Selector::parse("div#actionlist>ul>li>a").unwrap();

    for _match in document.select(&selector) {
        let mut element = _match.text().collect::<Vec<_>>().join(" ");
        element = element.trim().replace("\n", " ");

        if element.starts_with("Source") { /* Source Files, a.k.a repo with files to download) */
            let opt = Some(_match.value().attr("href"));
            let opt_2 = opt.as_ref().unwrap();

            download_arch_package(stdout, &opt_2.as_ref().unwrap().to_owned().to_string());
        }
    }
}

fn scrape_arch_matches(stdout: &mut Stdout, url: &String, package: &String) {
    let response = reqwest::blocking::get(
        url,
    )
    .unwrap()
    .text()
    .unwrap();

    let document = scraper::Html::parse_document(&response);
    let selector = scraper::Selector::parse("div#exact-matches>table>tbody>tr").unwrap();

    let mut matches: Vec<String> = Vec::new();
    for _match in document.select(&selector) {
        let mut element = _match.text().collect::<Vec<_>>().join(" ");
        element = element.trim().replace("\n", " ");

        matches.push(element);
    }

    if matches.len() == 0 {

        stdout.queue(style::SetForegroundColor(Color::Yellow)).expect("Failed to set foreground color!");
        println!("Package not found!");
        println!("Searching in AUR...");
        stdout.queue(style::SetForegroundColor(Color::Reset)).expect("Failed to set/reset foreground color!");
        
        println!("");
        search_aur(stdout, package);

        std::process::exit(0);
    }

    let re = Regex::new(r"\s{2,}").unwrap();
    let words: Vec<&str> = re.split(matches[0].as_str()).collect();

    let mut word_mapping = HashMap::new();
    for (index, word) in words.iter().enumerate() {
        word_mapping.insert((index).to_string(), word.to_string());
    }

    stdout.queue(style::SetForegroundColor(Color::Green)).expect("Failed to set foreground color!");
    println!("Package found!");
    stdout.queue(style::SetForegroundColor(Color::Reset)).expect("Failed to set/reset foreground color!");

    let key = "0";
    if let Some(value) = word_mapping.get(key) {
        println!("    Arch: {}", value);
    } else {
        println!("    Key not found: {} (Arch)!", key);
    }

    let key = "1";
    if let Some(value) = word_mapping.get(key) {
        println!("    Repo: {}", value);
    } else {
        println!("    Key not found: {} (Repo)!", key);
    }

    let key = "2";
    if let Some(value) = word_mapping.get(key) {
        println!("    Name: {}", value);
    } else {
        println!("    Key not found: {} (Name)!", key);
    }

    let key = "3";
    if let Some(value) = word_mapping.get(key) {
        println!("    Version: {}", value);
    } else {
        println!("    Key not found: {} (Version)!", key);
    }

    let key = "4";
    if let Some(value) = word_mapping.get(key) {
        println!("    Description: {}", value);
    } else {
        println!("    Key not found: {} (Description)!", key);
    }

    let key = "5";
    if let Some(value) = word_mapping.get(key) {
        println!("    Last Updated: {}", value);
    } else {
        println!("    Key not found: {} (Last Updated)!", key);
    }

    let mut pkg_url: String = String::new();
    pkg_url = "https://archlinux.org/packages/".to_string();

    pkg_url += word_mapping.get("1").unwrap();
    pkg_url += "/";

    pkg_url += word_mapping.get("0").unwrap();
    pkg_url += "/";

    pkg_url += word_mapping.get("2").unwrap();
    pkg_url += "/";

    scrape_arch_package(stdout, &pkg_url);
}

pub fn generate_url(stdout: &mut Stdout, os: &String, package: &String) {
    let mut url: String = String::new();

    match os.as_str() {
        "arch" => {
            url = "https://archlinux.org/packages/".to_string();
            url += "?sort=&q=";
            url += &package;

            scrape_arch_matches(stdout, &url, package);
        },
        _ => println!("Unknown operating system!"),
    }
}
