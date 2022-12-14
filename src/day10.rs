pub fn solve() {
    let input = crate::input("day10.txt");

    let mut cycle = 1;
    let mut x = 1;
    let mut part_one = 0;
    let mut part_two = String::new();
    for line in input.lines() {
        let mut split = line.split_whitespace();
        match split.next().unwrap() {
            "addx" => {
                next_cycle(&mut cycle, &x, &mut part_one, &mut part_two);
                next_cycle(&mut cycle, &x, &mut part_one, &mut part_two);
                let v: i32 = split.next().unwrap().parse().unwrap();
                x += v;
            }
            "noop" => {
                next_cycle(&mut cycle, &x, &mut part_one, &mut part_two);
            }
            _ => {}
        }
    }

    println!("Day 10\n\tPart One - {part_one}\n\tPart Two - \n{part_two}",);
}

fn next_cycle(cycle: &mut i32, x: &i32, part_one: &mut i32, part_two: &mut String) {
    if (*cycle + 20) % 40 == 0 {
        *part_one += *cycle * x;
    }
    if (x - 1..=x + 1).contains(&(*cycle % 40 - 1)) {
        part_two.push('#');
    } else {
        part_two.push('.');
    }
    if *cycle % 40 == 0 {
        part_two.push('\n');
    }
    *cycle += 1;
}
