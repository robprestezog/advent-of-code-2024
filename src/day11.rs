use std::collections::HashMap;

use aoc_runner_derive::aoc;

#[aoc(day11, part1)]
pub fn part1(input: &str) -> u64 {
    let mut count_lookup: HashMap<(u64, u8), u64> = HashMap::new();
    input
        .split_ascii_whitespace()
        .map(|digits| digits.parse().unwrap())
        .map(|num| count(num, 25, &mut count_lookup))
        .sum()
}

#[aoc(day11, part2)]
pub fn part2(input: &str) -> u64 {
    let mut count_lookup: HashMap<(u64, u8), u64> = HashMap::new();
    input
        .split_ascii_whitespace()
        .map(|digits| digits.parse().unwrap())
        .map(|num| count(num, 75, &mut count_lookup))
        .sum()
}

#[derive(Clone)]
enum Blink {
    One(u64),
    Two(u64, u64),
}

fn blink(stone: u64) -> Blink {
    match stone {
        0 => Blink::One(1),
        1..10 => Blink::One(stone * 2024),
        10..100 => Blink::Two(stone / 10, stone % 10),
        100..1000 => Blink::One(stone * 2024),
        1000..10000 => Blink::Two(stone / 100, stone % 100),
        10000..100000 => Blink::One(stone * 2024),
        100000..1000000 => Blink::Two(stone / 1000, stone % 1000),
        1000000..10000000 => Blink::One(stone * 2024),
        10000000..100000000 => Blink::Two(stone / 10000, stone % 10000),
        100000000..1000000000 => Blink::One(stone * 2024),
        1000000000..10000000000 => Blink::Two(stone / 100000, stone % 100000),
        10000000000..100000000000 => Blink::One(stone * 2024),
        100000000000..1000000000000 => Blink::Two(stone / 1000000, stone % 1000000),
        1000000000000..10000000000000 => Blink::One(stone * 2024),
        10000000000000..100000000000000 => Blink::Two(stone / 10000000, stone % 10000000),
        100000000000000..1000000000000000 => Blink::One(stone * 2024),
        1000000000000000..10000000000000000 => Blink::Two(stone / 100000000, stone % 100000000),
        _ => panic!("Likely overflow"),
    }
}

fn count(num: u64, blinks: u8, count_lookup: &mut HashMap<(u64, u8), u64>) -> u64 {
    if blinks == 0 {
        return 1;
    }
    if let Some(c) = count_lookup.get(&(num, blinks)) {
        return *c;
    }
    let c = match blink(num) {
        Blink::One(x) => count(x, blinks - 1, count_lookup),
        Blink::Two(x, y) => count(x, blinks - 1, count_lookup) + count(y, blinks - 1, count_lookup),
    };
    count_lookup.insert((num, blinks), c);
    c
}
