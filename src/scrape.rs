use std::collections::HashMap;
use std::io::{Stdout, Write};
use std::process::Command;
use std::{thread, time};

use crossterm::{style, style::Color, QueueableCommand};
use regex::Regex;

use crate::utilities;

fn search_aur() {
    
}

fn download_arch_package(stdout: &mut Stdout, url: &String) {
    utilities::create_temp_folder();
    utilities::cd_to_temp_folder();

    let mut git = Command::new("git");
    git.arg("clone")
        .arg(url);

    let output = git.output().expect("Failed to execute process 'git clone`!");

    println!("");

    stdout.queue(style::SetForegroundColor(Color::Cyan)).expect("Failed to set foreground color!");
    println!("Files for installation have been downloaded!");

    stdout.queue(style::SetForegroundColor(Color::Rgb { r: (232), g: (149), b: (187) })).expect("Failed to set foreground color!");
    std::io::stdout().flush().unwrap();
    print!("Do you want to install them? \\ Wait - ");

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

    let install = Command::new("makepkg")
            .arg("-si")
            .output()
            .expect("Failed to execute process!");

    if !install.status.success() {
        eprintln!("makepkg failed with exit code {:?}", output.status);
        std::process::exit(1);
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

        if element.starts_with("Source") { /*  Source Files, a.k.a repo with files to download)*/
            let opt = Some(_match.value().attr("href"));
            let opt_2 = opt.as_ref().unwrap();

            download_arch_package(stdout, &opt_2.as_ref().unwrap().to_owned().to_string());
        }
    }
}

fn scrape_arch_matches(stdout: &mut Stdout, url: &String) {
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

        search_aur()
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

pub fn scrape_url(stdout: &mut Stdout, os: &String, package: &String) {
    let mut url: String = String::new();

    match os.as_str() {
        "arch" => {
            url = "https://archlinux.org/packages/".to_string();
            url += "?sort=&q=";
            url += &package;

            scrape_arch_matches(stdout, &url);
        },
        _ => println!("Unknown operating system!"),
    }
}
