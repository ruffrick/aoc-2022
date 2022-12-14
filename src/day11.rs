use std::collections::VecDeque;

#[derive(Clone, Debug, Default)]
struct Monkey {
    index: usize,
    items: VecDeque<u64>,
    operator: Operator,
    operand: Operand,
    test: u64,
    if_true: usize,
    if_false: usize,
    inspected_items: u64,
}

#[derive(Clone, Debug, Default)]
enum Operator {
    Multiply,
    #[default]
    Add,
}

impl TryFrom<&str> for Operator {
    type Error = ();

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            "*" => Ok(Self::Multiply),
            "+" => Ok(Self::Add),
            _ => Err(()),
        }
    }
}

#[derive(Clone, Debug, Default)]
enum Operand {
    Value(u64),
    #[default]
    Old,
}

impl TryFrom<&str> for Operand {
    type Error = ();

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            "old" => Ok(Self::Old),
            val => Ok(Self::Value(val.parse().unwrap())),
        }
    }
}

#[derive(Debug)]
enum Line {
    Monkey(usize),
    StartingItems(VecDeque<u64>),
    Operation(Operator, Operand),
    Test(u64),
    IfTrue(usize),
    IfFalse(usize),
}

impl TryFrom<&str> for Line {
    type Error = ();

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        return if let Some(value) = value
            .strip_prefix("Monkey ")
            .and_then(|value| value.strip_suffix(':'))
        {
            Ok(Line::Monkey(value.parse().unwrap()))
        } else if let Some(value) = value.strip_prefix("  Starting items: ") {
            Ok(Line::StartingItems(
                value
                    .split(", ")
                    .map(|item| item.parse().unwrap())
                    .collect(),
            ))
        } else if let Some(value) = value.strip_prefix("  Operation: new = old ") {
            let value: Vec<&str> = value.split_whitespace().collect();
            Ok(Line::Operation(
                value[0].try_into().unwrap(),
                value[1].try_into().unwrap(),
            ))
        } else if let Some(value) = value.strip_prefix("  Test: divisible by ") {
            Ok(Line::Test(value.parse().unwrap()))
        } else if let Some(value) = value.strip_prefix("    If true: throw to monkey ") {
            Ok(Line::IfTrue(value.parse().unwrap()))
        } else if let Some(value) = value.strip_prefix("    If false: throw to monkey ") {
            Ok(Line::IfFalse(value.parse().unwrap()))
        } else {
            Err(())
        };
    }
}

pub fn solve() {
    let input = crate::input("day11.txt");
    let mut monkeys = Vec::new();
    for monkey in input.split("\n\n") {
        let lines: Vec<Line> = monkey
            .lines()
            .map(|line| line.try_into().unwrap())
            .collect();
        let mut monkey = Monkey::default();
        for line in lines {
            match line {
                Line::Monkey(index) => monkey.index = index,
                Line::StartingItems(items) => monkey.items = items,
                Line::Operation(operator, operand) => {
                    monkey.operator = operator;
                    monkey.operand = operand;
                }
                Line::Test(test) => monkey.test = test,
                Line::IfTrue(if_true) => monkey.if_true = if_true,
                Line::IfFalse(if_false) => monkey.if_false = if_false,
            }
        }
        monkeys.push(monkey);
    }

    let part_one = monkey_business(monkeys.clone(), 20, None);
    let modulo = monkeys
        .iter()
        .map(|monkey| monkey.test)
        .reduce(|acc, test| acc * test);
    let part_two = monkey_business(monkeys, 10000, modulo);
    println!("Day 11\n\tPart One - {part_one}\n\tPart Two - {part_two}\n",);
}

fn monkey_business(mut monkeys: Vec<Monkey>, rounds: u32, modulo: Option<u64>) -> u64 {
    let indices: Vec<(usize, usize, usize)> = monkeys
        .iter()
        .map(|monkey| (monkey.index, monkey.if_true, monkey.if_false))
        .collect();
    for _ in 0..rounds {
        for (index, if_true, if_false) in indices.iter() {
            if let Ok([monkey, if_true, if_false]) =
                monkeys.get_many_mut([*index, *if_true, *if_false])
            {
                while !monkey.items.is_empty() {
                    let mut item = monkey.items.pop_front().unwrap();
                    let operand = match monkey.operand {
                        Operand::Value(value) => value,
                        Operand::Old => item,
                    };
                    match monkey.operator {
                        Operator::Multiply => item *= operand,
                        Operator::Add => item += operand,
                    }
                    if let Some(modulo) = modulo {
                        item %= modulo;
                    } else {
                        item /= 3;
                    }
                    if item % monkey.test == 0 {
                        if_true.items.push_back(item);
                    } else {
                        if_false.items.push_back(item);
                    }
                    monkey.inspected_items += 1;
                }
            }
        }
    }
    monkeys.sort_by_key(|monkey| monkey.inspected_items);
    monkeys
        .iter()
        .map(|monkey| monkey.inspected_items)
        .rev()
        .take(2)
        .reduce(|acc, inspected_items| acc * inspected_items)
        .unwrap()
}
