use std::collections::HashMap;

use aoc_runner_derive::aoc;

#[aoc(day11, part1)]
pub fn part1(input: &str) -> u64 {
    let mut lookup: HashMap<(u64, u8), u64> = HashMap::new();
    input
        .split_ascii_whitespace()
        .map(|digits| digits.parse().unwrap())
        .map(|num| count(num, 25, &mut lookup))
        .sum()
}

fn count(num: u64, blinks: u8, lookup: &mut HashMap<(u64, u8), u64>) -> u64 {
    if blinks == 0 {
        return 1;
    }
    if let Some(c) = lookup.get(&(num, blinks)) {
        return *c;
    }
    let c = if num == 0 {
        count(1, blinks - 1, lookup)
    } else {
        let num_string = num.to_string();
        if num_string.len() % 2 == 0 {
            let (first, second) = num_string.split_at(num_string.len() / 2);
            count(first.parse().unwrap(), blinks - 1, lookup)
                + count(second.parse().unwrap(), blinks - 1, lookup)
        } else {
            count(num * 2024, blinks - 1, lookup)
        }
    };
    lookup.insert((num, blinks), c);
    c
}

#[aoc(day11, part2)]
pub fn part2(input: &str) -> u64 {
    let mut lookup: HashMap<(u64, u8), u64> = HashMap::new();
    input
        .split_ascii_whitespace()
        .map(|digits| digits.parse().unwrap())
        .map(|num| count(num, 75, &mut lookup))
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_example() {
        assert_eq!(part1("125 17"), 55312);
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2(""), "");
    }
}
