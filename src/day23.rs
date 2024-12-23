use aoc_runner_derive::aoc;
use std::collections::HashSet;

#[aoc(day23, part1)]
pub fn part1(input: &str) -> u32 {
    let mut total = 0;

    let mut tx_neighbors = vec![HashSet::<(u8, u8)>::new(); 26];

    let mut neighbors = HashSet::<(u8, u8, u8, u8)>::new();

    for line in input.lines() {
        let bytes = line.as_bytes();
        if bytes[0] == 't' as u8 {
            tx_neighbors[(bytes[1] - 'a' as u8) as usize].insert((bytes[3], bytes[4]));
        }
        if bytes[3] == 't' as u8 {
            tx_neighbors[(bytes[4] - 'a' as u8) as usize].insert((bytes[0], bytes[1]));
        }

        neighbors.insert((bytes[0], bytes[1], bytes[3], bytes[4]));
    }

    for set in tx_neighbors {
        for (a1, a2) in &set {
            for (b1, b2) in &set {
                if neighbors.contains(&(*a1, *a2, *b1, *b2)) {
                    total += 2;
                    if *a1 == 't' as u8 {
                        total -= 1;
                    }
                    if *b1 == 't' as u8 {
                        total -= 1;
                    }
                    if *a1 == 't' as u8 && *b1 == 't' as u8 {
                        total += 2;
                    }
                }
            }
        }
    }

    total / 2
}

#[derive(Clone)]
struct Edge(u16, u16);

fn find_largest_clique(edges: &[Edge]) -> Vec<u16> {
    if edges.len() == 0 {
        return vec![];
    }
    if edges.len() == 1 {
        return vec![edges[0].0, edges[0].1];
    }
    // Select a node to pivot on
    let pivot = edges[0].0;

    let neighbors: HashSet<u16> = edges
        .iter()
        .filter_map(|e| {
            if e.0 == pivot {
                Some(e.1)
            } else if e.1 == pivot {
                Some(e.0)
            } else {
                None
            }
        })
        .collect();

    let non_pivot_edges: Vec<Edge> = edges
        .iter()
        .filter_map(|e| {
            if e.0 == pivot {
                None
            } else if e.1 == pivot {
                None
            } else {
                Some(e.clone())
            }
        })
        .collect();

    let non_pivot_clique = find_largest_clique(&non_pivot_edges);

    if non_pivot_clique.len() > neighbors.len() {
        return non_pivot_clique;
    }

    let neighbors_edges: Vec<Edge> = edges
        .iter()
        .filter_map(|e| {
            if neighbors.contains(&e.0) && neighbors.contains(&e.1) {
                Some(e.clone())
            } else {
                None
            }
        })
        .collect();

    let mut pivot_neighbors_clique = find_largest_clique(&neighbors_edges);

    if non_pivot_clique.len() > pivot_neighbors_clique.len() {
        non_pivot_clique
    } else {
        pivot_neighbors_clique.push(pivot);
        pivot_neighbors_clique
    }
}

#[aoc(day23, part2)]
pub fn part2(input: &str) -> String {
    let mut edges: Vec<_> = input
        .lines()
        .map(|line| {
            let bytes = line.as_bytes();
            if bytes[0] < bytes[3] {
                Edge(
                    (bytes[0] - 'a' as u8) as u16 * 26 + (bytes[1] - 'a' as u8) as u16,
                    (bytes[3] - 'a' as u8) as u16 * 26 + (bytes[4] - 'a' as u8) as u16,
                )
            } else if bytes[0] > bytes[3] {
                Edge(
                    (bytes[3] - 'a' as u8) as u16 * 26 + (bytes[4] - 'a' as u8) as u16,
                    (bytes[0] - 'a' as u8) as u16 * 26 + (bytes[1] - 'a' as u8) as u16,
                )
            } else {
                if bytes[1] < bytes[4] {
                    Edge(
                        (bytes[0] - 'a' as u8) as u16 * 26 + (bytes[1] - 'a' as u8) as u16,
                        (bytes[3] - 'a' as u8) as u16 * 26 + (bytes[4] - 'a' as u8) as u16,
                    )
                } else if bytes[1] > bytes[4] {
                    Edge(
                        (bytes[3] - 'a' as u8) as u16 * 26 + (bytes[4] - 'a' as u8) as u16,
                        (bytes[0] - 'a' as u8) as u16 * 26 + (bytes[1] - 'a' as u8) as u16,
                    )
                } else {
                    panic!("unexpected self edge");
                }
            }
        })
        .collect();

    let mut clique = find_largest_clique(&mut edges);
    clique.sort();

    let bytes: Vec<u8> = clique
        .into_iter()
        .flat_map(|code| {
            [
                (code / 26) as u8 + 'a' as u8,
                (code % 26) as u8 + 'a' as u8,
                ',' as u8,
            ]
        })
        .collect();

    String::from_utf8(bytes)
        .unwrap()
        .strip_suffix(',')
        .unwrap()
        .to_string()
}
