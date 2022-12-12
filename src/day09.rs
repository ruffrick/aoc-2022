use std::collections::HashSet;

#[derive(Eq, PartialEq, Hash, Clone)]
struct Point {
    x: i32,
    y: i32,
}

enum Dir {
    Left,
    Right,
    Up,
    Down,
}

impl TryFrom<&str> for Dir {
    type Error = ();

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            "L" => Ok(Dir::Left),
            "R" => Ok(Dir::Right),
            "U" => Ok(Dir::Up),
            "D" => Ok(Dir::Down),
            _ => Err(()),
        }
    }
}

pub fn solve() {
    let input = crate::input("day09.txt");

    let mut visited_one: HashSet<Point> = HashSet::new();
    let mut visited_two: HashSet<Point> = HashSet::new();

    let mut head = Point { x: 0, y: 0 };
    let mut tail = Point { x: 0, y: 0 };

    let mut knots = Vec::new();
    for _ in 0..10 {
        knots.push(Point { x: 0, y: 0 })
    }

    for line in input.lines() {
        let parts: Vec<&str> = line.split_whitespace().collect();
        let dir = Dir::try_from(parts[0]).unwrap();
        let step: i32 = parts[1].parse().unwrap();

        for _ in 0..step {
            match dir {
                Dir::Left => head.x -= 1,
                Dir::Right => head.x += 1,
                Dir::Up => head.y -= 1,
                Dir::Down => head.y += 1,
            }
            move_tail(&head, &mut tail);
            visited_one.insert(tail.clone());
        }

        for _ in 0..step {
            let head = knots.first_mut().unwrap();
            match dir {
                Dir::Left => head.x -= 1,
                Dir::Right => head.x += 1,
                Dir::Up => head.y -= 1,
                Dir::Down => head.y += 1,
            }
            for i in 0..knots.len() - 1 {
                if let Ok([head, tail]) = knots.get_many_mut([i, i + 1]) {
                    move_tail(head, tail);
                }
            }
            visited_two.insert(knots.last().unwrap().clone());
        }
    }

    let part_one = visited_one.len();
    let part_two = visited_two.len();

    println!("Day 9\n\tPart One - {part_one}\n\tPart Two - {part_two}\n");
}

fn move_tail(head: &Point, tail: &mut Point) {
    let diff_x = head.x - tail.x;
    let diff_y = head.y - tail.y;
    if diff_x.abs() > 1 || diff_y.abs() > 1 {
        tail.x += diff_x.clamp(-1, 1);
        tail.y += diff_y.clamp(-1, 1);
    }
}
