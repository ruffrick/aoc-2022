use std::collections::HashMap;

use clap::Parser;
use rust_embed::RustEmbed;

mod day1;

#[derive(Parser)]
struct Args {
    #[arg(short, long)]
    day: Option<u8>,
}

#[derive(RustEmbed)]
#[folder = "input/"]
struct Input;

fn main() {
    let args = Args::parse();

    let mut days = HashMap::new();
    days.insert(1, day1::solve);

    println!("Advent of Code 2022\n");
    match args.day {
        Some(day) => match days.get(&day) {
            Some(solve) => solve(),
            None => println!("Unknown day"),
        },
        None => {
            for solve in days.values() {
                solve();
            }
        }
    }
}

fn input(file_path: &str) -> String {
    let input = Input::get(file_path).unwrap();
    String::from_utf8(Vec::from(input.data)).unwrap()
}
