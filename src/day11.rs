use std::collections::VecDeque;

use regex::Regex;

#[derive(Clone, Debug)]
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

#[derive(Clone, Debug)]
enum Operator {
    Multiply,
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

#[derive(Clone, Debug)]
enum Operand {
    Value(u64),
    Old,
}

impl TryFrom<&str> for Operand {
    type Error = ();

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            "old" => Ok(Self::Old),
            _ => Ok(Self::Value(value.parse().unwrap())),
        }
    }
}

pub fn solve() {
    let input = crate::input("day11.txt");

    let regex = Regex::new(r"Monkey (\d+):\n {2}Starting items: ([\d, ]+)\n {2}Operation: new = old ([+*]) (.+)\n {2}Test: divisible by (\d+)\n {4}If true: throw to monkey (\d+)\n {4}If false: throw to monkey (\d+)\n").unwrap();
    let monkeys: Vec<Monkey> = regex
        .captures_iter(input.as_str())
        .map(|captures| Monkey {
            index: captures[1].parse().unwrap(),
            items: captures[2]
                .split(", ")
                .map(|num| num.parse().unwrap())
                .collect(),
            operator: captures[3].try_into().unwrap(),
            operand: captures[4].try_into().unwrap(),
            test: captures[5].parse().unwrap(),
            if_true: captures[6].parse().unwrap(),
            if_false: captures[7].parse().unwrap(),
            inspected_items: 0,
        })
        .collect();

    let part_one = monkey_business(monkeys.clone(), 20, None);
    let modulo = monkeys.iter().map(|monkey| monkey.test).product();
    let part_two = monkey_business(monkeys, 10000, Some(modulo));
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
        .product()
}
