use std::collections::HashSet;

use aoc_runner_derive::aoc;

#[derive(PartialEq, Eq, Hash)]
struct Node {
    x: usize,
    y: usize,
}

struct Graph {
    bytes: Vec<Vec<u8>>,
    width: usize,
    height: usize,
}

impl Graph {
    pub fn new(input: &str) -> Self {
        // let mut bytes = vec![];
        let mut width = 0;
        let mut height = 0;
        let bytes = input
            .lines()
            .map(|line| {
                width = line.len();
                height += 1;

                line.as_bytes().iter().map(|b| b - '0' as u8).collect()
            })
            .collect();

        Self {
            bytes,
            width,
            height,
        }
    }

    pub fn get_trailheads(&self) -> Vec<Node> {
        let mut trailheads = vec![];
        for y in 0..self.height {
            for x in 0..self.width {
                if self.bytes[y][x] == 0 {
                    trailheads.push(Node { x, y });
                }
            }
        }
        trailheads
    }

    pub fn get_neighbors(&self, nodes: HashSet<Node>) -> HashSet<Node> {
        let mut neighbors = HashSet::new();
        for node in nodes {
            let value = self.bytes[node.y][node.x] + 1;
            if node.y > 0 && self.bytes[node.y - 1][node.x] == value {
                neighbors.insert(Node {
                    x: node.x,
                    y: node.y - 1,
                });
            }
            if node.x > 0 && self.bytes[node.y][node.x - 1] == value {
                neighbors.insert(Node {
                    x: node.x - 1,
                    y: node.y,
                });
            }
            if node.y + 1 < self.height && self.bytes[node.y + 1][node.x] == value {
                neighbors.insert(Node {
                    x: node.x,
                    y: node.y + 1,
                });
            }
            if node.x + 1 < self.width && self.bytes[node.y][node.x + 1] == value {
                neighbors.insert(Node {
                    x: node.x + 1,
                    y: node.y,
                });
            }
        }
        neighbors
    }
}

#[aoc(day10, part1)]
fn part1(input: &str) -> usize {
    let graph = Graph::new(input);
    let trailheads = graph.get_trailheads();

    let mut score = 0;
    for trailhead in trailheads {
        let mut nodes = HashSet::from([trailhead]);
        for _ in 0..9 {
            nodes = graph.get_neighbors(nodes);
        }
        score += nodes.len();
    }

    score
}

struct Node2 {
    level: u8,
    ways: u16,
}

struct Graph2 {
    nodes: Vec<Vec<Node2>>,
    width: usize,
    height: usize,
}

impl Graph2 {
    pub fn new(input: &str) -> Self {
        // let mut bytes = vec![];
        let mut width = 0;
        let mut height = 0;
        let nodes = input
            .lines()
            .map(|line| {
                width = line.len();
                height += 1;

                line.as_bytes()
                    .iter()
                    .map(|b| Node2 {
                        level: b - '0' as u8,
                        ways: 0,
                    })
                    .collect()
            })
            .collect();

        Self {
            nodes,
            width,
            height,
        }
    }
    pub fn get_trailheads(&mut self) -> HashSet<Node> {
        let mut trailheads = HashSet::new();
        for y in 0..self.height {
            for x in 0..self.width {
                if self.nodes[y][x].level == 0 {
                    trailheads.insert(Node { x, y });
                    self.nodes[y][x].ways = 1;
                }
            }
        }
        trailheads
    }
    pub fn get_neighbors(&mut self, nodes: HashSet<Node>) -> HashSet<Node> {
        let mut neighbors = HashSet::new();
        for node in nodes {
            let value = self.nodes[node.y][node.x].level + 1;
            if node.y > 0 && self.nodes[node.y - 1][node.x].level == value {
                neighbors.insert(Node {
                    x: node.x,
                    y: node.y - 1,
                });
                self.nodes[node.y - 1][node.x].ways += self.nodes[node.y][node.x].ways;
            }
            if node.x > 0 && self.nodes[node.y][node.x - 1].level == value {
                neighbors.insert(Node {
                    x: node.x - 1,
                    y: node.y,
                });
                self.nodes[node.y][node.x - 1].ways += self.nodes[node.y][node.x].ways;
            }
            if node.y + 1 < self.height && self.nodes[node.y + 1][node.x].level == value {
                neighbors.insert(Node {
                    x: node.x,
                    y: node.y + 1,
                });
                self.nodes[node.y + 1][node.x].ways += self.nodes[node.y][node.x].ways;
            }
            if node.x + 1 < self.width && self.nodes[node.y][node.x + 1].level == value {
                neighbors.insert(Node {
                    x: node.x + 1,
                    y: node.y,
                });
                self.nodes[node.y][node.x + 1].ways += self.nodes[node.y][node.x].ways;
            }
        }
        neighbors
    }
    pub fn score(&self, nodes: HashSet<Node>) -> u32 {
        let mut score = 0;
        for node in nodes {
            score += self.nodes[node.y][node.x].ways as u32;
        }
        score
    }
}

#[aoc(day10, part2)]
fn part2(input: &str) -> u32 {
    let mut graph = Graph2::new(input);
    let mut nodes = graph.get_trailheads();
    for _ in 0..9 {
        nodes = graph.get_neighbors(nodes);
    }
    graph.score(nodes)
}
