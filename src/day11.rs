use std::collections::HashMap;

use aoc_runner_derive::aoc;

#[aoc(day11, part1)]
pub fn part1(input: &str) -> u64 {
    let mut count_lookup: HashMap<(u64, u8), u64> = HashMap::new();
    let mut blink_lookup: HashMap<u64, Blink> = HashMap::new();
    input
        .split_ascii_whitespace()
        .map(|digits| digits.parse().unwrap())
        .map(|num| count(num, 25, &mut count_lookup, &mut blink_lookup))
        .sum()
}

#[aoc(day11, part2)]
pub fn part2(input: &str) -> u64 {
    let mut count_lookup: HashMap<(u64, u8), u64> = HashMap::new();
    let mut blink_lookup: HashMap<u64, Blink> = HashMap::new();
    input
        .split_ascii_whitespace()
        .map(|digits| digits.parse().unwrap())
        .map(|num| count(num, 75, &mut count_lookup, &mut blink_lookup))
        .sum()
}

#[derive(Clone)]
enum Blink {
    One(u64),
    Two(u64, u64),
}

fn blink(stone: u64, blink_lookup: &mut HashMap<u64, Blink>) -> Blink {
    blink_lookup
        .entry(stone)
        .or_insert({
            if stone == 0 {
                Blink::One(1)
            } else {
                let num_string = stone.to_string();
                if num_string.len() % 2 == 0 {
                    let (first, second) = num_string.split_at(num_string.len() / 2);
                    Blink::Two(first.parse().unwrap(), second.parse().unwrap())
                } else {
                    Blink::One(stone * 2024)
                }
            }
        })
        .clone()
}

fn count(
    num: u64,
    blinks: u8,
    count_lookup: &mut HashMap<(u64, u8), u64>,
    blink_lookup: &mut HashMap<u64, Blink>,
) -> u64 {
    if blinks == 0 {
        return 1;
    }
    if blinks == 1 {
        if num.to_string().len() % 2 == 0 {
            return 2;
        } else {
            return 1;
        }
    }
    if let Some(c) = count_lookup.get(&(num, blinks)) {
        return *c;
    }
    let c = match blink(num, blink_lookup) {
        Blink::One(x) => count(x, blinks - 1, count_lookup, blink_lookup),
        Blink::Two(x, y) => {
            count(x, blinks - 1, count_lookup, blink_lookup)
                + count(y, blinks - 1, count_lookup, blink_lookup)
        }
    };
    count_lookup.insert((num, blinks), c);
    c
}
