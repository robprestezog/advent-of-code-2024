use std::collections::HashMap;
use std::collections::HashSet;

use aoc_runner_derive::{aoc, aoc_generator};

struct Inputs {
    rules: HashMap<u8, HashSet<u8>>,
    updates: Vec<Vec<u8>>,
}

#[aoc_generator(day5)]
fn parse(input: &str) -> Inputs {
    let mut inputs = Inputs {
        rules: HashMap::new(),
        updates: vec![],
    };
    let mut more_rules = true;
    for line in input.lines() {
        if line.len() == 0 {
            more_rules = false;
            continue;
        }
        if more_rules {
            let mut iter = line.split('|').map(|x| x.parse().unwrap());
            let before = iter.next().unwrap();
            let after = iter.next().unwrap();
            inputs
                .rules
                .entry(after)
                .and_modify(|e| {
                    e.insert(before);
                })
                .or_insert(HashSet::from([before]));
        } else {
            let update = line.split(',').map(|d| d.parse().unwrap()).collect();
            inputs.updates.push(update);
        }
    }

    inputs
}

#[aoc(day5, part1)]
fn part1(inputs: &Inputs) -> u32 {
    let mut total = 0;
    for update in inputs.updates.iter() {
        if is_good(update, &inputs.rules) {
            total += update[update.len() / 2] as u32;
        }
    }
    total
}

fn is_good(update: &Vec<u8>, rules: &HashMap<u8, HashSet<u8>>) -> bool {
    let mut forbidden: HashSet<u8> = HashSet::new();
    for page in update {
        if forbidden.contains(page) {
            return false;
        }
        if let Some(before_set) = rules.get(page) {
            forbidden.extend(before_set);
        }
    }
    true
}

#[aoc(day5, part2)]
fn part2(inputs: &Inputs) -> u32 {
    let mut total = 0;
    for update in inputs.updates.iter() {
        if !is_good(update, &inputs.rules) {
            total += median(update.clone(), &inputs.rules) as u32;
        }
    }
    total
}

fn median(mut update: Vec<u8>, rules: &HashMap<u8, HashSet<u8>>) -> u8 {
    let compare = |a: &u8, b: &u8| {
        if let Some(before_set) = rules.get(b) {
            if before_set.contains(a) {
                return std::cmp::Ordering::Less;
            }
        }
        if let Some(before_set) = rules.get(a) {
            if before_set.contains(b) {
                return std::cmp::Ordering::Greater;
            }
        }
        std::cmp::Ordering::Equal
    };
    update.sort_by(compare);
    update[update.len() / 2]
}
