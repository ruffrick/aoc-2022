use std::collections::HashSet;
use std::str::Chars;

pub fn solve() {
    let input = crate::input("day6.txt");

    let part_one = find_marker(input.chars(), 4);
    let part_two = find_marker(input.chars(), 14);

    println!("Day 6\n\tPart One - {part_one}\n\tPart Two - {part_two}\n");
}

fn find_marker(input: Chars, len: usize) -> usize {
    let mut vec = Vec::new();
    for (i, c) in input.enumerate() {
        vec.push(c);
        if vec.len() > len {
            vec.remove(0);
        }

        let set: HashSet<char> = HashSet::from_iter(vec.clone());
        if vec.len() == len && set.len() == vec.len() {
            return i + 1;
        }
    }
    0
}
