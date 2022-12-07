pub fn solve() {
    let mut calories: Vec<u32> = crate::input("day1.txt")
        .split("\n\n")
        .map(|lines| lines.lines().map(|line| line.parse::<u32>().unwrap()).sum())
        .collect();
    calories.sort();

    let part_one = *calories.last().unwrap();
    let part_two: u32 = calories.iter().rev().take(3).sum();

    println!("Day 1\n\tPart One - {part_one}\n\tPart Two - {part_two}\n");
}
