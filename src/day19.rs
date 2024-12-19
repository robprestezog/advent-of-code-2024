use std::collections::HashMap;

use aoc_runner_derive::aoc;

#[derive(PartialEq, Debug, Eq, Hash, Clone)]
enum Color {
    White,
    Blue,
    Black,
    Red,
    Green,
}

struct Towel {
    stripes: Vec<Color>,
}

fn color(c: char) -> Color {
    match c {
        'w' => Color::White,
        'u' => Color::Blue,
        'b' => Color::Black,
        'r' => Color::Red,
        'g' => Color::Green,
        _ => panic!("Unexpected color"),
    }
}

fn towel(stripes: &str) -> Towel {
    Towel {
        stripes: stripes.chars().map(color).collect(),
    }
}

fn can_make<'a>(
    stripes: &'a [Color],
    towels: &Vec<Towel>,
    lookup: &mut HashMap<&'a [Color], bool>,
) -> bool {
    if stripes.len() == 0 {
        return true;
    }

    if lookup.contains_key(stripes) {
        return lookup[stripes];
    }

    for towel in towels {
        if towel.stripes.len() <= stripes.len() {
            if towel.stripes[..] == stripes[..towel.stripes.len()] {
                if can_make(&stripes[towel.stripes.len()..], towels, lookup) {
                    lookup.insert(stripes, true);
                    return true;
                }
            }
        }
    }
    lookup.insert(stripes, false);
    false
}

#[aoc(day19, part1)]
pub fn part1(input: &str) -> u32 {
    let mut iter = input.lines();

    let towels: Vec<Towel> = iter.next().unwrap().split(", ").map(towel).collect();
    iter.next();
    let designs: Vec<Towel> = iter.map(towel).collect();

    let mut lookup: HashMap<&[Color], bool> = HashMap::new();

    let mut count = 0;
    for design in &designs {
        if can_make(&design.stripes, &towels, &mut lookup) {
            count += 1;
        }
    }
    count
}

fn make_ways<'a>(
    stripes: &'a [Color],
    towels: &Vec<Towel>,
    lookup: &mut HashMap<&'a [Color], u64>,
) -> u64 {
    if stripes.len() == 0 {
        return 1;
    }

    if lookup.contains_key(stripes) {
        return lookup[stripes];
    }

    let mut ways = 0;

    for towel in towels {
        if towel.stripes.len() <= stripes.len() {
            if towel.stripes[..] == stripes[..towel.stripes.len()] {
                ways += make_ways(&stripes[towel.stripes.len()..], towels, lookup);
            }
        }
    }
    lookup.insert(stripes, ways);
    ways
}

#[aoc(day19, part2)]
pub fn part2(input: &str) -> u64 {
    let mut iter = input.lines();

    let towels: Vec<Towel> = iter.next().unwrap().split(", ").map(towel).collect();
    iter.next();
    let designs: Vec<Towel> = iter.map(towel).collect();

    let mut lookup: HashMap<&[Color], u64> = HashMap::new();

    let mut count = 0;
    for design in &designs {
        count += make_ways(&design.stripes, &towels, &mut lookup);
    }
    count
}
