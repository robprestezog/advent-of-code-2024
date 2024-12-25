use std::collections::HashMap;

use aoc_runner_derive::aoc;

struct Input {
    name: [u8; 3],
    value: bool,
}

enum GateType {
    And,
    Or,
    Xor,
}

struct Gate {
    name1: [u8; 3],
    name2: [u8; 3],
    gate_type: GateType,
    name3: [u8; 3],
}

fn gate_map_insert(gate_map: &mut HashMap<[u8; 3], Vec<usize>>, name: &[u8], gate_id: usize) {
    let key = [name[0], name[1], name[2]];
    if !gate_map.contains_key(&key) {
        gate_map.insert(key, vec![gate_id]);
    } else {
        gate_map.get_mut(&key).unwrap().push(gate_id);
    }
}

#[aoc(day24, part1)]
fn part1(input_str: &str) -> u64 {
    let mut iter = input_str.lines();

    let mut input_queue = vec![];
    let mut signals: HashMap<[u8; 3], bool> = HashMap::new();

    loop {
        let line = iter.next().unwrap();
        if line.len() == 0 {
            break;
        }

        let (name, truth) = line.split_once(": ").unwrap();
        let name = name.as_bytes();
        let name = [name[0], name[1], name[2]];
        let truth = truth.parse::<u8>().unwrap() != 0;
        input_queue.push(Input { name, value: truth });
        signals.insert(name, truth);
    }

    let mut gate_map: HashMap<[u8; 3], Vec<usize>> = HashMap::new();

    let gates: Vec<_> = iter
        .enumerate()
        .map(|(i, line)| {
            let mut tokens = line.split_ascii_whitespace();
            let name1 = tokens.next().unwrap().as_bytes();
            let gate_type = tokens.next().unwrap();
            let name2 = tokens.next().unwrap().as_bytes();
            tokens.next();
            let name3 = tokens.next().unwrap().as_bytes();
            gate_map_insert(&mut gate_map, name1, i);
            gate_map_insert(&mut gate_map, name2, i);
            Gate {
                name1: [name1[0], name1[1], name1[2]],
                name2: [name2[0], name2[1], name2[2]],
                gate_type: match gate_type {
                    "AND" => GateType::And,
                    "OR" => GateType::Or,
                    "XOR" => GateType::Xor,
                    _ => panic!("Unexpected gate type"),
                },
                name3: [name3[0], name3[1], name3[2]],
            }
        })
        .collect();

    let mut z = 0;

    let mut gate_queue = vec![];

    loop {
        let input = match input_queue.pop() {
            None => {
                break;
            }
            Some(x) => x,
        };

        if input.name[0] == 'z' as u8 && input.value {
            z += (1 as u64) << ((input.name[1] - '0' as u8) * 10 + (input.name[2] - '0' as u8));
        }
        if gate_map.contains_key(&input.name) {
            for id in gate_map.get(&input.name).unwrap() {
                gate_queue.push(*id);
            }
        }

        loop {
            let gate = match gate_queue.pop() {
                None => {
                    break;
                }
                Some(x) => &gates[x],
            };

            if let Some(_) = signals.get(&gate.name3) {
                continue;
            }

            let value1 = match signals.get(&gate.name1) {
                None => {
                    continue;
                }
                Some(truth) => *truth,
            };
            let value2 = match signals.get(&gate.name2) {
                None => {
                    continue;
                }
                Some(truth) => *truth,
            };

            let value3 = match gate.gate_type {
                GateType::And => value1 && value2,
                GateType::Or => value1 || value2,
                GateType::Xor => value1 ^ value2,
            };
            signals.insert(gate.name3, value3);
            input_queue.push(Input {
                name: gate.name3,
                value: value3,
            });
        }
    }

    z
}

#[aoc(day24, part2)]
fn part2(_input: &str) -> String {
    "Done by hand".to_string()
}
