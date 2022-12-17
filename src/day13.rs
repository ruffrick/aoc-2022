use std::cmp::Ordering;

use Packet::*;

#[derive(Clone, Eq, PartialEq)]
enum Packet {
    Int(u8),
    List(Vec<Self>),
}

impl Ord for Packet {
    fn cmp(&self, other: &Self) -> Ordering {
        match (self, other) {
            (Int(a), Int(b)) => a.cmp(b),
            (Int(a), List(_)) => List(vec![Int(*a)]).cmp(other),
            (List(_), Int(b)) => self.cmp(&List(vec![Int(*b)])),
            (List(a), List(b)) => a.cmp(b),
        }
    }
}

impl PartialOrd for Packet {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

pub fn solve() {
    let input = crate::input("day13.txt");

    let mut part_one = 0;
    for (index, pair) in input.split("\n\n").enumerate() {
        let (left, right) = pair.trim().split_once('\n').unwrap();
        let left = parse_packet(left);
        let right = parse_packet(right);

        if left <= right {
            part_one += index + 1;
        }
    }

    let mut packets: Vec<Packet> = input
        .lines()
        .filter_map(|line| {
            if !line.is_empty() {
                Some(parse_packet(line))
            } else {
                None
            }
        })
        .collect();
    let div1 = parse_packet("[[2]]");
    let div2 = parse_packet("[[6]]");
    packets.push(div1.clone());
    packets.push(div2.clone());
    packets.sort();
    let div1 = packets.binary_search(&div1).unwrap() + 1;
    let div2 = packets.binary_search(&div2).unwrap() + 1;
    let part_two = div1 * div2;
    println!("Day 13\n\tPart One - {part_one}\n\tPart Two - {part_two}\n",);
}

fn parse_packet(packet: &str) -> Packet {
    parse_bytes(packet.as_bytes(), 0).0
}

fn parse_bytes(bytes: &[u8], start: usize) -> (Packet, usize) {
    let mut result = Vec::new();

    let mut i = start;
    let mut num = String::new();

    while i < bytes.len() {
        match bytes[i] as char {
            '[' => {
                let (inner, j) = parse_bytes(bytes, i + 1);
                result.push(inner);
                i = j;
            }
            ']' => {
                if !num.is_empty() {
                    result.push(Int(num.parse().unwrap()));
                }
                return (List(result), i + 1);
            }
            ',' => {
                if !num.is_empty() {
                    result.push(Int(num.parse().unwrap()));
                }
                num = String::new();
                i += 1;
            }
            c => {
                num.push(c);
                i += 1;
            }
        }
    }

    if !num.is_empty() {
        result.push(Int(num.parse().unwrap()));
    }
    (List(result), i)
}
