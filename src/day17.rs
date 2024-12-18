use aoc_runner_derive::aoc;
use std::mem;

struct Machine {
    register_a: u64,
    register_b: u64,
    register_c: u64,

    program: Vec<u8>,
    program_counter: usize,

    output: Vec<u8>,
}

fn parse(input: &str) -> Machine {
    let mut iter = input.lines();
    let register_a = iter
        .next()
        .unwrap()
        .strip_prefix("Register A: ")
        .unwrap()
        .parse()
        .unwrap();
    let register_b = iter
        .next()
        .unwrap()
        .strip_prefix("Register B: ")
        .unwrap()
        .parse()
        .unwrap();
    let register_c = iter
        .next()
        .unwrap()
        .strip_prefix("Register C: ")
        .unwrap()
        .parse()
        .unwrap();
    iter.next();
    let program = iter
        .next()
        .unwrap()
        .strip_prefix("Program: ")
        .unwrap()
        .split(',')
        .map(|d| d.parse().unwrap())
        .collect();

    Machine {
        register_a,
        register_b,
        register_c,
        program,
        program_counter: 0,
        output: vec![],
    }
}

impl Machine {
    fn run(&mut self) {
        let register_a = &mut self.register_a;
        let register_b = &mut self.register_b;
        let register_c = &mut self.register_c;
        loop {
            let mut iter = self.program.iter().skip(self.program_counter);
            loop {
                match (iter.next(), iter.next()) {
                    (Some(0), Some(0)) => (),
                    (Some(0), Some(1)) => {
                        *register_a >>= 1;
                    }
                    (Some(0), Some(2)) => {
                        *register_a >>= 2;
                    }
                    (Some(0), Some(3)) => {
                        *register_a >>= 3;
                    }
                    (Some(0), Some(4)) => {
                        *register_a >>= *register_a;
                    }
                    (Some(0), Some(5)) => {
                        *register_a >>= *register_b;
                    }
                    (Some(0), Some(6)) => {
                        *register_a >>= *register_c;
                    }
                    (Some(1), Some(operand @ 0..=7)) => {
                        *register_b ^= *operand as u64;
                    }
                    (Some(2), Some(operand @ 0..=3)) => {
                        *register_b = *operand as u64;
                    }
                    (Some(2), Some(4)) => {
                        *register_b = *register_a & 7;
                    }
                    (Some(2), Some(5)) => {
                        *register_b &= 7;
                    }
                    (Some(2), Some(6)) => {
                        *register_b = *register_c & 7;
                    }
                    (Some(3), Some(operand)) => {
                        if *register_a != 0 {
                            self.program_counter = *operand as usize;
                            break;
                        }
                    }
                    (Some(4), Some(0..=7)) => {
                        *register_b ^= *register_c;
                    }
                    (Some(5), Some(operand @ 0..=3)) => {
                        self.output.push(*operand);
                    }
                    (Some(5), Some(4)) => {
                        self.output.push((*register_a & 7) as u8);
                    }
                    (Some(5), Some(5)) => {
                        self.output.push((*register_b & 7) as u8);
                    }
                    (Some(5), Some(6)) => {
                        self.output.push((*register_c & 7) as u8);
                    }
                    (Some(6), Some(0)) => {
                        *register_b = *register_a;
                    }
                    (Some(6), Some(1)) => {
                        *register_b = *register_a >> 1;
                    }
                    (Some(6), Some(2)) => {
                        *register_b = *register_a >> 2;
                    }
                    (Some(6), Some(3)) => {
                        *register_b = *register_a >> 3;
                    }
                    (Some(6), Some(4)) => {
                        *register_b = *register_a >> *register_a;
                    }
                    (Some(6), Some(5)) => {
                        *register_b = *register_a >> *register_b;
                    }
                    (Some(6), Some(6)) => {
                        *register_b = *register_a >> *register_c;
                    }
                    (Some(7), Some(0)) => {
                        *register_c = *register_a;
                    }
                    (Some(7), Some(1)) => {
                        *register_c = *register_a >> 1;
                    }
                    (Some(7), Some(2)) => {
                        *register_c = *register_a >> 2;
                    }
                    (Some(7), Some(3)) => {
                        *register_c = *register_a >> 3;
                    }
                    (Some(7), Some(4)) => {
                        *register_c = *register_a >> *register_a;
                    }
                    (Some(7), Some(5)) => {
                        *register_c = *register_a >> *register_b;
                    }
                    (Some(7), Some(6)) => {
                        *register_c = *register_a >> *register_c;
                    }
                    (None, None) => {
                        return;
                    }
                    (_, _) => panic!("Poorly formed program."),
                }
            }
        }
    }
}

#[aoc(day17, part1)]
pub fn part1(input: &str) -> String {
    let mut machine = parse(input);

    machine.run();

    machine
        .output
        .into_iter()
        .map(|d| d.to_string())
        .collect::<Vec<_>>()
        .join(",")
}

#[aoc(day17, part2)]
pub fn part2(input: &str) -> u64 {
    let mut machine = parse(input);

    // I'm only going to solve for this specific program,
    // not all all programs that this machine could run.
    assert_eq!(
        machine.program,
        vec![2, 4, 1, 7, 7, 5, 1, 7, 4, 6, 0, 3, 5, 5, 3, 0]
    );

    // This program consumes the low three bits from register A each iteration and mixes them
    // with higher bits to produce an output, until register A is zero.

    // We'll build up candidate values for register A from the end of the program.
    let mut prefixes = vec![0u64];
    let mut new_prefixes = vec![];
    for output_bits in machine.program.iter().rev() {
        new_prefixes.clear();
        for prefix in &prefixes {
            for new_prefix in (prefix << 3)..=((prefix << 3) + 7) {
                if new_prefix > 0
                    && (((new_prefix >> ((new_prefix & 7) ^ 7)) ^ new_prefix) & 7) as u8
                        == *output_bits
                {
                    new_prefixes.push(new_prefix);
                }
            }
        }
        mem::swap(&mut prefixes, &mut new_prefixes);
    }
    let min_register_a = prefixes[0];

    machine.register_a = min_register_a;
    // machine.run();
    // assert_eq!(machine.output, machine.program);

    min_register_a
}
