use std::collections::HashMap;
use std::collections::HashSet;

use aoc_runner_derive::aoc;

#[derive(Eq, Hash, PartialEq)]
struct Position {
    x: usize,
    y: usize,
}

#[aoc(day8, part1)]
fn part1(input: &str) -> usize {
    let mut nodes: HashMap<char, Vec<Position>> = HashMap::new();
    let mut antinodes: HashSet<Position> = HashSet::new();
    let mut width = 0;
    let mut height = 0;
    input.lines().enumerate().for_each(|(y, line)| {
        width = line.len();
        height += 1;
        line.chars().enumerate().for_each(|(x, c)| {
            if c == '.' {
                return;
            }
            nodes.entry(c).or_insert(vec![]).iter().for_each(|pos| {
                if 2 * pos.x >= x && 2 * pos.y >= y {
                    antinodes.insert(Position {
                        x: 2 * pos.x - x,
                        y: 2 * pos.y - y,
                    });
                }
                if 2 * x >= pos.x && 2 * y >= pos.y {
                    antinodes.insert(Position {
                        x: 2 * x - pos.x,
                        y: 2 * y - pos.y,
                    });
                }
            });
            nodes.get_mut(&c).unwrap().push(Position { x, y });
        });
    });

    let mut count = 0;
    for pos in antinodes.into_iter() {
        if pos.x < width && pos.y < height {
            count += 1;
        }
    }
    count
}

fn get_grid_size(input: &str) -> (usize, usize) {
    let mut width = 0;
    let mut height = 0;
    input.lines().for_each(|line| {
        width = line.len();
        height += 1;
    });
    (width, height)
}

fn gcd(mut a: usize, mut b: usize) -> usize {
    while a > 0 {
        (a, b) = (b % a, a);
    }
    b
}

#[aoc(day8, part2)]
fn part2(input: &str) -> usize {
    let (width, height) = get_grid_size(input);
    let mut nodes: HashMap<char, Vec<Position>> = HashMap::new();
    let mut antinodes: HashSet<Position> = HashSet::new();
    input.lines().enumerate().for_each(|(y, line)| {
        line.chars().enumerate().for_each(|(x, c)| {
            if c == '.' {
                return;
            }

            nodes.entry(c).or_insert(vec![]).iter().for_each(|pos| {
                // y >= pos.y
                if x >= pos.x {
                    let g = gcd(x - pos.x, y - pos.y);
                    let dx = (x - pos.x) / g;
                    let dy = (y - pos.y) / g;

                    let mut ax = x;
                    let mut ay = y;
                    loop {
                        antinodes.insert(Position { x: ax, y: ay });
                        if ax < dx || ay < dy {
                            break;
                        }
                        ax -= dx;
                        ay -= dy;
                    }
                    ax = x;
                    ay = y;
                    loop {
                        if ax + dx >= width || ay + dy >= height {
                            break;
                        }
                        ax += dx;
                        ay += dy;
                        antinodes.insert(Position { x: ax, y: ay });
                    }
                } else {
                    let g = gcd(pos.x - x, y - pos.y);
                    let dx = (pos.x - x) / g;
                    let dy = (y - pos.y) / g;
                    let mut ax = x;
                    let mut ay = y;
                    loop {
                        antinodes.insert(Position { x: ax, y: ay });
                        if ax + dx >= width || ay < dy {
                            break;
                        }
                        ax += dx;
                        ay -= dy;
                    }
                    ax = x;
                    ay = y;
                    loop {
                        if ax < dx || ay + dy >= height {
                            break;
                        }
                        ax -= dx;
                        ay += dy;
                        antinodes.insert(Position { x: ax, y: ay });
                    }
                }
            });
            nodes.get_mut(&c).unwrap().push(Position { x, y });
        });
    });

    antinodes.len()
}
