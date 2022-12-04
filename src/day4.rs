pub fn solve() {
    let mut part_one: u32 = 0;
    let mut part_two: u32 = 0;
    for line in crate::input("day4.txt").lines() {
        let ranges: Vec<Vec<u32>> = line
            .split(",")
            .map(|range| {
                let start_end: Vec<u32> =
                    range.split("-").map(|num| num.parse().unwrap()).collect();
                (start_end[0]..=start_end[1]).collect()
            })
            .collect();

        let first = &ranges[0];
        let second = &ranges[1];

        if first.iter().all(|n| second.contains(&n)) || second.iter().all(|n| first.contains(&n)) {
            part_one += 1;
        }

        if first.iter().any(|n| second.contains(&n)) {
            part_two += 1;
        }
    }

    println!(
        "Day 4\n\tPart One - {}\n\tPart Two - {}\n",
        part_one, part_two
    );
}
