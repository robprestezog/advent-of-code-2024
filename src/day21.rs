use std::collections::HashMap;

use aoc_runner_derive::aoc;

// Keypad one
// +---+---+---+
// | 7 | 8 | 9 |
// +---+---+---+
// | 4 | 5 | 6 |
// +---+---+---+
// | 1 | 2 | 3 |
// +---+---+---+
//     | 0 | A |
//     +---+---+
enum OneKey {
    K0,
    K1,
    K2,
    K3,
    K4,
    K5,
    K6,
    K7,
    K8,
    K9,
    KA,
}

// Generate keypad 2 sequences that press a key from a given start state.
fn get_2_options_from_1_transition(start: &OneKey, target: &OneKey) -> Vec<Vec<TwoKey>> {
    use OneKey::*;
    use TwoKey::*;
    match (start, target) {
        (K0, K0)
        | (K1, K1)
        | (K2, K2)
        | (K3, K3)
        | (K4, K4)
        | (K5, K5)
        | (K6, K6)
        | (K7, K7)
        | (K8, K8)
        | (K9, K9)
        | (KA, KA) => vec![vec![A]],
        (K0, K1) => vec![vec![U, L, A]],
        (K0, K2) | (K1, K4) | (K2, K5) | (K3, K6) | (K4, K7) | (K5, K8) | (K6, K9) | (KA, K3) => {
            vec![vec![U, A]]
        }
        (K0, K3) | (K1, K5) | (K2, K6) | (K4, K8) | (K5, K9) => vec![vec![U, R, A], vec![R, U, A]],
        (K0, K4) => vec![vec![U, U, L, A]],
        (K0, K5) | (K1, K7) | (K2, K8) | (K3, K9) | (KA, K6) => vec![vec![U, U, A]],
        (K0, K6) | (K1, K8) | (K2, K9) => vec![vec![U, U, R, A], vec![R, U, U, A]],
        (K0, K7) => vec![vec![U, U, U, L, A]],
        (K0, K8) | (KA, K9) => vec![vec![U, U, U, A]],
        (K0, K9) => vec![vec![U, U, U, R, A], vec![R, U, U, U, A]],
        (K0, KA) | (K1, K2) | (K2, K3) | (K4, K5) | (K5, K6) | (K7, K8) | (K8, K9) => {
            vec![vec![R, A]]
        }
        (K1, K0) => vec![vec![R, D, A]],
        (K1, K3) | (K4, K6) | (K7, K9) => vec![vec![R, R, A]],
        (K1, K6) | (K4, K9) => vec![vec![R, R, U, A], vec![U, R, R, A]],
        (K1, K9) => vec![vec![R, R, U, U, A], vec![U, U, R, R, A]],
        (K1, KA) => vec![vec![R, R, D, A]],
        (K2, K0) | (K3, KA) | (K4, K1) | (K5, K2) | (K6, K3) | (K7, K4) | (K8, K5) | (K9, K6) => {
            vec![vec![D, A]]
        }
        (K2, K1) | (K3, K2) | (K5, K4) | (K6, K5) | (K8, K7) | (K9, K8) | (KA, K0) => {
            vec![vec![L, A]]
        }
        (K2, K4) | (K3, K5) | (K5, K7) | (K6, K8) | (KA, K2) => vec![vec![L, U, A], vec![U, L, A]],
        (K2, K7) | (K3, K8) | (KA, K5) => vec![vec![L, U, U, A], vec![U, U, L, A]],
        (K2, KA) | (K4, K2) | (K5, K3) | (K7, K5) | (K8, K6) => vec![vec![R, D, A], vec![D, R, A]],
        (K3, K0) | (K5, K1) | (K6, K2) | (K8, K4) | (K9, K5) => vec![vec![L, D, A], vec![D, L, A]],
        (K3, K1) | (K6, K4) | (K9, K7) => vec![vec![L, L, A]],
        (K3, K4) | (K6, K7) => vec![vec![L, L, U, A], vec![U, L, L, A]],
        (K3, K7) => vec![vec![L, L, U, U, A], vec![U, U, L, L, A]],
        (K4, K0) => vec![vec![R, D, D, A]],
        (K4, K3) | (K7, K6) => vec![vec![R, R, D, A], vec![D, R, R, A]],
        (K4, KA) => vec![vec![R, R, D, D, A]],
        (K5, K0) | (K6, KA) | (K7, K1) | (K8, K2) | (K9, K3) => vec![vec![D, D, A]],
        (K5, KA) | (K7, K2) | (K8, K3) => vec![vec![R, D, D, A], vec![D, D, R, A]],
        (K6, K0) | (K8, K1) | (K9, K2) => vec![vec![L, D, D, A], vec![D, D, L, A]],
        (K6, K1) | (K9, K4) => vec![vec![L, L, D, A], vec![D, L, L, A]],
        (K7, K0) => vec![vec![R, D, D, D, A]],
        (K7, K3) => vec![vec![R, R, D, D, A]],
        (K7, KA) => vec![vec![R, R, D, D, D, A]],
        (K8, K0) | (K9, KA) => vec![vec![D, D, D, A]],
        (K8, KA) => vec![vec![D, D, D, R, A], vec![R, D, D, D, A]],
        (K9, K0) => vec![vec![D, D, D, L, A], vec![L, D, D, D, A]],
        (K9, K1) => vec![vec![D, D, L, L, A], vec![L, L, D, D, A]],
        (KA, K1) => vec![vec![U, L, L, A]],
        (KA, K4) => vec![vec![U, U, L, L, A]],
        (KA, K7) => vec![vec![U, U, U, L, L, A]],
        (KA, K8) => vec![vec![U, U, U, L, A], vec![L, U, U, U, A]],
    }
}

// Find the shortest length keypad 4 sequence to press a key from a given start state.
fn get_shortest_from_1_transition(
    start: &OneKey,
    target: &OneKey,
    middle_bots: u32,
    lookup: &mut HashMap<(TwoKey, TwoKey, u32), u64>,
) -> u64 {
    let mut shortest = None;
    for option in get_2_options_from_1_transition(start, target) {
        let mut current = TwoKey::A;
        let mut length = 0;
        for next in option {
            length += get_shortest_from_2_transition(current, next, middle_bots, lookup);
            current = next;
        }
        if shortest == None || shortest.unwrap() > length {
            shortest = Some(length);
        }
    }
    shortest.unwrap()
}

// Find the shortest length keypad 4 sequence to press a key from a given start state.
fn get_shortest_from_2_transition(
    start: TwoKey,
    target: TwoKey,
    middle_bots: u32,
    lookup: &mut HashMap<(TwoKey, TwoKey, u32), u64>,
) -> u64 {
    if lookup.contains_key(&(start, target, middle_bots)) {
        return *lookup.get(&(start, target, middle_bots)).unwrap();
    }

    let mut shortest = None;
    for option in get_3_options_from_2_transition(start, target) {
        let mut current = TwoKey::A;
        let mut length = 0;
        if middle_bots > 1 {
            for next in option {
                length += get_shortest_from_2_transition(current, next, middle_bots - 1, lookup);
                current = next;
            }
        } else {
            length = option.len() as u64;
        }
        if shortest == None || shortest.unwrap() > length {
            shortest = Some(length);
        }
    }
    lookup.insert((start, target, middle_bots), shortest.unwrap());
    shortest.unwrap()
}

// Generate keypad 3 sequences that press a key from a given start state.
fn get_3_options_from_2_transition(start: TwoKey, target: TwoKey) -> Vec<Vec<TwoKey>> {
    use TwoKey::*;
    match (start, target) {
        (U, U) | (D, D) | (L, L) | (R, R) | (A, A) => vec![vec![A]],
        (U, D) | (D, U) | (L, R) | (R, L) => panic!("Unexpected transition"),
        (U, L) => vec![vec![D, L, A]],
        (U, R) => vec![vec![D, R, A], vec![R, D, A]],
        (U, A) | (D, R) | (L, D) => vec![vec![R, A]],
        (D, L) | (R, D) | (A, U) => vec![vec![L, A]],
        (D, A) => vec![vec![U, R, A], vec![R, U, A]],
        (L, U) => vec![vec![R, U, A]],
        (L, A) => vec![vec![R, R, U, A]],
        (R, U) => vec![vec![U, L, A], vec![L, U, A]],
        (R, A) => vec![vec![U, A]],
        (A, D) => vec![vec![D, L, A], vec![L, D, A]],
        (A, L) => vec![vec![D, L, L, A]],
        (A, R) => vec![vec![D, A]],
    }
}

// Keypad two
//     +---+---+
//     | ^ | A |
// +---+---+---+
// | < | v | > |
// +---+---+---+
#[derive(Eq, Hash, PartialEq, Clone, Copy)]
enum TwoKey {
    U,
    D,
    L,
    R,
    A,
}

fn get_shortest(
    line: &str,
    middle_bots: u32,
    lookup: &mut HashMap<(TwoKey, TwoKey, u32), u64>,
) -> u64 {
    let mut length = 0;
    let mut current = OneKey::KA;
    for c in line.chars() {
        let next = match c {
            '0' => OneKey::K0,
            '1' => OneKey::K1,
            '2' => OneKey::K2,
            '3' => OneKey::K3,
            '4' => OneKey::K4,
            '5' => OneKey::K5,
            '6' => OneKey::K6,
            '7' => OneKey::K7,
            '8' => OneKey::K8,
            '9' => OneKey::K9,
            'A' => OneKey::KA,
            _ => panic!("Unexpected character"),
        };
        length += get_shortest_from_1_transition(&current, &next, middle_bots, lookup);
        current = next;
    }
    length
}

#[aoc(day21, part1)]
pub fn part1(input: &str) -> u64 {
    let mut lookup = HashMap::new();
    input
        .lines()
        .map(|line| {
            let shortest = get_shortest(line, 2, &mut lookup);
            let code: u64 = line.strip_suffix('A').unwrap().parse().unwrap();
            code * shortest
        })
        .sum()
}

#[aoc(day21, part2)]
pub fn part2(input: &str) -> u64 {
    let mut lookup = HashMap::new();
    input
        .lines()
        .map(|line| {
            let shortest = get_shortest(line, 25, &mut lookup);
            let code: u64 = line.strip_suffix('A').unwrap().parse().unwrap();
            code * shortest
        })
        .sum()
}
