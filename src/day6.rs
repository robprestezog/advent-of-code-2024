use std::collections::HashSet;

use aoc_runner_derive::aoc;

enum State {
    Empty,
    Obstacle,
    Visited,
}

enum Direction {
    Up,
    Right,
    Down,
    Left,
}

struct Map {
    rows: usize,
    columns: usize,
    grid: Vec<Vec<State>>,
    x: usize,
    y: usize,
    direction: Direction,
    visited: u32,
}

impl Direction {
    fn turn(&mut self) {
        *self = match self {
            Direction::Up => Direction::Right,
            Direction::Right => Direction::Down,
            Direction::Down => Direction::Left,
            Direction::Left => Direction::Up,
        };
    }
}

fn parse(input: &str) -> Map {
    let mut map = Map {
        rows: 0,
        columns: 0,
        grid: vec![],
        x: 0,
        y: 0,
        direction: Direction::Up,
        visited: 0,
    };

    input.lines().enumerate().for_each(|(y, line)| {
        map.rows += 1;
        map.columns = line.len();
        map.grid.push(
            line.chars()
                .enumerate()
                .map(|(x, c)| match c {
                    '.' => State::Empty,
                    '#' => State::Obstacle,
                    '^' => {
                        map.x = x;
                        map.y = y;
                        map.direction = Direction::Up;
                        map.visited += 1;
                        State::Visited
                    }
                    _ => panic!("Unexpected character"),
                })
                .collect(),
        );
    });
    map
}

enum Outcome {
    Moving,
    Edge,
}

fn move_guard(mut map: Map) -> (Map, Outcome) {
    let (x, y) = match map.direction {
        Direction::Up => {
            if map.y == 0 {
                return (map, Outcome::Edge);
            }
            (map.x, map.y - 1)
        }
        Direction::Right => {
            if map.x + 1 == map.columns {
                return (map, Outcome::Edge);
            }
            (map.x + 1, map.y)
        }
        Direction::Down => {
            if map.y + 1 == map.rows {
                return (map, Outcome::Edge);
            }
            (map.x, map.y + 1)
        }
        Direction::Left => {
            if map.x == 0 {
                return (map, Outcome::Edge);
            }
            (map.x - 1, map.y)
        }
    };
    match map.grid[y][x] {
        State::Empty => {
            map.grid[y][x] = State::Visited;
            map.visited += 1;
            map.x = x;
            map.y = y;
        }
        State::Visited => {
            map.x = x;
            map.y = y;
        }
        State::Obstacle => {
            map.direction.turn();
        }
    }
    (map, Outcome::Moving)
}

#[aoc(day6, part1)]
fn part1(input: &str) -> u32 {
    let mut map = parse(input);
    let mut outcome;

    loop {
        (map, outcome) = move_guard(map);
        match outcome {
            Outcome::Edge => {
                return map.visited;
            }
            _ => (),
        }
    }
}

#[aoc(day6, part2)]
fn part2(input: &str) -> u32 {
    let mut map = parse(input);
    let mut outcome;

    let start = (map.x, map.y);

    let mut candidates: HashSet<(usize, usize)> = HashSet::new();

    loop {
        candidates.insert((map.x, map.y));
        (map, outcome) = move_guard(map);
        match outcome {
            Outcome::Edge => {
                break;
            }
            _ => (),
        }
    }

    let mut total = 0;

    for position in candidates {
        if position == start {
            continue;
        }
        map.x = start.0;
        map.y = start.1;
        map.direction = Direction::Up;

        map.grid[position.1][position.0] = State::Obstacle;
        let mut up_states: HashSet<(usize, usize)> = HashSet::new();
        loop {
            match map.direction {
                Direction::Up => {
                    if up_states.contains(&(map.x, map.y)) {
                        total += 1;
                        break;
                    }
                    up_states.insert((map.x, map.y));
                }
                _ => (),
            }
            (map, outcome) = move_guard(map);
            match outcome {
                Outcome::Edge => {
                    break;
                }
                _ => (),
            }
        }
        map.grid[position.1][position.0] = State::Visited;
    }

    total
}
