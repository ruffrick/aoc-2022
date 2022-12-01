use rust_embed::RustEmbed;

mod day1;

#[derive(RustEmbed)]
#[folder = "input/"]
struct Input;

fn main() {
    println!("Advent of Code 2022\n");

    let day1 = day1::solve();
    println!("Day 1\n\tPart One - {}\n\tPart Two - {}\n", day1.0, day1.1);
}

fn input(file_path: &str) -> String {
    let input = Input::get(file_path).unwrap();
    String::from_utf8(Vec::from(input.data)).unwrap()
}
