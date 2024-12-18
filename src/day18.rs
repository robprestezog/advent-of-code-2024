use aoc_runner_derive::aoc;

const SIZE: usize = 71;
const SIZE_PLUS_ONE: usize = SIZE + 1;

#[derive(PartialEq)]
enum Space {
    Empty,
    Visited,
    Full,
}

struct Position {
    x: usize,
    y: usize,
}

#[aoc(day18, part1)]
pub fn part1(input: &str) -> u32 {
    let mut grid: Vec<Vec<Space>> = (0..SIZE)
        .map(|_| (0..SIZE).map(|_| Space::Empty).collect())
        .collect();

    for (i, line) in input.lines().enumerate() {
        if i >= 1024 {
            break;
        }
        let (x, y) = line.split_once(',').unwrap();
        let (x, y): (usize, usize) = (x.parse().unwrap(), y.parse().unwrap());
        grid[y][x] = Space::Full;
    }
    let mut steps = 0;

    let mut positions = vec![Position { x: 0, y: 0 }];
    grid[0][0] = Space::Visited;
    while grid[SIZE - 1][SIZE - 1] != Space::Visited {
        let mut new_positions = vec![];

        for position in positions {
            if position.y > 0 && grid[position.y - 1][position.x] == Space::Empty {
                new_positions.push(Position {
                    x: position.x,
                    y: position.y - 1,
                });
                grid[position.y - 1][position.x] = Space::Visited;
            }
            if position.y + 1 < SIZE && grid[position.y + 1][position.x] == Space::Empty {
                new_positions.push(Position {
                    x: position.x,
                    y: position.y + 1,
                });
                grid[position.y + 1][position.x] = Space::Visited;
            }
            if position.x > 0 && grid[position.y][position.x - 1] == Space::Empty {
                new_positions.push(Position {
                    x: position.x - 1,
                    y: position.y,
                });
                grid[position.y][position.x - 1] = Space::Visited;
            }
            if position.x + 1 < SIZE && grid[position.y][position.x + 1] == Space::Empty {
                new_positions.push(Position {
                    x: position.x + 1,
                    y: position.y,
                });
                grid[position.y][position.x + 1] = Space::Visited;
            }
        }
        steps += 1;
        positions = new_positions;
    }

    steps
}

struct Region {
    root: usize,
}
struct DisjointRegions {
    regions: Vec<Region>,
}

impl DisjointRegions {
    pub fn new() -> Self {
        Self { regions: vec![] }
    }
    fn get_root(&mut self, id: usize) -> usize {
        let cur_root = self.regions[id].root;
        if cur_root == id {
            return cur_root;
        }
        let root = self.get_root(cur_root);
        if root != cur_root {
            self.regions[id].root = root;
        }
        root
    }
    pub fn new_region(&mut self) -> usize {
        let id = self.regions.len();
        self.regions.push(Region { root: id });
        id
    }
    pub fn merge(&mut self, id1: usize, id2: usize) -> usize {
        let root1 = self.get_root(id1);
        let root2 = self.get_root(id2);
        if root1 == root2 {
            root1
        } else if root1 < root2 {
            self.regions[root2].root = root1;
            root1
        } else {
            self.regions[root1].root = root2;
            root2
        }
    }
    pub fn is_root(&self, id: usize) -> bool {
        self.regions[id].root == id
    }
}

#[aoc(day18, part2)]
pub fn part2(input: &str) -> String {
    let mut regions = DisjointRegions::new();
    let empty = regions.new_region();
    let top_right = regions.new_region();
    let bottom_left = regions.new_region();
    let mut grid: Vec<Vec<usize>> = (0..SIZE + 2)
        .map(|y| {
            (0..SIZE + 2)
                .map(|x| match (x, y) {
                    (0, _) => top_right,
                    (_, 0) => bottom_left,
                    (SIZE_PLUS_ONE, _) => bottom_left,
                    (_, SIZE_PLUS_ONE) => top_right,
                    (_, _) => empty,
                })
                .collect()
        })
        .collect();

    for line in input.lines() {
        let (x, y) = line.split_once(',').unwrap();
        let (x, y): (usize, usize) = (
            x.parse::<usize>().unwrap() + 1,
            y.parse::<usize>().unwrap() + 1,
        );
        let mut region = empty;
        for ay in (y - 1)..=(y + 1) {
            for ax in (x - 1)..=(x + 1) {
                if grid[ay][ax] == empty {
                    continue;
                }
                if region == empty {
                    region = grid[ay][ax];
                } else if region != grid[ay][ax] {
                    region = regions.merge(region, grid[ay][ax]);
                    // check if bottom_left has been merged with top_right
                    if !regions.is_root(bottom_left) {
                        return line.to_string();
                    }
                }
            }
        }
        if region == empty {
            region = regions.new_region();
        }
        grid[y][x] = region;
    }
    "None".to_string()
}
