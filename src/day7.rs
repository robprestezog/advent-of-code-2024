use aoc_runner_derive::aoc;

fn check(line: &str) -> u64 {
    let mut iter = line.split_ascii_whitespace();

    let answer = iter
        .next()
        .unwrap()
        .split_once(':')
        .unwrap()
        .0
        .parse()
        .unwrap();

    let terms: Vec<u64> = iter.map(|digits| digits.parse().unwrap()).collect();

    if search(answer, &terms, terms.len()) {
        answer
    } else {
        0
    }
}

fn search(answer: u64, terms: &Vec<u64>, max_index: usize) -> bool {
    if max_index == 1 {
        return answer == terms[0];
    }
    // try division
    if answer % terms[max_index - 1] == 0 && answer / terms[max_index - 1] >= terms[0] {
        if search(answer / terms[max_index - 1], terms, max_index - 1) {
            return true;
        }
    }
    // try subtraction
    if answer - terms[max_index - 1] >= terms[0] {
        if search(answer - terms[max_index - 1], terms, max_index - 1) {
            return true;
        }
    }
    false
}

#[aoc(day7, part1)]
fn part1(input: &str) -> u64 {
    input.lines().map(check).sum()
}

fn check2(line: &str) -> u64 {
    let mut iter = line.split_ascii_whitespace();

    let answer = iter
        .next()
        .unwrap()
        .split_once(':')
        .unwrap()
        .0
        .parse()
        .unwrap();

    let terms: Vec<u64> = iter.map(|digits| digits.parse().unwrap()).collect();

    if search2(answer, &terms, terms.len()) {
        answer
    } else {
        0
    }
}

fn get_divisor(suffix: u64) -> u64 {
    match suffix {
        0 => panic!("Unexpected zero term"),
        1..10 => 10,
        10..100 => 100,
        100..1000 => 1000,
        _ => panic!("Unexpected large term"),
    }
}

fn strip_suffix(num: u64, suffix: u64) -> u64 {
    if suffix >= num {
        return 0;
    }
    let divisor = get_divisor(suffix);

    if (num - suffix) % divisor == 0 {
        return (num - suffix) / divisor;
    }
    0
}

fn search2(answer: u64, terms: &Vec<u64>, max_index: usize) -> bool {
    if max_index == 1 {
        return answer == terms[0];
    }
    // try division
    if answer % terms[max_index - 1] == 0 && answer / terms[max_index - 1] >= terms[0] {
        if search2(answer / terms[max_index - 1], terms, max_index - 1) {
            return true;
        }
    }
    // try de-concatenation
    // if the right part of answer is the last term and the left part is large enough, recurse!
    let stripped = strip_suffix(answer, terms[max_index - 1]);
    if stripped >= terms[0] {
        if search2(stripped, terms, max_index - 1) {
            return true;
        }
    }
    // try subtraction
    if answer - terms[max_index - 1] >= terms[0] {
        if search2(answer - terms[max_index - 1], terms, max_index - 1) {
            return true;
        }
    }
    false
}
#[aoc(day7, part2)]
fn part2(input: &str) -> u64 {
    input.lines().map(check2).sum()
}
