// Import necessary libraries and modules
use std::env;
use std::process::exit;

use crossterm::{execute, style, style::Color, terminal};
use crossterm::cursor::MoveTo;
use std::io::Stdout;

// Import custom modules
mod scrape;
mod utilities;

fn main() {
    // Clear the screen
    clear_screen();

    // Get command-line arguments
    let args: Vec<String> = env::args().collect();

    // Check if there are enough command-line arguments
    if args.len() < 4 {
        println!("Too few arguments passed!");
        println!("Usage: ./cargo run --release <distro> <architecture> <package>");
        exit(0);
    }

    // Display warnings
    display_warnings();

    // Display gathered information
    display_info(&args[1], &args[2], &args[3]);

    // Perform the scraping
    scrape::scrape(&mut stdout(), &args[1], &args[3]);
}

fn clear_screen() {
    // Clear the screen using the clearscreen crate
    clearscreen::clear().expect("Failed to clean screen!");
}

fn display_warnings() {
    // Display warnings in red color
    let warning_1 = "Juric++ is not an official package manager of your distribution. ringwormGO does not guarantee the safety of your files!";
    let warning_2 = "User discretion is advised!";

    let x = terminal::size().unwrap().0;
    let mut stdout: Stdout = stdout();

    stdout.queue(style::SetForegroundColor(Color::Red)).expect("Failed to set foreground color!");

    display_centered_text(&mut stdout, warning_1);
    display_centered_text(&mut stdout, warning_2);

    // Reset text color to green
    stdout.queue(style::SetForegroundColor(Color::Green)).expect("Failed to set foreground color!");
    println!("Gathering information...");
    stdout.queue(style::SetForegroundColor(Color::Reset)).expect("Failed to set/reset foreground color!");
}

fn display_centered_text(stdout: &mut Stdout, text: &str) {
    // Display text centered horizontally
    let x = terminal::size().unwrap().0;
    let center_x = (x / 2) - (text.len() as u16 / 2);

    execute!(stdout, MoveTo(center_x, 0)).expect("Failed to move cursor!");
    println!("{}", text);
}

fn display_info(distro: &str, architecture: &str, package: &str) {
    // Display gathered information
    println!("OS: {} | Requested OS: {}", whoami::distro().to_string(), distro);
    println!("Architecture requested: {}", architecture);
    println!("Package requested: {}", package);
    println!();
}
