use aoc_runner_derive::aoc;

const OVERFLOW_MASK: u32 = 0x88888;
const SHIM: u32 = 0x11111;

fn parse_line(line: &str) -> u32 {
    let mut out = 0;
    line.chars().for_each(|c| match c {
        '#' => {
            out *= 16;
            out += 1;
        }
        '.' => {
            out *= 16;
        }
        _ => panic!("Unexpected char"),
    });
    out
}

fn parse(input: &str) -> (Vec<u32>, Vec<u32>) {
    let mut iter = input.lines();
    let mut locks = vec![];
    let mut keys = vec![];
    loop {
        let line = match iter.next() {
            None => return (locks, keys),
            Some(x) => x,
        };
        match line {
            "#####" => {
                let mut lock = SHIM;
                lock += parse_line(iter.next().unwrap());
                lock += parse_line(iter.next().unwrap());
                lock += parse_line(iter.next().unwrap());
                lock += parse_line(iter.next().unwrap());
                lock += parse_line(iter.next().unwrap());
                locks.push(lock);
            }
            "....." => {
                let mut key = SHIM;
                key += parse_line(iter.next().unwrap());
                key += parse_line(iter.next().unwrap());
                key += parse_line(iter.next().unwrap());
                key += parse_line(iter.next().unwrap());
                key += parse_line(iter.next().unwrap());
                keys.push(key);
            }
            _ => panic!("Unexpected input line"),
        }
        iter.next();
        iter.next();
    }
}

#[aoc(day25, part1)]
pub fn part1(input: &str) -> u32 {
    let (locks, keys) = parse(input);
    let mut count = 0;
    for lock in locks {
        for key in &keys {
            if (lock + key) & OVERFLOW_MASK == 0 {
                count += 1;
            }
        }
    }
    count
}

#[aoc(day25, part2)]
fn part2(_input: &str) -> String {
    todo!()
}
