use aoc_runner_derive::aoc;

const MAX_NODE: usize = 26 * 26;
const MAX_NODE_2: usize = MAX_NODE * MAX_NODE;

const HIGH_T: u16 = ('t' as u8 - 'a' as u8) as u16 * 26;
const HIGH_U: u16 = ('u' as u8 - 'a' as u8) as u16 * 26;
const LOW_T: u8 = 't' as u8 - 'a' as u8;

#[aoc(day23, part1)]
pub fn part1(input: &str) -> u32 {
    let mut total = 0;

    let mut tx_neighbors = vec![Vec::with_capacity(MAX_NODE); 26];
    let mut neighbors = [false; MAX_NODE_2];

    for line in input.lines() {
        let bytes = line.as_bytes();
        let a1 = bytes[0] - 'a' as u8;
        let a2 = bytes[1] - 'a' as u8;
        let b1 = bytes[3] - 'a' as u8;
        let b2 = bytes[4] - 'a' as u8;
        let a = (a1 as u16 * 26) + a2 as u16;
        let b = (b1 as u16 * 26) + b2 as u16;

        if a1 == LOW_T {
            tx_neighbors[a2 as usize].push(b);
        }
        if b1 == LOW_T {
            tx_neighbors[b2 as usize].push(a);
        }
        neighbors[(a as usize * MAX_NODE) + (b as usize)] = true;
    }

    for set in tx_neighbors {
        for a in &set {
            for b in &set {
                if neighbors[(*a as usize * MAX_NODE) + (*b as usize)] {
                    total += 2;
                    if *a >= HIGH_T && *a < HIGH_U {
                        if *b < HIGH_T || *b >= HIGH_U {
                            total -= 1;
                        }
                    } else if *b >= HIGH_T && *b < HIGH_U {
                        total -= 1;
                    }
                }
            }
        }
    }

    total / 2
}

#[derive(Clone)]
struct Edge(u16, u16);

fn get_non_pivot_edges(edges: &mut [Edge], pivot: u16) -> &mut [Edge] {
    let mut i = 0;

    for j in 0..edges.len() {
        if edges[j].0 == pivot || edges[j].1 == pivot {
            edges.swap(j, i);
            i += 1;
        }
    }
    &mut edges[i..]
}

fn get_pivot_neighbor_edges<'a>(edges: &'a mut [Edge], neighbors: &[bool]) -> &'a mut [Edge] {
    let mut i = 0;
    for j in 0..edges.len() {
        if neighbors[edges[j].0 as usize] && neighbors[edges[j].1 as usize] {
            edges.swap(j, i);
            i += 1;
        }
    }
    &mut edges[..i]
}

fn find_largest_clique(edges: &mut [Edge], neighbors: &[bool]) -> Vec<u16> {
    if edges.len() == 0 {
        return vec![];
    }
    if edges.len() == 1 {
        return vec![edges[0].0, edges[0].1];
    }
    // Select a node to pivot on
    let pivot = edges[0].0;

    let non_pivot_edges = get_non_pivot_edges(edges, pivot);
    let non_pivot_clique = find_largest_clique(non_pivot_edges, neighbors);

    let pivot_neighbor_edges = get_pivot_neighbor_edges(
        non_pivot_edges,
        &neighbors[(pivot as usize * MAX_NODE)..((pivot + 1) as usize * MAX_NODE)],
    );
    if pivot_neighbor_edges.len() * 2 < non_pivot_clique.len() * (non_pivot_clique.len() - 1) {
        // We don't have enough edges to make a clique as big as this one.
        return non_pivot_clique;
    }

    let mut pivot_neighbor_clique = find_largest_clique(pivot_neighbor_edges, neighbors);

    if non_pivot_clique.len() > pivot_neighbor_clique.len() {
        non_pivot_clique
    } else {
        pivot_neighbor_clique.push(pivot);
        pivot_neighbor_clique
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

    let mut neighbors = [false; MAX_NODE_2];
    for Edge(i, j) in &edges {
        neighbors[*i as usize * MAX_NODE + *j as usize] = true;
        neighbors[*j as usize * MAX_NODE + *i as usize] = true;
    }

    let mut clique = find_largest_clique(&mut edges, &neighbors);
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
