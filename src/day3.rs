pub fn solve() {
    let part_one: u32 = crate::input("day3.txt")
        .lines()
        .map(|line| {
            let (first, second) = line.split_at(line.len() / 2);
            let shared = first
                .chars()
                .filter(|c| second.contains(*c))
                .next()
                .unwrap();
            priority(shared)
        })
        .sum();

    let part_two: u32 = crate::input("day3.txt")
        .lines()
        .array_chunks()
        .map(|[first, second, third]| {
            let shared = first
                .chars()
                .filter(|c| second.contains(*c) && third.contains(*c))
                .next()
                .unwrap();
            priority(shared)
        })
        .sum();

    println!(
        "Day 3\n\tPart One - {}\n\tPart Two - {}\n",
        part_one, part_two
    );
}

fn priority(c: char) -> u32 {
    let ascii = c as u8;
    return if ascii > 90 { ascii - 96 } else { ascii - 38 } as u32;
}
