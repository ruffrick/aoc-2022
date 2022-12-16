use std::collections::{HashMap, HashSet, VecDeque};

type Height = i8;

#[derive(Debug, Eq, PartialEq, Hash, Clone, Copy)]
struct Square {
    x: usize,
    y: usize,
}

pub fn solve() {
    let mut start = None;
    let mut end = None;
    let heightmap: Vec<Vec<Height>> = crate::input("day12.txt")
        .lines()
        .enumerate()
        .map(|(y, line)| {
            line.chars()
                .enumerate()
                .map(|(x, square)| {
                    let elevation = match square {
                        'S' => {
                            start = Some(Square { x, y });
                            'a'
                        }
                        'E' => {
                            end = Some(Square { x, y });
                            'z'
                        }
                        c => c,
                    };
                    elevation as Height - 'a' as Height
                })
                .collect()
        })
        .collect();
    let start = start.unwrap();
    let end = end.unwrap();

    let steps = find_steps(&heightmap, end);
    let part_one = construct_path(&steps, start, end)
        .map(|path| path.len() - 1)
        .unwrap();
    let part_two = heightmap
        .iter()
        .enumerate()
        .flat_map(|(y, row)| {
            row.iter()
                .enumerate()
                .filter_map(move |(x, h)| if *h == 0 { Some(Square { x, y }) } else { None })
        })
        .filter_map(|start| construct_path(&steps, start, end).map(|path| path.len() - 1))
        .min()
        .unwrap();

    println!("Day 12\n\tPart One - {part_one}\n\tPart Two - {part_two}\n",);
}

fn neighbors(heightmap: &Vec<Vec<Height>>, square: Square) -> Vec<Square> {
    let mut neighbors = Vec::new();
    let h = heightmap[square.y][square.x];
    if square.x > 0 && h - heightmap[square.y][square.x - 1] <= 1 {
        neighbors.push(Square {
            x: square.x - 1,
            y: square.y,
        })
    }
    if square.x < heightmap[square.y].len() - 1 && h - heightmap[square.y][square.x + 1] <= 1 {
        neighbors.push(Square {
            x: square.x + 1,
            y: square.y,
        });
    }
    if square.y > 0 && h - heightmap[square.y - 1][square.x] <= 1 {
        neighbors.push(Square {
            x: square.x,
            y: square.y - 1,
        });
    }
    if square.y < heightmap.len() - 1 && h - heightmap[square.y + 1][square.x] <= 1 {
        neighbors.push(Square {
            x: square.x,
            y: square.y + 1,
        });
    }
    neighbors
}

fn find_steps(heightmap: &Vec<Vec<Height>>, end: Square) -> HashMap<Square, Square> {
    let mut visited = HashSet::new();
    let mut queue = VecDeque::new();
    let mut steps = HashMap::new();
    visited.insert(end);
    queue.push_back(end);
    while let Some(square) = queue.pop_front() {
        for neighbor in neighbors(heightmap, square) {
            if visited.insert(neighbor) {
                steps.insert(neighbor, square);
                queue.push_back(neighbor);
            }
        }
    }
    steps
}

fn construct_path(
    steps: &HashMap<Square, Square>,
    start: Square,
    end: Square,
) -> Option<Vec<Square>> {
    let mut prev = start;
    let mut path = vec![prev];
    while prev != end {
        if let Some(square) = steps.get(&prev) {
            prev = *square;
            path.push(prev);
        } else {
            return None;
        }
    }
    Some(path)
}
