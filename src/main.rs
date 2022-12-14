#![feature(iter_array_chunks)]
#![feature(get_many_mut)]

use std::collections::BTreeMap;

use clap::Parser;
use rust_embed::RustEmbed;

mod day01;
mod day02;
mod day03;
mod day04;
mod day05;
mod day06;
mod day07;
mod day08;
mod day09;
mod day10;
mod day11;

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
    days.insert(1, day01::solve);
    days.insert(2, day02::solve);
    days.insert(3, day03::solve);
    days.insert(4, day04::solve);
    days.insert(5, day05::solve);
    days.insert(6, day06::solve);
    days.insert(7, day07::solve);
    days.insert(8, day08::solve);
    days.insert(9, day09::solve);
    days.insert(10, day10::solve);
    days.insert(11, day11::solve);

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
