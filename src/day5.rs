pub fn solve() {
    let input = crate::input("day5.txt");

    let parts: Vec<&str> = input.split("\n\n").collect();
    let mut stacks: Vec<Vec<char>> = Vec::new();

    let mut lines = parts[0].lines().rev();
    let indices = lines.next().unwrap().split_whitespace().count();
    for line in lines {
        for i in 0..indices {
            let c = line.chars().nth(i * 4 + 1).unwrap();
            if !c.is_whitespace() {
                if let Some(stack) = stacks.get_mut(i) {
                    stack.push(c);
                } else {
                    stacks.push(vec![c]);
                }
            }
        }
    }

    for line in parts[1].lines() {
        let vec: Vec<usize> = line
            .split_whitespace()
            .filter_map(|part| part.parse().ok())
            .collect();
        let from = stacks.get_mut(vec[1] - 1).unwrap();
        let mut new = Vec::new();
        for _ in 0..vec[0] {
            if let Some(c) = from.pop() {
                new.push(c);
            }
        }
        stacks
            .get_mut(vec[2] - 1)
            .unwrap()
            .extend_from_slice(new.as_slice());
    }

    let part_one: String = stacks.iter().map(|stack| stack.last().unwrap()).collect();

    println!("Day 5\n\tPart One - {}\n\tPart Two - {}\n", part_one, 0);
}
