use aoc_runner_derive::aoc;

struct Region {
    id: usize,
    root: usize,
    area: u32,
    perimeter: u32,
    sides: u32,
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
    fn get(&mut self, id: usize) -> &mut Region {
        let root = self.get_root(id);
        &mut self.regions[root]
    }
    pub fn new_region(&mut self, area: u32, perimeter: u32, sides: u32) -> usize {
        let id = self.regions.len();
        self.regions.push(Region {
            id,
            root: id,
            area,
            perimeter,
            sides,
        });
        id
    }
    pub fn merge(&mut self, id1: usize, id2: usize) -> usize {
        let root1 = self.get_root(id1);
        let root2 = self.get_root(id2);
        if root1 == root2 {
            return root1;
        }
        let region2 = self.get(root2);
        let area = region2.area;
        let perimeter = region2.perimeter;
        let sides = region2.sides;
        region2.area = 0;
        region2.perimeter = 0;
        region2.sides = 0;
        region2.root = root1;

        self.increase(root1, area, perimeter, sides);
        root1
    }
    pub fn increase(&mut self, id: usize, area: u32, perimeter: u32, sides: u32) {
        let region = self.get(id);
        region.area += area;
        region.perimeter += perimeter;
        region.sides += sides;
    }
    pub fn total(self) -> u32 {
        let mut total = 0;
        for region in self.regions {
            if region.id > 0 && region.id == region.root {
                total += region.perimeter * region.area;
            }
        }
        total
    }
    pub fn total2(self) -> u32 {
        let mut total = 0;
        for region in self.regions {
            if region.id > 0 && region.id == region.root {
                total += region.sides * region.area;
            }
        }
        total
    }
}

#[derive(Clone)]
struct Plot {
    region_id: usize,
    kind: char,
    top_wall: bool,
    left_wall: bool,
}

fn process(input: &str) -> DisjointRegions {
    let mut regions = DisjointRegions::new();
    let border_region_id = regions.new_region(0, 0, 0);

    let mut plots: Vec<Plot> = vec![];
    input.lines().for_each(|line| {
        if plots.len() < line.len() + 1 {
            plots = vec![
                Plot {
                    region_id: border_region_id,
                    kind: ' ',
                    top_wall: false,
                    left_wall: false,
                };
                line.len() + 1
            ];
        }

        line.chars().enumerate().for_each(|(index, kind)| {
            match (plots[index].kind, plots[index + 1].kind, kind) {
                (left, up, cur) if left == cur && up == cur => {
                    // Merge Left and Up if needed.
                    let id = regions.merge(plots[index].region_id, plots[index + 1].region_id);
                    // Add one area
                    regions.increase(id, 1, 0, 0);
                    // Update Plot
                    plots[index + 1] = Plot {
                        region_id: plots[index].region_id,
                        kind: cur,
                        top_wall: false,
                        left_wall: false,
                    };
                }
                (left, _up, cur) if left == cur => {
                    // Left gets 1 area and 1 fence.
                    if plots[index].top_wall {
                        regions.increase(plots[index].region_id, 1, 1, 0);
                    } else {
                        regions.increase(plots[index].region_id, 1, 1, 1);
                    }
                    // Up gets a fence.
                    if plots[index].top_wall && !plots[index + 1].left_wall {
                        regions.increase(plots[index + 1].region_id, 0, 1, 0);
                    } else {
                        regions.increase(plots[index + 1].region_id, 0, 1, 1);
                    }
                    // Update Plot
                    plots[index + 1] = Plot {
                        region_id: plots[index].region_id,
                        kind: cur,
                        top_wall: true,
                        left_wall: false,
                    };
                }
                (_left, up, cur) if up == cur => {
                    // Left gets a fence.
                    if !plots[index].top_wall && plots[index + 1].left_wall {
                        regions.increase(plots[index].region_id, 0, 1, 0);
                    } else {
                        regions.increase(plots[index].region_id, 0, 1, 1);
                    }
                    // Up gets 1 area and 1 fence.
                    if plots[index + 1].left_wall {
                        regions.increase(plots[index + 1].region_id, 1, 1, 0);
                    } else {
                        regions.increase(plots[index + 1].region_id, 1, 1, 1);
                    }
                    // Update Plot
                    plots[index + 1] = Plot {
                        region_id: plots[index + 1].region_id,
                        kind: cur,
                        top_wall: false,
                        left_wall: true,
                    };
                }
                (_left, _up, cur) => {
                    // Left gets a fence.
                    if !plots[index].top_wall && plots[index + 1].left_wall {
                        regions.increase(plots[index].region_id, 0, 1, 0);
                    } else {
                        regions.increase(plots[index].region_id, 0, 1, 1);
                    }
                    // Up gets a fence.
                    if plots[index].top_wall && !plots[index + 1].left_wall {
                        regions.increase(plots[index + 1].region_id, 0, 1, 0);
                    } else {
                        regions.increase(plots[index + 1].region_id, 0, 1, 1);
                    }
                    // Update Plot
                    let id = regions.new_region(1, 2, 2);
                    plots[index + 1] = Plot {
                        region_id: id,
                        kind: cur,
                        top_wall: true,
                        left_wall: true,
                    };
                }
            };
        });
        // At the end of the line, add a fence to the last plot.
        if !plots[line.len()].top_wall {
            regions.increase(plots[line.len()].region_id, 0, 1, 0)
        } else {
            regions.increase(plots[line.len()].region_id, 0, 1, 1)
        }
    });
    // After the last line, add fences to all the plots.
    for plot in plots.iter() {
        if !plot.left_wall {
            regions.increase(plot.region_id, 0, 1, 0);
        } else {
            regions.increase(plot.region_id, 0, 1, 1);
        }
    }
    regions
}

#[aoc(day12, part1)]
pub fn part1(input: &str) -> u32 {
    process(input).total()
}

#[aoc(day12, part2)]
pub fn part2(input: &str) -> u32 {
    process(input).total2()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_example() {
        assert_eq!(
            part1(
                "RRRRIICCFF
RRRRIICCCF
VVRRRCCFFF
VVRCCCJFFF
VVVVCJJCFE
VVIVCCJJEE
VVIIICJJEE
MIIIIIJJEE
MIIISIJEEE
MMMISSJEEE"
            ),
            1930
        );
    }

    #[test]
    fn part2_example() {
        assert_eq!(
            part2(
                "RRRRIICCFF
RRRRIICCCF
VVRRRCCFFF
VVRCCCJFFF
VVVVCJJCFE
VVIVCCJJEE
VVIIICJJEE
MIIIIIJJEE
MIIISIJEEE
MMMISSJEEE"
            ),
            1206
        );
    }
}
