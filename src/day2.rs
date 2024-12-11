use aoc_runner_derive::aoc;

enum State {
    Start,
    First(u8),
    Increasing(u8),
    Decreasing(u8),
    Unsafe,
}

#[aoc(day2, part1)]
fn part1(input: &str) -> u32 {
    input
        .lines()
        .map(|line| {
            match line
                .split_ascii_whitespace()
                .map(|d| d.parse().unwrap())
                .fold(State::Start, |state, num| match state {
                    State::Start => State::First(num),
                    State::First(prev) if num > prev && num <= prev + 3 => State::Increasing(num),
                    State::First(prev) if num < prev && num + 3 >= prev => State::Decreasing(num),
                    State::First(_) => State::Unsafe,
                    State::Increasing(prev) if num > prev && num <= prev + 3 => {
                        State::Increasing(num)
                    }
                    State::Increasing(_) => State::Unsafe,
                    State::Decreasing(prev) if num < prev && num + 3 >= prev => {
                        State::Decreasing(num)
                    }
                    State::Decreasing(_) => State::Unsafe,
                    State::Unsafe => State::Unsafe,
                }) {
                State::Unsafe => 0,
                _ => 1,
            }
        })
        .fold(0, |acc, x| acc + x)
}

enum S2 {
    Start,
    First(u8),
    SecondIncreasing(u8, u8),
    SecondDecreasing(u8, u8),
    SecondBadGap(u8, u8), // Values in low, high order
    UpOrAmbiguous(u8, u8),
    DownOrAmbiguous(u8, u8),
    UpOrDown(u8, u8),
    Increasing(u8, u8),
    Decreasing(u8, u8),
    UpOrUp(u8, u8),     // Values in low, high order
    DownOrDown(u8, u8), // Values in low, high order
    AmbiguousWarn(u8),
    IncreasingWarn(u8),
    DecreasingWarn(u8),
    Unsafe,
}

#[aoc(day2, part2)]
fn part2(input: &str) -> u32 {
    input
        .lines()
        .map(|line| is_safe(line))
        .fold(0, |acc, x| if x { acc + 1 } else { acc })
}

fn is_safe(line: &str) -> bool {
    match line
        .split_ascii_whitespace()
        .map(|d| d.parse().unwrap())
        .fold(S2::Start, |state, c| match state {
            S2::Start => S2::First(c),
            S2::First(a) => {
                if c > a + 3 {
                    S2::SecondBadGap(a, c)
                } else if c > a {
                    S2::SecondIncreasing(a, c)
                } else if c == a {
                    S2::AmbiguousWarn(c)
                } else if c + 3 >= a {
                    S2::SecondDecreasing(a, c)
                } else {
                    S2::SecondBadGap(c, a)
                }
            }
            S2::SecondIncreasing(a, b) => {
                if c > b + 3 {
                    S2::IncreasingWarn(b)
                } else if c > b {
                    S2::Increasing(b, c)
                } else if c == b {
                    S2::IncreasingWarn(b)
                } else if c > a {
                    S2::UpOrAmbiguous(b, c)
                } else if c + 3 >= a {
                    S2::UpOrDown(b, c)
                } else {
                    S2::IncreasingWarn(b)
                }
            }
            S2::SecondDecreasing(a, b) => {
                if c > a + 3 {
                    S2::DecreasingWarn(b)
                } else if c >= a {
                    S2::UpOrDown(c, b)
                } else if c > b {
                    S2::DownOrAmbiguous(b, c)
                } else if c == b {
                    S2::DecreasingWarn(b)
                } else if c + 3 >= b {
                    S2::Decreasing(b, c)
                } else {
                    S2::DecreasingWarn(b)
                }
            }
            S2::SecondBadGap(low, high) => {
                if c > high + 3 {
                    S2::Unsafe
                } else if c > high {
                    S2::IncreasingWarn(c)
                } else if c == high {
                    S2::Unsafe
                } else if c + 3 < low {
                    S2::Unsafe
                } else if c < low {
                    S2::DecreasingWarn(c)
                } else if c == low {
                    S2::Unsafe
                } else if c + 3 >= high && c <= low + 3 {
                    S2::AmbiguousWarn(c)
                } else if c + 3 >= high {
                    S2::DecreasingWarn(c)
                } else if c <= low + 3 {
                    S2::IncreasingWarn(c)
                } else {
                    S2::Unsafe
                }
            }
            S2::UpOrAmbiguous(a, b) => {
                if c > a + 3 {
                    S2::Unsafe
                } else if c > b {
                    S2::IncreasingWarn(c)
                } else if c == b {
                    S2::Unsafe
                } else if c + 3 >= b {
                    S2::DecreasingWarn(c)
                } else {
                    S2::Unsafe
                }
            }
            S2::DownOrAmbiguous(a, b) => {
                if c > b + 3 {
                    S2::Unsafe
                } else if c > b {
                    S2::IncreasingWarn(c)
                } else if c == b {
                    S2::Unsafe
                } else if c + 3 >= a {
                    S2::DecreasingWarn(c)
                } else {
                    S2::Unsafe
                }
            }
            S2::UpOrDown(a, b) => {
                if c > a + 3 {
                    S2::Unsafe
                } else if c > a {
                    S2::IncreasingWarn(c)
                } else if c >= b {
                    S2::Unsafe
                } else if c + 3 >= b {
                    S2::DecreasingWarn(c)
                } else {
                    S2::Unsafe
                }
            }
            S2::Increasing(a, b) => {
                if c > b + 3 {
                    S2::IncreasingWarn(b)
                } else if c > b {
                    S2::Increasing(b, c)
                } else if c == b {
                    S2::IncreasingWarn(b)
                } else if c > a {
                    S2::UpOrUp(c, b)
                } else {
                    S2::IncreasingWarn(b)
                }
            }
            S2::Decreasing(a, b) => {
                if c >= a {
                    S2::DecreasingWarn(b)
                } else if c > b {
                    S2::DownOrDown(b, c)
                } else if c == b {
                    S2::DecreasingWarn(b)
                } else if c + 3 >= b {
                    S2::Decreasing(b, c)
                } else {
                    S2::DecreasingWarn(b)
                }
            }
            S2::UpOrUp(low, high) => {
                if c > high + 3 {
                    S2::Unsafe
                } else if c > low {
                    S2::IncreasingWarn(c)
                } else {
                    S2::Unsafe
                }
            }
            S2::DownOrDown(low, high) => {
                if c >= high {
                    S2::Unsafe
                } else if c + 3 >= low {
                    S2::DecreasingWarn(c)
                } else {
                    S2::Unsafe
                }
            }
            S2::AmbiguousWarn(b) => {
                if c > b + 3 {
                    S2::Unsafe
                } else if c > b {
                    S2::IncreasingWarn(c)
                } else if c == b {
                    S2::Unsafe
                } else if c + 3 >= b {
                    S2::DecreasingWarn(c)
                } else {
                    S2::Unsafe
                }
            }
            S2::IncreasingWarn(b) => {
                if c > b + 3 {
                    S2::Unsafe
                } else if c > b {
                    S2::IncreasingWarn(c)
                } else {
                    S2::Unsafe
                }
            }
            S2::DecreasingWarn(b) => {
                if c >= b {
                    S2::Unsafe
                } else if c + 3 >= b {
                    S2::DecreasingWarn(c)
                } else {
                    S2::Unsafe
                }
            }
            S2::Unsafe => S2::Unsafe,
        }) {
        S2::Unsafe => false,
        _ => true,
    }
}

// fn is_safe_slow(line: &str) -> bool {
//     let sequence: Vec<i32> = line
//         .split_ascii_whitespace()
//         .map(|d| d.parse().unwrap())
//         .collect();
//     return is_safe_increasing(&sequence) || is_safe_decreasing(&sequence);
// }

// fn is_safe_increasing(sequence: &[i32]) -> bool {
//     let mut i = 0;
//     while i + 1 < sequence.len() {
//         if sequence[i + 1] <= sequence[i] || sequence[i + 1] > sequence[i] + 3 {
//             return is_safe_increasing_with_skip(sequence, i)
//                 || is_safe_increasing_with_skip(sequence, i + 1);
//         }
//         i += 1;
//     }
//     return true;
// }
// fn is_safe_increasing_with_skip(sequence: &[i32], skip: usize) -> bool {
//     let mut prev = 0;
//     let mut next = 1;

//     while next < sequence.len() {
//         if prev == skip {
//             prev += 1;
//             next = prev + 1;
//         } else if next == skip {
//             next += 1;
//         } else {
//             if sequence[next] <= sequence[prev] || sequence[next] > sequence[prev] + 3 {
//                 return false;
//             }
//             prev = next;
//             next += 1;
//         }
//     }
//     return true;
// }
// fn is_safe_decreasing(sequence: &[i32]) -> bool {
//     let mut i = 0;
//     while i + 1 < sequence.len() {
//         if sequence[i + 1] >= sequence[i] || sequence[i + 1] < sequence[i] - 3 {
//             return is_safe_decreasing_with_skip(sequence, i)
//                 || is_safe_decreasing_with_skip(sequence, i + 1);
//         }
//         i += 1;
//     }
//     return true;
// }
// fn is_safe_decreasing_with_skip(sequence: &[i32], skip: usize) -> bool {
//     let mut prev = 0;
//     let mut next = 1;

//     while next < sequence.len() {
//         if prev == skip {
//             prev += 1;
//             next = prev + 1;
//         } else if next == skip {
//             next += 1;
//         } else {
//             if sequence[next] >= sequence[prev] || sequence[next] < sequence[prev] - 3 {
//                 return false;
//             }
//             prev = next;
//             next += 1;
//         }
//     }
//     return true;
// }
