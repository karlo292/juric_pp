use std::io::{stdout, Stdout};
use std::{env, process::exit};

use crossterm::{style, style::Color, terminal, QueueableCommand, execute};
use crossterm::cursor::MoveTo;

mod scrape;
mod utilities;

fn main() {
    clearscreen::clear().expect("Failed to clean screen!");
    let mut stdout: Stdout = stdout();

    let args: Vec<String> = env::args().collect();

    if args.len() < 4 {
        println!("Too few arguments passed!");
        println!("Usage: ./cargo run --release <distro> <architecture> <package>");

        exit(0);
    }

    let warning_1: String = String::from("Juric++ is not offical package manager of your distribution. ringwormGO does not gurrant safety of your files!");
    let warning_2: String = String::from("User discretion advised!");

    let x: u16 = terminal::size().unwrap().0;
    stdout.queue(style::SetForegroundColor(Color::Red)).expect("Failed to set foreground color!");

    execute!(stdout, MoveTo((x / 2) - (u16::try_from(warning_1.len() / 2).unwrap()), 0)).expect("Failed to move cursor!");
    println!("{}", warning_1);

    execute!(stdout, MoveTo((x / 2) - (u16::try_from(warning_2.len() / 2).unwrap()), 1)).expect("Failed to move cursor!");
    println!("{}", warning_2);

    stdout.queue(style::SetForegroundColor(Color::Green)).expect("Failed to set foreground color!");
    println!("Gathering informations...");
    stdout.queue(style::SetForegroundColor(Color::Reset)).expect("Failed to set/reset foreground color!");

    println!("    OS: {} | Requested OS: {}", whoami::distro().to_string(), &args[1]);
    println!("    Architecture requested: {}", &args[2]);
    println!("    Package requested: {}", &args[3]);

    println!("");

    scrape::scrape(&mut stdout, &args[1], &args[3]);
}
