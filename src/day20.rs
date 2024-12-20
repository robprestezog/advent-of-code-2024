use std::cmp::min;

use aoc_runner_derive::aoc;

const MIN_CHEAT: usize = 100;

#[derive(PartialEq)]
enum Space {
    Empty,
    Wall,
    Path(usize),
}

struct Position {
    x: usize,
    y: usize,
}

struct Map {
    grid: Vec<Vec<Space>>,
    position: Position,
    time: usize,
    width: usize,
    height: usize,
}

impl Map {
    fn race(&mut self) -> bool {
        if self.position.y > 0 && self.grid[self.position.y - 1][self.position.x] == Space::Empty {
            self.position.y -= 1;
            self.time += 1;
            self.grid[self.position.y][self.position.x] = Space::Path(self.time);
            true
        } else if self.position.y + 1 < self.height
            && self.grid[self.position.y + 1][self.position.x] == Space::Empty
        {
            self.position.y += 1;
            self.time += 1;
            self.grid[self.position.y][self.position.x] = Space::Path(self.time);
            true
        } else if self.position.x > 0
            && self.grid[self.position.y][self.position.x - 1] == Space::Empty
        {
            self.position.x -= 1;
            self.time += 1;
            self.grid[self.position.y][self.position.x] = Space::Path(self.time);
            true
        } else if self.position.x + 1 < self.width
            && self.grid[self.position.y][self.position.x + 1] == Space::Empty
        {
            self.position.x += 1;
            self.time += 1;
            self.grid[self.position.y][self.position.x] = Space::Path(self.time);
            true
        } else {
            false
        }
    }

    fn count_cheats(&self, max_time: usize) -> usize {
        let mut cheats = 0;
        for dy in 0..=(min(self.position.y, max_time)) {
            for dx in 0..=(min(self.position.x, max_time - dy)) {
                if let Space::Path(t) = self.grid[self.position.y - dy][self.position.x - dx] {
                    if self.time - t >= MIN_CHEAT + dx + dy {
                        cheats += 1;
                    }
                }
            }
            for dx in 1..=(min(self.width - 1 - self.position.x, max_time - dy)) {
                if let Space::Path(t) = self.grid[self.position.y - dy][self.position.x + dx] {
                    if self.time - t >= MIN_CHEAT + dx + dy {
                        cheats += 1;
                    }
                }
            }
        }
        for dy in 1..=(min(self.height - 1 - self.position.y, max_time)) {
            for dx in 0..=(min(self.position.x, max_time - dy)) {
                if let Space::Path(t) = self.grid[self.position.y + dy][self.position.x - dx] {
                    if self.time - t >= MIN_CHEAT + dx + dy {
                        cheats += 1;
                    }
                }
            }
            for dx in 1..=(min(self.width - 1 - self.position.x, max_time - dy)) {
                if let Space::Path(t) = self.grid[self.position.y + dy][self.position.x + dx] {
                    if self.time - t >= MIN_CHEAT + dx + dy {
                        cheats += 1;
                    }
                }
            }
        }
        cheats
    }
}

fn parse(input: &str) -> Map {
    let mut position = Position { x: 0, y: 0 };
    let mut width = 0;
    let mut height = 0;
    let time = 0;

    let grid = input
        .lines()
        .enumerate()
        .map(|(y, line)| {
            height += 1;
            width = line.len();
            line.chars()
                .enumerate()
                .map(|(x, c)| match c {
                    '#' => Space::Wall,
                    '.' => Space::Empty,
                    'E' => Space::Empty,
                    'S' => {
                        position = Position { x, y };
                        Space::Path(time)
                    }
                    _ => panic!("Unexpected char"),
                })
                .collect()
        })
        .collect();

    Map {
        grid,
        position,
        time,
        width,
        height,
    }
}

#[aoc(day20, part1)]
pub fn part1(input: &str) -> usize {
    let mut map = parse(input);

    let mut cheats = 0;
    while map.race() {
        cheats += map.count_cheats(2);
    }
    cheats
}

#[aoc(day20, part2)]
pub fn part2(input: &str) -> usize {
    let mut map = parse(input);

    let mut cheats = 0;
    while map.race() {
        cheats += map.count_cheats(20);
    }
    cheats
}
