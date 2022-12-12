pub fn solve() {
    let input = crate::input("day05.txt");

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

    let mut part_one = stacks.clone();
    let mut part_two = stacks;
    for line in parts[1].lines() {
        let vec: Vec<usize> = line
            .split_whitespace()
            .filter_map(|part| part.parse().ok())
            .collect();

        let from = vec[1] - 1;
        let from_one = part_one.get_mut(from).unwrap();
        let from_two = part_two.get_mut(from).unwrap();

        let mut to_one = Vec::new();
        let mut to_two = Vec::new();
        for _ in 0..vec[0] {
            to_one.push(from_one.pop().unwrap());
            to_two.push(from_two.pop().unwrap());
        }

        let to = vec[2] - 1;
        part_one.get_mut(to).unwrap().extend(to_one);
        part_two.get_mut(to).unwrap().extend(to_two.iter().rev());
    }

    let part_one: String = part_one.iter().map(|stack| stack.last().unwrap()).collect();
    let part_two: String = part_two.iter().map(|stack| stack.last().unwrap()).collect();

    println!("Day 5\n\tPart One - {part_one}\n\tPart Two - {part_two}\n");
}
