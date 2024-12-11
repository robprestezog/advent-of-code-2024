use aoc_runner_derive::aoc;

enum S1 {
    Start,
    M,
    U,
    L,
    Open,
    A1(u32),
    A2(u32),
    A3(u32),
    Comma(u32),
    B1(u32, u32),
    B2(u32, u32),
    B3(u32, u32),
}

#[aoc(day3, part1)]
fn part1(input: &str) -> u32 {
    let mut total = 0;
    input.chars().fold(S1::Start, |state, x| match (state, x) {
        (S1::Start, 'm') => S1::M,
        (S1::M, 'u') => S1::U,
        (S1::U, 'l') => S1::L,
        (S1::L, '(') => S1::Open,
        (S1::Open, '1'..='9') => S1::A1(x as u32 - '0' as u32),
        (S1::A1(a), '0'..='9') => S1::A2(x as u32 - '0' as u32 + a * 10),
        (S1::A1(a), ',') => S1::Comma(a),
        (S1::A2(a), '0'..='9') => S1::A3(x as u32 - '0' as u32 + a * 10),
        (S1::A2(a), ',') => S1::Comma(a),
        (S1::A3(a), ',') => S1::Comma(a),
        (S1::Comma(a), '1'..='9') => S1::B1(a, x as u32 - '0' as u32),
        (S1::B1(a, b), '0'..='9') => S1::B2(a, x as u32 - '0' as u32 + b * 10),
        (S1::B1(a, b), ')') => {
            total += a * b;
            S1::Start
        }
        (S1::B2(a, b), '0'..='9') => S1::B3(a, x as u32 - '0' as u32 + b * 10),
        (S1::B2(a, b), ')') => {
            total += a * b;
            S1::Start
        }
        (S1::B3(a, b), ')') => {
            total += a * b;
            S1::Start
        }
        (_, 'm') => S1::M,
        (_, _) => S1::Start,
    });
    total
}

enum S2 {
    // On states
    Start,
    M,
    U,
    L,
    Open,
    A1(u32),
    A2(u32),
    A3(u32),
    Comma(u32),
    B1(u32, u32),
    B2(u32, u32),
    B3(u32, u32),
    D,
    O,
    N,
    Quote,
    T,
    DontOpen,
    // Off states
    Off,
    OffD,
    OffO,
    OffOpen,
}

#[aoc(day3, part2)]
fn part2(input: &str) -> u32 {
    let mut total = 0;
    input.chars().fold(S2::Start, |state, x| match (state, x) {
        (S2::Start, 'm') => S2::M,
        (S2::M, 'u') => S2::U,
        (S2::U, 'l') => S2::L,
        (S2::L, '(') => S2::Open,
        (S2::Open, '1'..='9') => S2::A1(x as u32 - '0' as u32),
        (S2::A1(a), '0'..='9') => S2::A2(x as u32 - '0' as u32 + a * 10),
        (S2::A1(a), ',') => S2::Comma(a),
        (S2::A2(a), '0'..='9') => S2::A3(x as u32 - '0' as u32 + a * 10),
        (S2::A2(a), ',') => S2::Comma(a),
        (S2::A3(a), ',') => S2::Comma(a),
        (S2::Comma(a), '1'..='9') => S2::B1(a, x as u32 - '0' as u32),
        (S2::B1(a, b), '0'..='9') => S2::B2(a, x as u32 - '0' as u32 + b * 10),
        (S2::B1(a, b), ')') => {
            total += a * b;
            S2::Start
        }
        (S2::B2(a, b), '0'..='9') => S2::B3(a, x as u32 - '0' as u32 + b * 10),
        (S2::B2(a, b), ')') => {
            total += a * b;
            S2::Start
        }
        (S2::B3(a, b), ')') => {
            total += a * b;
            S2::Start
        }
        (S2::D, 'o') => S2::O,
        (S2::O, 'n') => S2::N,
        (S2::N, '\'') => S2::Quote,
        (S2::Quote, 't') => S2::T,
        (S2::T, '(') => S2::DontOpen,
        (S2::DontOpen, ')') => S2::Off,
        (S2::Off, 'd') => S2::OffD,
        (S2::Off, _) => S2::Off,
        (S2::OffD, 'o') => S2::OffO,
        (S2::OffD, 'd') => S2::OffD,
        (S2::OffD, _) => S2::Off,
        (S2::OffO, '(') => S2::OffOpen,
        (S2::OffO, 'd') => S2::OffD,
        (S2::OffO, _) => S2::Off,
        (S2::OffOpen, ')') => S2::Start,
        (S2::OffOpen, 'd') => S2::OffD,
        (S2::OffOpen, _) => S2::Off,
        (_, 'm') => S2::M,
        (_, 'd') => S2::D,
        (_, _) => S2::Start,
    });
    total
}
