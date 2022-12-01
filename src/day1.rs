pub fn solve() -> (i32, i32) {
    let mut calories: Vec<i32> = crate::input("day1.txt")
        .split("\n\n")
        .map(|lines| {
            lines
                .split("\n")
                .filter(|line| !line.is_empty())
                .map(|line| line.parse::<i32>().unwrap())
                .sum()
        })
        .collect();
    calories.sort();

    let most = *calories.last().unwrap();
    let top_three = calories.iter().rev().take(3).sum();

    (most, top_three)
}
