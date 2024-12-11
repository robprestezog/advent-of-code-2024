use aoc_runner_derive::aoc;

struct Grid {
    columns: usize,
    rows: usize,
    grid: Vec<Vec<char>>,
}

#[aoc(day4, part1)]
fn part1(input: &str) -> u32 {
    let mut grid = Grid {
        columns: 0,
        rows: 0,
        grid: vec![],
    };
    input.lines().for_each(|line| {
        let row: Vec<char> = line.chars().collect();
        grid.columns = row.len();
        grid.grid.push(row);
        grid.rows += 1;
    });

    let mut total: u32 = 0;

    for y in 0..grid.rows {
        for x in 0..grid.columns {
            if grid.grid[y][x] == 'X' {
                // right
                if x + 3 < grid.columns {
                    if grid.grid[y][x + 1] == 'M'
                        && grid.grid[y][x + 2] == 'A'
                        && grid.grid[y][x + 3] == 'S'
                    {
                        total += 1;
                    }
                }
                // left
                if x >= 3 {
                    if grid.grid[y][x - 1] == 'M'
                        && grid.grid[y][x - 2] == 'A'
                        && grid.grid[y][x - 3] == 'S'
                    {
                        total += 1;
                    }
                }
                // up
                if y >= 3 {
                    if grid.grid[y - 1][x] == 'M'
                        && grid.grid[y - 2][x] == 'A'
                        && grid.grid[y - 3][x] == 'S'
                    {
                        total += 1;
                    }
                }
                // down
                if y + 3 < grid.rows {
                    if grid.grid[y + 1][x] == 'M'
                        && grid.grid[y + 2][x] == 'A'
                        && grid.grid[y + 3][x] == 'S'
                    {
                        total += 1;
                    }
                }
                // right down
                if x + 3 < grid.columns && y + 3 < grid.rows {
                    if grid.grid[y + 1][x + 1] == 'M'
                        && grid.grid[y + 2][x + 2] == 'A'
                        && grid.grid[y + 3][x + 3] == 'S'
                    {
                        total += 1;
                    }
                }
                // left down
                if x >= 3 && y + 3 < grid.rows {
                    if grid.grid[y + 1][x - 1] == 'M'
                        && grid.grid[y + 2][x - 2] == 'A'
                        && grid.grid[y + 3][x - 3] == 'S'
                    {
                        total += 1;
                    }
                }
                // right up
                if x + 3 < grid.columns && y >= 3 {
                    if grid.grid[y - 1][x + 1] == 'M'
                        && grid.grid[y - 2][x + 2] == 'A'
                        && grid.grid[y - 3][x + 3] == 'S'
                    {
                        total += 1;
                    }
                }
                // left up
                if x >= 3 && y >= 3 {
                    if grid.grid[y - 1][x - 1] == 'M'
                        && grid.grid[y - 2][x - 2] == 'A'
                        && grid.grid[y - 3][x - 3] == 'S'
                    {
                        total += 1;
                    }
                }
            }
        }
    }

    total
}

#[aoc(day4, part2)]
fn part2(input: &str) -> u32 {
    let mut grid = Grid {
        columns: 0,
        rows: 0,
        grid: vec![],
    };
    input.lines().for_each(|line| {
        let row: Vec<char> = line.chars().collect();
        grid.columns = row.len();
        grid.grid.push(row);
        grid.rows += 1;
    });

    let mut total: u32 = 0;

    for y in 1..(grid.rows - 1) {
        for x in 1..(grid.columns - 1) {
            if grid.grid[y][x] == 'A' {
                // right right
                if grid.grid[y - 1][x - 1] == 'M'
                    && grid.grid[y + 1][x + 1] == 'S'
                    && grid.grid[y + 1][x - 1] == 'M'
                    && grid.grid[y - 1][x + 1] == 'S'
                {
                    total += 1;
                }
                // right left
                if grid.grid[y - 1][x - 1] == 'M'
                    && grid.grid[y + 1][x + 1] == 'S'
                    && grid.grid[y + 1][x - 1] == 'S'
                    && grid.grid[y - 1][x + 1] == 'M'
                {
                    total += 1;
                }
                // left right
                if grid.grid[y - 1][x - 1] == 'S'
                    && grid.grid[y + 1][x + 1] == 'M'
                    && grid.grid[y + 1][x - 1] == 'M'
                    && grid.grid[y - 1][x + 1] == 'S'
                {
                    total += 1;
                }
                // left left
                if grid.grid[y - 1][x - 1] == 'S'
                    && grid.grid[y + 1][x + 1] == 'M'
                    && grid.grid[y + 1][x - 1] == 'S'
                    && grid.grid[y - 1][x + 1] == 'M'
                {
                    total += 1;
                }
            }
        }
    }

    total
}
