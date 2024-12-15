use aoc_runner_derive::aoc;

enum Space {
    Empty,
    Wall,
    Box,
    Robot,
}

enum Direction {
    Up,
    Down,
    Left,
    Right,
}

struct Vector {
    x: i32,
    y: i32,
}

impl Vector {
    fn add(&self, other: &Vector) -> Vector {
        Vector {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
    fn inc(&mut self, other: &Vector) {
        self.x += other.x;
        self.y += other.y;
    }
}

fn push(map: &mut Vec<Vec<Space>>, robot: &mut Vector, direction: &Direction) {
    let delta = match direction {
        Direction::Up => Vector { x: 0, y: -1 },
        Direction::Down => Vector { x: 0, y: 1 },
        Direction::Left => Vector { x: -1, y: 0 },
        Direction::Right => Vector { x: 1, y: 0 },
    };
    let mut space = robot.add(&delta);
    let mut push_box = false;
    loop {
        match map[space.y as usize][space.x as usize] {
            Space::Empty => {
                break;
            }
            Space::Box => {
                push_box = true;
                space = space.add(&delta);
            }
            Space::Robot => panic!("What is the robot doing here?!"),
            Space::Wall => {
                return;
            }
        }
    }
    if push_box {
        // put a box in the space
        map[space.y as usize][space.x as usize] = Space::Box;
    }
    map[robot.y as usize][robot.x as usize] = Space::Empty;
    robot.inc(&delta);
    map[robot.y as usize][robot.x as usize] = Space::Robot;
}

#[aoc(day15, part1)]
pub fn part1(input: &str) -> usize {
    let mut iter = input.lines().enumerate();
    let mut map: Vec<Vec<Space>> = vec![];
    let mut robot = Vector { x: 0, y: 0 };
    loop {
        let (y, line) = iter.next().unwrap();
        if line.len() == 0 {
            break;
        }
        map.push(
            line.chars()
                .enumerate()
                .map(|(x, c)| match c {
                    '.' => Space::Empty,
                    '#' => Space::Wall,
                    'O' => Space::Box,
                    '@' => {
                        robot = Vector {
                            x: x as i32,
                            y: y as i32,
                        };
                        Space::Robot
                    }
                    _ => panic!("Unexpected character"),
                })
                .collect(),
        );
    }

    iter.for_each(|(_, line)| {
        line.chars().for_each(|c| {
            let direction = match c {
                '^' => Direction::Up,
                'v' => Direction::Down,
                '<' => Direction::Left,
                '>' => Direction::Right,
                _ => panic!("Unexpected character"),
            };
            push(&mut map, &mut robot, &direction);
        })
    });

    let mut checksum = 0;
    for (y, row) in map.into_iter().enumerate() {
        for (x, space) in row.into_iter().enumerate() {
            if let Space::Box = space {
                checksum += 100 * y + x;
            }
        }
    }
    checksum
}

enum Space2 {
    Empty,
    Wall,
    LeftBox,
    RightBox,
    Robot,
}

fn can_push(map: &Vec<Vec<Space2>>, pos: &Vector, direction: &Direction) -> bool {
    match (direction, &map[pos.y as usize][pos.x as usize]) {
        (_, Space2::Wall) => false,
        (_, Space2::Empty) => true,
        (Direction::Up, Space2::Robot) => can_push(
            map,
            &Vector {
                x: pos.x,
                y: pos.y - 1,
            },
            direction,
        ),
        (Direction::Down, Space2::Robot) => can_push(
            map,
            &Vector {
                x: pos.x,
                y: pos.y + 1,
            },
            direction,
        ),
        (Direction::Left, Space2::Robot) => can_push(
            map,
            &Vector {
                x: pos.x - 1,
                y: pos.y,
            },
            direction,
        ),
        (Direction::Right, Space2::Robot) => can_push(
            map,
            &Vector {
                x: pos.x + 1,
                y: pos.y,
            },
            direction,
        ),
        (Direction::Up, Space2::LeftBox) => {
            can_push(
                map,
                &Vector {
                    x: pos.x,
                    y: pos.y - 1,
                },
                direction,
            ) && can_push(
                map,
                &Vector {
                    x: pos.x + 1,
                    y: pos.y - 1,
                },
                direction,
            )
        }
        (Direction::Down, Space2::LeftBox) => {
            can_push(
                map,
                &Vector {
                    x: pos.x,
                    y: pos.y + 1,
                },
                direction,
            ) && can_push(
                map,
                &Vector {
                    x: pos.x + 1,
                    y: pos.y + 1,
                },
                direction,
            )
        }
        (Direction::Left, Space2::LeftBox) => panic!("Unexpected push"),
        (Direction::Right, Space2::LeftBox) => can_push(
            map,
            &Vector {
                x: pos.x + 2,
                y: pos.y,
            },
            direction,
        ),
        (Direction::Up, Space2::RightBox) => {
            can_push(
                map,
                &Vector {
                    x: pos.x,
                    y: pos.y - 1,
                },
                direction,
            ) && can_push(
                map,
                &Vector {
                    x: pos.x - 1,
                    y: pos.y - 1,
                },
                direction,
            )
        }
        (Direction::Down, Space2::RightBox) => {
            can_push(
                map,
                &Vector {
                    x: pos.x,
                    y: pos.y + 1,
                },
                direction,
            ) && can_push(
                map,
                &Vector {
                    x: pos.x - 1,
                    y: pos.y + 1,
                },
                direction,
            )
        }
        (Direction::Left, Space2::RightBox) => can_push(
            map,
            &Vector {
                x: pos.x - 2,
                y: pos.y,
            },
            direction,
        ),
        (Direction::Right, Space2::RightBox) => panic!("Unexpected push"),
    }
}

fn push2(map: &mut Vec<Vec<Space2>>, pos: &mut Vector, direction: &Direction) {
    match (direction, &map[pos.y as usize][pos.x as usize]) {
        (_, Space2::Wall) => panic!("Cannot push a wall!"),
        (_, Space2::Empty) => (),
        (Direction::Up, Space2::Robot) => {
            push2(
                map,
                &mut Vector {
                    x: pos.x,
                    y: pos.y - 1,
                },
                direction,
            );
            map[pos.y as usize][pos.x as usize] = Space2::Empty;
            pos.inc(&Vector { x: 0, y: -1 });
            map[pos.y as usize][pos.x as usize] = Space2::Robot;
        }
        (Direction::Down, Space2::Robot) => {
            push2(
                map,
                &mut Vector {
                    x: pos.x,
                    y: pos.y + 1,
                },
                direction,
            );
            map[pos.y as usize][pos.x as usize] = Space2::Empty;
            pos.inc(&Vector { x: 0, y: 1 });
            map[pos.y as usize][pos.x as usize] = Space2::Robot;
        }
        (Direction::Left, Space2::Robot) => {
            push2(
                map,
                &mut Vector {
                    x: pos.x - 1,
                    y: pos.y,
                },
                direction,
            );
            map[pos.y as usize][pos.x as usize] = Space2::Empty;
            pos.inc(&Vector { x: -1, y: 0 });
            map[pos.y as usize][pos.x as usize] = Space2::Robot;
        }
        (Direction::Right, Space2::Robot) => {
            push2(
                map,
                &mut Vector {
                    x: pos.x + 1,
                    y: pos.y,
                },
                direction,
            );
            map[pos.y as usize][pos.x as usize] = Space2::Empty;
            pos.inc(&Vector { x: 1, y: 0 });
            map[pos.y as usize][pos.x as usize] = Space2::Robot;
        }
        (Direction::Up, Space2::LeftBox) => {
            push2(
                map,
                &mut Vector {
                    x: pos.x,
                    y: pos.y - 1,
                },
                direction,
            );
            push2(
                map,
                &mut Vector {
                    x: pos.x + 1,
                    y: pos.y - 1,
                },
                direction,
            );
            map[pos.y as usize][pos.x as usize] = Space2::Empty;
            map[pos.y as usize][pos.x as usize + 1] = Space2::Empty;
            map[pos.y as usize - 1][pos.x as usize] = Space2::LeftBox;
            map[pos.y as usize - 1][pos.x as usize + 1] = Space2::RightBox;
        }
        (Direction::Down, Space2::LeftBox) => {
            push2(
                map,
                &mut Vector {
                    x: pos.x,
                    y: pos.y + 1,
                },
                direction,
            );
            push2(
                map,
                &mut Vector {
                    x: pos.x + 1,
                    y: pos.y + 1,
                },
                direction,
            );
            map[pos.y as usize][pos.x as usize] = Space2::Empty;
            map[pos.y as usize][pos.x as usize + 1] = Space2::Empty;
            map[pos.y as usize + 1][pos.x as usize] = Space2::LeftBox;
            map[pos.y as usize + 1][pos.x as usize + 1] = Space2::RightBox;
        }
        (Direction::Left, Space2::LeftBox) => panic!("Unexpected push"),
        (Direction::Right, Space2::LeftBox) => {
            push2(
                map,
                &mut Vector {
                    x: pos.x + 2,
                    y: pos.y,
                },
                direction,
            );
            map[pos.y as usize][pos.x as usize] = Space2::Empty;
            map[pos.y as usize][pos.x as usize + 1] = Space2::LeftBox;
            map[pos.y as usize][pos.x as usize + 2] = Space2::RightBox;
        }
        (Direction::Up, Space2::RightBox) => {
            push2(
                map,
                &mut Vector {
                    x: pos.x,
                    y: pos.y - 1,
                },
                direction,
            );
            push2(
                map,
                &mut Vector {
                    x: pos.x - 1,
                    y: pos.y - 1,
                },
                direction,
            );
            map[pos.y as usize][pos.x as usize] = Space2::Empty;
            map[pos.y as usize][pos.x as usize - 1] = Space2::Empty;
            map[pos.y as usize - 1][pos.x as usize] = Space2::RightBox;
            map[pos.y as usize - 1][pos.x as usize - 1] = Space2::LeftBox;
        }
        (Direction::Down, Space2::RightBox) => {
            push2(
                map,
                &mut Vector {
                    x: pos.x,
                    y: pos.y + 1,
                },
                direction,
            );
            push2(
                map,
                &mut Vector {
                    x: pos.x - 1,
                    y: pos.y + 1,
                },
                direction,
            );
            map[pos.y as usize][pos.x as usize] = Space2::Empty;
            map[pos.y as usize][pos.x as usize - 1] = Space2::Empty;
            map[pos.y as usize + 1][pos.x as usize] = Space2::RightBox;
            map[pos.y as usize + 1][pos.x as usize - 1] = Space2::LeftBox;
        }
        (Direction::Left, Space2::RightBox) => {
            push2(
                map,
                &mut Vector {
                    x: pos.x - 2,
                    y: pos.y,
                },
                direction,
            );
            map[pos.y as usize][pos.x as usize] = Space2::Empty;
            map[pos.y as usize][pos.x as usize - 1] = Space2::RightBox;
            map[pos.y as usize][pos.x as usize - 2] = Space2::LeftBox;
        }
        (Direction::Right, Space2::RightBox) => panic!("Unexpected push"),
    }
}

#[aoc(day15, part2)]
pub fn part2(input: &str) -> usize {
    let mut iter = input.lines().enumerate();
    let mut map: Vec<Vec<Space2>> = vec![];
    let mut robot = Vector { x: 0, y: 0 };
    loop {
        let (y, line) = iter.next().unwrap();
        if line.len() == 0 {
            break;
        }
        let mut row = vec![];
        row.reserve(line.len() * 2);
        line.chars().enumerate().for_each(|(x, c)| match c {
            '.' => {
                row.push(Space2::Empty);
                row.push(Space2::Empty);
            }
            '#' => {
                row.push(Space2::Wall);
                row.push(Space2::Wall);
            }
            'O' => {
                row.push(Space2::LeftBox);
                row.push(Space2::RightBox);
            }
            '@' => {
                robot = Vector {
                    x: 2 * x as i32,
                    y: y as i32,
                };
                row.push(Space2::Robot);
                row.push(Space2::Empty);
            }
            _ => panic!("Unexpected character"),
        });
        map.push(row);
    }

    iter.for_each(|(_, line)| {
        line.chars().for_each(|c| {
            let direction = match c {
                '^' => Direction::Up,
                'v' => Direction::Down,
                '<' => Direction::Left,
                '>' => Direction::Right,
                _ => panic!("Unexpected character"),
            };
            if can_push(&map, &robot, &direction) {
                push2(&mut map, &mut robot, &direction);
            }
        })
    });

    let mut checksum = 0;
    for (y, row) in map.into_iter().enumerate() {
        for (x, space) in row.into_iter().enumerate() {
            if let Space2::LeftBox = space {
                checksum += 100 * y + x;
            }
        }
    }
    checksum
}
