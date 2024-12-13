use aoc_runner_derive::aoc;

#[aoc(day13, part1)]
pub fn part1(input: &str) -> i32 {
    let mut iter = input.lines();
    let mut total = 0;
    loop {
        // Button A: X+33, Y+93
        let pair_a = iter
            .next()
            .unwrap()
            .strip_prefix("Button A: X+")
            .unwrap()
            .split_once(", Y+")
            .unwrap();
        let (ax, ay): (i32, i32) = (pair_a.0.parse().unwrap(), pair_a.1.parse().unwrap());
        // Button B: X+98, Y+36
        let pair_b = iter
            .next()
            .unwrap()
            .strip_prefix("Button B: X+")
            .unwrap()
            .split_once(", Y+")
            .unwrap();
        let (bx, by): (i32, i32) = (pair_b.0.parse().unwrap(), pair_b.1.parse().unwrap());
        // Prize: X=6697, Y=10467
        let pair_p = iter
            .next()
            .unwrap()
            .strip_prefix("Prize: X=")
            .unwrap()
            .split_once(", Y=")
            .unwrap();
        let (px, py): (i32, i32) = (pair_p.0.parse().unwrap(), pair_p.1.parse().unwrap());

        // A = (px*by - py*bx)/(ax*by - ay*bx)
        let num = px * by - py * bx;
        let den = ax * by - ay * bx;
        if den == 0 {
            // The buttons are colinear
            if num == 0 {
                // So are the points!
                panic!("Implement this case!")
            }
        } else if (num > 0 && den > 0 && num % den == 0)
            || (num < 0 && den < 0 && (-num) % (-den) == 0)
        {
            let a = num / den;
            if (px - a * ax) % bx == 0 {
                let b = (px - a * ax) / bx;
                if b >= 0 {
                    total += 3 * a + b;
                }
            }
        }

        if let None = iter.next() {
            break;
        }
    }
    total
}

#[aoc(day13, part2)]
pub fn part2(input: &str) -> i64 {
    let mut iter = input.lines();
    let mut total = 0;
    loop {
        // Button A: X+33, Y+93
        let pair_a = iter
            .next()
            .unwrap()
            .strip_prefix("Button A: X+")
            .unwrap()
            .split_once(", Y+")
            .unwrap();
        let (ax, ay): (i64, i64) = (pair_a.0.parse().unwrap(), pair_a.1.parse().unwrap());
        // Button B: X+98, Y+36
        let pair_b = iter
            .next()
            .unwrap()
            .strip_prefix("Button B: X+")
            .unwrap()
            .split_once(", Y+")
            .unwrap();
        let (bx, by): (i64, i64) = (pair_b.0.parse().unwrap(), pair_b.1.parse().unwrap());
        // Prize: X=6697, Y=10467
        let pair_p = iter
            .next()
            .unwrap()
            .strip_prefix("Prize: X=")
            .unwrap()
            .split_once(", Y=")
            .unwrap();
        let (px, py): (i64, i64) = (
            pair_p.0.parse::<i64>().unwrap() + 10000000000000,
            pair_p.1.parse::<i64>().unwrap() + 10000000000000,
        );

        // A = (px*by - py*bx)/(ax*by - ay*bx)
        let num = px * by - py * bx;
        let den = ax * by - ay * bx;
        if den == 0 {
            // The buttons are colinear
            if num == 0 {
                // So are the points!
                panic!("Implement this case!")
            }
        } else if (num > 0 && den > 0 && num % den == 0)
            || (num < 0 && den < 0 && (-num) % (-den) == 0)
        {
            let a = num / den;
            if (px - a * ax) % bx == 0 {
                let b = (px - a * ax) / bx;
                if b >= 0 {
                    total += 3 * a + b;
                }
            }
        }

        if let None = iter.next() {
            break;
        }
    }
    total
}
