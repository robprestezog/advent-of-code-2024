use aoc_runner_derive::aoc;

use std::collections::{BinaryHeap, HashMap, HashSet};

#[derive(PartialEq, Eq, PartialOrd, Ord, Hash, Clone)]
struct Position {
    x: usize,
    y: usize,
}

#[derive(PartialEq)]
enum Space {
    Empty,
    Wall,
}

fn parse(input: &str) -> (Position, Position, Vec<Vec<Space>>) {
    let mut start = Position { x: 0, y: 0 };
    let mut end = Position { x: 0, y: 0 };
    let map = input
        .lines()
        .enumerate()
        .map(|(y, line)| {
            line.chars()
                .enumerate()
                .map(|(x, c)| match c {
                    '#' => Space::Wall,
                    '.' => Space::Empty,
                    'S' => {
                        start = Position { x, y };
                        Space::Empty
                    }
                    'E' => {
                        end = Position { x, y };
                        Space::Empty
                    }
                    _ => panic!("Unexpected character"),
                })
                .collect()
        })
        .collect();

    (start, end, map)
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Hash, Clone)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Hash, Clone)]
struct State {
    position: Position,
    direction: Direction,
}

#[derive(PartialEq, Eq, PartialOrd, Ord)]
struct Node {
    cost: i32,
    state: State,
}

#[derive(PartialEq, Eq, PartialOrd, Ord)]
struct Node2 {
    cost: i32,
    state: State,
    prior_states: Vec<State>,
}

#[aoc(day16, part1)]
pub fn part1(input: &str) -> u32 {
    let (start, end, map) = parse(input);

    let mut heap = BinaryHeap::new();
    heap.push(Node {
        cost: 0,
        state: State {
            position: start,
            direction: Direction::Right,
        },
    });

    let mut visited = HashSet::new();

    loop {
        let node = heap.pop().unwrap();
        if node.state.position == end {
            return (-node.cost) as u32;
        }
        if visited.contains(&node.state) {
            continue;
        }

        let forward = match node.state.direction {
            Direction::Up => State {
                position: Position {
                    x: node.state.position.x,
                    y: node.state.position.y - 1,
                },
                direction: Direction::Up,
            },
            Direction::Down => State {
                position: Position {
                    x: node.state.position.x,
                    y: node.state.position.y + 1,
                },
                direction: Direction::Down,
            },
            Direction::Left => State {
                position: Position {
                    x: node.state.position.x - 1,
                    y: node.state.position.y,
                },
                direction: Direction::Left,
            },
            Direction::Right => State {
                position: Position {
                    x: node.state.position.x + 1,
                    y: node.state.position.y,
                },
                direction: Direction::Right,
            },
        };
        let clockwise = match node.state.direction {
            Direction::Up => State {
                position: Position {
                    x: node.state.position.x + 1,
                    y: node.state.position.y,
                },
                direction: Direction::Right,
            },
            Direction::Down => State {
                position: Position {
                    x: node.state.position.x - 1,
                    y: node.state.position.y,
                },
                direction: Direction::Left,
            },
            Direction::Left => State {
                position: Position {
                    x: node.state.position.x,
                    y: node.state.position.y - 1,
                },
                direction: Direction::Up,
            },
            Direction::Right => State {
                position: Position {
                    x: node.state.position.x,
                    y: node.state.position.y + 1,
                },
                direction: Direction::Down,
            },
        };
        let counter_clockwise = match node.state.direction {
            Direction::Up => State {
                position: Position {
                    x: node.state.position.x - 1,
                    y: node.state.position.y,
                },
                direction: Direction::Left,
            },
            Direction::Down => State {
                position: Position {
                    x: node.state.position.x + 1,
                    y: node.state.position.y,
                },
                direction: Direction::Right,
            },
            Direction::Left => State {
                position: Position {
                    x: node.state.position.x,
                    y: node.state.position.y + 1,
                },
                direction: Direction::Down,
            },
            Direction::Right => State {
                position: Position {
                    x: node.state.position.x,
                    y: node.state.position.y - 1,
                },
                direction: Direction::Up,
            },
        };
        if map[forward.position.y][forward.position.x] == Space::Empty
            && !visited.contains(&forward)
        {
            heap.push(Node {
                cost: node.cost - 1,
                state: forward,
            });
        }
        if map[clockwise.position.y][clockwise.position.x] == Space::Empty
            && !visited.contains(&clockwise)
        {
            heap.push(Node {
                cost: node.cost - 1001,
                state: clockwise,
            });
        }
        if map[counter_clockwise.position.y][counter_clockwise.position.x] == Space::Empty
            && !visited.contains(&counter_clockwise)
        {
            heap.push(Node {
                cost: node.cost - 1001,
                state: counter_clockwise,
            });
        }
        visited.insert(node.state);
    }
}

#[aoc(day16, part2)]
pub fn part2(input: &str) -> u32 {
    let (start, end, map) = parse(input);

    let mut heap = BinaryHeap::new();
    heap.push(Node2 {
        cost: 0,
        state: State {
            position: start,
            direction: Direction::Right,
        },
        prior_states: vec![],
    });

    let mut visited: HashMap<State, Node2> = HashMap::new();

    let mut best_cost = None;
    let mut best_states = vec![];

    loop {
        let mut node = heap.pop().unwrap();
        if let Some(c) = best_cost {
            if c > node.cost {
                break;
            }
        }
        if node.state.position == end {
            best_cost = Some(node.cost);
            best_states.push(node.state.clone());
        }
        if visited.contains_key(&node.state) {
            if visited[&node.state].cost == node.cost && !node.prior_states.is_empty() {
                visited
                    .get_mut(&node.state)
                    .unwrap()
                    .prior_states
                    .push(node.prior_states.pop().unwrap());
            }
            continue;
        }

        let forward = match node.state.direction {
            Direction::Up => State {
                position: Position {
                    x: node.state.position.x,
                    y: node.state.position.y - 1,
                },
                direction: Direction::Up,
            },
            Direction::Down => State {
                position: Position {
                    x: node.state.position.x,
                    y: node.state.position.y + 1,
                },
                direction: Direction::Down,
            },
            Direction::Left => State {
                position: Position {
                    x: node.state.position.x - 1,
                    y: node.state.position.y,
                },
                direction: Direction::Left,
            },
            Direction::Right => State {
                position: Position {
                    x: node.state.position.x + 1,
                    y: node.state.position.y,
                },
                direction: Direction::Right,
            },
        };
        let clockwise = match node.state.direction {
            Direction::Up => State {
                position: Position {
                    x: node.state.position.x + 1,
                    y: node.state.position.y,
                },
                direction: Direction::Right,
            },
            Direction::Down => State {
                position: Position {
                    x: node.state.position.x - 1,
                    y: node.state.position.y,
                },
                direction: Direction::Left,
            },
            Direction::Left => State {
                position: Position {
                    x: node.state.position.x,
                    y: node.state.position.y - 1,
                },
                direction: Direction::Up,
            },
            Direction::Right => State {
                position: Position {
                    x: node.state.position.x,
                    y: node.state.position.y + 1,
                },
                direction: Direction::Down,
            },
        };
        let counter_clockwise = match node.state.direction {
            Direction::Up => State {
                position: Position {
                    x: node.state.position.x - 1,
                    y: node.state.position.y,
                },
                direction: Direction::Left,
            },
            Direction::Down => State {
                position: Position {
                    x: node.state.position.x + 1,
                    y: node.state.position.y,
                },
                direction: Direction::Right,
            },
            Direction::Left => State {
                position: Position {
                    x: node.state.position.x,
                    y: node.state.position.y + 1,
                },
                direction: Direction::Down,
            },
            Direction::Right => State {
                position: Position {
                    x: node.state.position.x,
                    y: node.state.position.y - 1,
                },
                direction: Direction::Up,
            },
        };
        if map[forward.position.y][forward.position.x] == Space::Empty
            && !visited.contains_key(&forward)
        {
            heap.push(Node2 {
                cost: node.cost - 1,
                state: forward,
                prior_states: vec![node.state.clone()],
            });
        }
        if map[clockwise.position.y][clockwise.position.x] == Space::Empty
            && !visited.contains_key(&clockwise)
        {
            heap.push(Node2 {
                cost: node.cost - 1001,
                state: clockwise,
                prior_states: vec![node.state.clone()],
            });
        }
        if map[counter_clockwise.position.y][counter_clockwise.position.x] == Space::Empty
            && !visited.contains_key(&counter_clockwise)
        {
            heap.push(Node2 {
                cost: node.cost - 1001,
                state: counter_clockwise,
                prior_states: vec![node.state.clone()],
            });
        }
        visited.insert(node.state.clone(), node);
    }

    let mut positions: HashSet<Position> = HashSet::new();
    while !best_states.is_empty() {
        let state = best_states.pop().unwrap();
        for prior_state in &visited.get(&state).unwrap().prior_states {
            best_states.push(prior_state.clone());
        }
        positions.insert(state.position);
    }

    positions.len() as u32
}
