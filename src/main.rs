#![feature(iter_array_chunks)]

use std::collections::BTreeMap;

use clap::Parser;
use rust_embed::RustEmbed;

mod day1;
mod day2;
mod day3;
mod day4;
mod day5;
mod day6;
mod day7;

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

    let mut days: BTreeMap<u8, fn()> = BTreeMap::new();
    days.insert(1, day1::solve);
    days.insert(2, day2::solve);
    days.insert(3, day3::solve);
    days.insert(4, day4::solve);
    days.insert(5, day5::solve);
    days.insert(6, day6::solve);
    days.insert(7, day7::solve);

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
