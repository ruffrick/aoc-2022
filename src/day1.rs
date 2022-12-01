pub fn solve() {
    let mut calories: Vec<u32> = crate::input("day1.txt")
        .split("\n\n")
        .map(|lines| lines.lines().map(|line| line.parse::<u32>().unwrap()).sum())
        .collect();
    calories.sort();

    let most = *calories.last().unwrap();
    let top_three: u32 = calories.iter().rev().take(3).sum();

    println!("Day 1\n\tPart One - {}\n\tPart Two - {}\n", most, top_three);
}
