use aoc_runner_derive::aoc;

const SIZE: usize = 20 * 20 * 20 * 20;
const MIN_INDEX: u32 = 20 * 20 * 20;

fn next(mut num: u32) -> u32 {
    num = (num ^ (num << 6)) & 0xffffff;
    num ^= num >> 5;
    (num ^ (num << 11)) & 0xffffff
}

#[aoc(day22, part1)]
pub fn part1(input: &str) -> u64 {
    input
        .lines()
        .map(|line| (0..2000).fold(line.parse().unwrap(), |acc, _| next(acc)) as u64)
        .sum()
}

#[aoc(day22, part2)]
pub fn part2(input: &str) -> u16 {
    let mut totals = [0 as u16; SIZE];
    let mut seen = [0 as u16; SIZE];

    input.lines().enumerate().for_each(|(i, line)| {
        let mut num = line.parse().unwrap();
        let mut cur_digit = num % 10;
        let mi = i as u16 + 1;
        let mut index: u32 = 0;
        for _ in 0..2000 {
            num = next(num);
            let next_digit = num % 10;
            index = ((index * 20) + 10 + next_digit - cur_digit) % SIZE as u32;
            cur_digit = next_digit;

            if index >= MIN_INDEX {
                if seen[index as usize] < mi {
                    seen[index as usize] = mi;
                    totals[index as usize] += cur_digit as u16;
                }
            }
        }
    });

    totals.into_iter().max().unwrap()
}
