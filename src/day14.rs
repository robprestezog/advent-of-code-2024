use aoc_runner_derive::aoc;

const WIDTH: i32 = 101;
const HEIGHT: i32 = 103;

// These were eyeballed from the periodically dense ranges in the first few hundred seconds.
const X_LOW: i32 = 37;
const X_HIGH: i32 = 75;
const Y_LOW: i32 = 19;
const Y_HIGH: i32 = 53;

struct Robot {
    px: i32,
    py: i32,
    vx: i32,
    vy: i32,
}

impl Robot {
    fn update(&mut self) {
        self.px += self.vx;
        self.py += self.vy;
        if self.px < 0 {
            self.px += WIDTH;
        } else if self.px >= WIDTH {
            self.px -= WIDTH;
        }
        if self.py < 0 {
            self.py += HEIGHT;
        } else if self.py >= HEIGHT {
            self.py -= HEIGHT;
        }
    }
}

fn parse(line: &str) -> Robot {
    // Example line:
    // p=10,3 v=-1,2
    let (p, v) = line.strip_prefix("p=").unwrap().split_once(" v=").unwrap();
    let (px, py) = p.split_once(',').unwrap();
    let (vx, vy) = v.split_once(',').unwrap();
    let px = px.parse().unwrap();
    let py = py.parse().unwrap();
    let vx = vx.parse().unwrap();
    let vy = vy.parse().unwrap();
    Robot { px, py, vx, vy }
}

#[aoc(day14, part1)]
pub fn part1(input: &str) -> u32 {
    let mut q1 = 0;
    let mut q2 = 0;
    let mut q3 = 0;
    let mut q4 = 0;
    for line in input.lines() {
        let robot = parse(line);
        let mut x = robot.px + 100 * robot.vx;
        let mut y = robot.py + 100 * robot.vy;
        if x >= WIDTH {
            x = x % WIDTH;
        } else if x < 0 {
            x = WIDTH - 1 - ((-x - 1) % WIDTH);
        }
        if y >= HEIGHT {
            y = y % HEIGHT;
        } else if y < 0 {
            y = HEIGHT - 1 - ((-y - 1) % HEIGHT);
        }
        if y < (HEIGHT - 1) / 2 {
            if x < (WIDTH - 1) / 2 {
                q1 += 1;
            } else if x >= (WIDTH + 1) / 2 {
                q2 += 1;
            }
        } else if y >= (HEIGHT + 1) / 2 {
            if x < (WIDTH - 1) / 2 {
                q3 += 1;
            } else if x >= (WIDTH + 1) / 2 {
                q4 += 1;
            }
        }
    }
    q1 * q2 * q3 * q4
}

#[aoc(day14, part2)]
pub fn part2(input: &str) -> i32 {
    let mut robots: Vec<_> = input.lines().map(parse).collect();
    let period = WIDTH * HEIGHT;
    let mut best_score = 0;
    let mut best_time = 0;
    for time in 0..period {
        let mut score = 0;
        // Uncomment for tree printing
        // let mut lines = vec![];
        // for _ in 0..HEIGHT {
        //     lines.push(vec!['.' as u8; WIDTH as usize]);
        // }
        for robot in &mut robots {
            if robot.px >= X_LOW && robot.px <= X_HIGH && robot.py >= Y_LOW && robot.py <= Y_HIGH {
                score += 1
            }
            // lines[robot.py as usize][robot.px as usize] = '*' as u8;
            robot.update();
        }
        if score > best_score {
            // for line in lines {
            //     println!("{}", String::from_utf8(line).unwrap());
            // }
            // println!("{}: {}", time, score);
            best_score = score;
            best_time = time;
        }
    }
    best_time
}
