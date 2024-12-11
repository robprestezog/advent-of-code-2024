#[aoc_generator(day1)]
pub fn input_generator(input: &str) -> Vec<(u32, u32)> {
    input
        .lines()
        .map(|line| {
            let mut numbers = line
                .trim()
                .split_ascii_whitespace()
                .map(|number| number.parse().unwrap());
            (numbers.next().unwrap(), numbers.next().unwrap())
        })
        .collect()
}

#[aoc(day1, part1)]
pub fn part1(input: &Vec<(u32, u32)>) -> u32 {
    let max: u32 = input.iter().fold(0, |acc, (i1, i2)| {
        if i1 > i2 {
            if *i1 > acc {
                *i1
            } else {
                acc
            }
        } else {
            if *i2 > acc {
                *i2
            } else {
                acc
            }
        }
    });

    let mut buckets1: Vec<u32> = vec![0; max as usize + 1];
    let mut buckets2: Vec<u32> = vec![0; max as usize + 1];

    input.iter().for_each(|(i1, i2)| {
        buckets1[*i1 as usize] += 1;
        buckets2[*i2 as usize] += 1;
    });

    let mut i1 = 0;
    let mut i2 = 0;
    let mut c1 = 0;
    let mut c2 = 0;
    let mut total: u32 = 0;

    loop {
        while c1 == 0 && i1 < max {
            i1 += 1;
            c1 = buckets1[i1 as usize];
        }
        while c2 == 0 && i2 < max {
            i2 += 1;
            c2 = buckets2[i2 as usize];
        }
        if c1 > 0 && c2 > 0 {
            if c1 > c2 {
                if i1 > i2 {
                    total += (i1 - i2) * c2;
                } else {
                    total += (i2 - i1) * c2;
                }
                c1 -= c2;
                c2 = 0;
            } else {
                if i1 > i2 {
                    total += (i1 - i2) * c1;
                } else {
                    total += (i2 - i1) * c1;
                }
                c2 -= c1;
                c1 = 0;
            }
        } else {
            break;
        }
    }

    total
}

#[aoc(day1, part2)]
pub fn part2(input: &Vec<(u32, u32)>) -> u32 {
    let max: u32 = input.iter().fold(0, |acc, (i1, i2)| {
        if i1 > i2 {
            if *i1 > acc {
                *i1
            } else {
                acc
            }
        } else {
            if *i2 > acc {
                *i2
            } else {
                acc
            }
        }
    });

    let mut buckets: Vec<(u32, u32)> = vec![(0, 0); max as usize + 1];

    input.iter().for_each(|(i1, i2)| {
        buckets[*i1 as usize].0 += 1;
        buckets[*i2 as usize].1 += 1;
    });

    buckets
        .into_iter()
        .enumerate()
        .fold(0, |acc, (i, (c1, c2))| acc + (i as u32 * c1 * c2))
}
