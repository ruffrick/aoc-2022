use std::collections::HashMap;

pub fn solve() {
    let input = crate::input("day07.txt");

    let mut dirs: HashMap<String, u32> = HashMap::new();
    let mut path: Vec<&str> = Vec::new();
    for line in input.lines() {
        let args: Vec<&str> = line.split_whitespace().collect();
        if args[0] == "$" {
            if args[1] == "cd" {
                match args[2] {
                    ".." => {
                        path.pop();
                    }
                    "/" => path = Vec::new(),
                    dir => path.push(dir),
                }
            }
        } else if let Ok(size) = args[0].parse::<u32>() {
            let mut current = "/".to_owned();
            add(&mut dirs, &current, size);
            for segment in path.iter() {
                current.push_str(segment);
                add(&mut dirs, &current, size);
            }
        }
    }

    let part_one: u32 = dirs.values().filter(|size| **size <= 100_000).sum();

    let free_space = 70_000_000 - dirs.get(&"/".to_owned()).unwrap();
    let part_two = dirs
        .values()
        .filter(|size| free_space + *size >= 30_000_000)
        .min()
        .unwrap();

    println!("Day 7\n\tPart One - {part_one}\n\tPart Two - {part_two}\n");
}

fn add(dirs: &mut HashMap<String, u32>, path: &String, size: u32) {
    if let Some(total) = dirs.get_mut(path) {
        *total += size;
    } else {
        dirs.insert(path.clone(), size);
    }
}
