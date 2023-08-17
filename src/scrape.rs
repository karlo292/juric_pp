use std::io::{Stdout};
use std::collections::HashMap;

use crossterm::{style, style::Color, terminal, QueueableCommand, execute};

pub fn get_url(os: &String, package: &String) -> String {
    let mut url: String = String::new();

    match os.as_str() {
        "arch" => {
            url = "https://archlinux.org/packages/".to_string();
            url += "?sort=&q=";
            url += &package;
        },
        _ => println!("Unknown operating system!"),
    }

    return url;
}

fn search_aur() {
    
}

fn scrape_matches(stdout: &mut Stdout, matches: &Vec<String>) {
    if matches.len() == 0 {

        stdout.queue(style::SetForegroundColor(Color::Yellow)).expect("Failed to set foreground color!");
        println!("Package not found!");
        println!("Searching in AUR...");
        stdout.queue(style::SetForegroundColor(Color::Reset)).expect("Failed to set/reset foreground color!");

        search_aur()
    }

    let words: Vec<&str> = matches[0].split_whitespace().collect();

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
}

pub fn scrape_url(stdout: &mut Stdout, url: &String) {
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

    scrape_matches(stdout, &matches);
}
