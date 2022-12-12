pub fn solve() {
    let trees: Vec<Vec<u32>> = crate::input("day08.txt")
        .lines()
        .map(|line| line.chars().map(|c| c.to_digit(10).unwrap()).collect())
        .collect();

    let len = trees.len();
    let mut part_one = (len - 1) * 4;
    let mut scores = Vec::new();
    for x in 1..len - 1 {
        for y in 1..len - 1 {
            if check_visibility(&trees, x, y) {
                part_one += 1;
            }
            scores.push(calculate_score(&trees, x, y))
        }
    }
    let part_two = scores.iter().max().unwrap();

    println!("Day 8\n\tPart One - {part_one}\n\tPart Two - {part_two}\n");
}

fn check_visibility(trees: &Vec<Vec<u32>>, x: usize, y: usize) -> bool {
    let len = trees.len();
    let tree = trees[x][y];

    let mut left = true;
    for i in 0..x {
        left = left && trees[i][y] < tree;
    }

    let mut right = true;
    for i in x + 1..len {
        right = right && trees[i][y] < tree;
    }

    let mut top = true;
    for i in 0..y {
        top = top && trees[x][i] < tree;
    }

    let mut bottom = true;
    for i in y + 1..len {
        bottom = bottom && trees[x][i] < tree;
    }

    left || right || top || bottom
}

fn calculate_score(trees: &Vec<Vec<u32>>, x: usize, y: usize) -> u32 {
    let len = trees.len();
    let tree = trees[x][y];

    let mut left = 0;
    for i in (0..x).rev() {
        left += 1;
        if trees[i][y] >= tree {
            break;
        }
    }

    let mut right = 0;
    for i in x + 1..len {
        right += 1;
        if trees[i][y] >= tree {
            break;
        }
    }

    let mut top = 0;
    for i in (0..y).rev() {
        top += 1;
        if trees[x][i] >= tree {
            break;
        }
    }

    let mut bottom = 0;
    for i in y + 1..len {
        bottom += 1;
        if trees[x][i] >= tree {
            break;
        }
    }

    left * right * top * bottom
}
