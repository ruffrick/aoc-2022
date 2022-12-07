#[derive(Clone, PartialEq)]
enum HandShape {
    Rock,
    Paper,
    Scissors,
}

impl HandShape {
    fn score(&self) -> u32 {
        match self {
            HandShape::Rock => 1,
            HandShape::Paper => 2,
            HandShape::Scissors => 3,
        }
    }

    fn beats(&self, other: HandShape) -> bool {
        match self {
            HandShape::Rock => matches!(other, HandShape::Scissors),
            HandShape::Paper => matches!(other, HandShape::Rock),
            HandShape::Scissors => matches!(other, HandShape::Paper),
        }
    }
}

impl TryFrom<char> for HandShape {
    type Error = ();

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            'A' | 'X' => Ok(HandShape::Rock),
            'B' | 'Y' => Ok(HandShape::Paper),
            'C' | 'Z' => Ok(HandShape::Scissors),
            _ => Err(()),
        }
    }
}

enum Outcome {
    Lose,
    Draw,
    Win,
}

impl TryFrom<char> for Outcome {
    type Error = ();

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            'X' => Ok(Outcome::Lose),
            'Y' => Ok(Outcome::Draw),
            'Z' => Ok(Outcome::Win),
            _ => Err(()),
        }
    }
}

pub fn solve() {
    let part_one: u32 = crate::input("day2.txt")
        .lines()
        .map(|line| {
            let chars: Vec<&str> = line.split_whitespace().collect();

            let abc = chars[0].chars().next().unwrap();
            let xyz = chars[1].chars().next().unwrap();

            let p1 = HandShape::try_from(abc).unwrap();
            let p2 = HandShape::try_from(xyz).unwrap();

            result(p1, p2)
        })
        .sum();

    let part_two: u32 = crate::input("day2.txt")
        .lines()
        .map(|line| {
            let chars: Vec<&str> = line.split_whitespace().collect();

            let abc = chars[0].chars().next().unwrap();
            let xyz = chars[1].chars().next().unwrap();

            let p1 = HandShape::try_from(abc).unwrap();
            let p2 = match Outcome::try_from(xyz).unwrap() {
                Outcome::Lose => match p1 {
                    HandShape::Rock => HandShape::Scissors,
                    HandShape::Paper => HandShape::Rock,
                    HandShape::Scissors => HandShape::Paper,
                },
                Outcome::Draw => p1.clone(),
                Outcome::Win => match p1 {
                    HandShape::Rock => HandShape::Paper,
                    HandShape::Paper => HandShape::Scissors,
                    HandShape::Scissors => HandShape::Rock,
                },
            };

            result(p1, p2)
        })
        .sum();

    println!("Day 2\n\tPart One - {part_one}\n\tPart Two - {part_two}\n");
}

fn result(p1: HandShape, p2: HandShape) -> u32 {
    if p1 == p2 {
        return p2.score() + 3;
    }
    if p2.beats(p1) {
        return p2.score() + 6;
    }
    p2.score()
}
