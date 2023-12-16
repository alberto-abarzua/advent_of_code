use std::collections::HashSet;

fn main() {
    println!("Solution:\n{:?}", solution(include_str!("../input.txt")));
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}
fn travel_path(
    grid: &Vec<Vec<char>>,
    visited: &mut HashSet<(i32, i32, Direction)>,
    from: Direction,
    loc: (i32, i32),
) -> usize {
    if loc.0 >= grid[0].len() as i32 || loc.1 >= grid.len() as i32 || loc.0 < 0 || loc.1 < 0 {
        return 0;
    }

    let (x, y) = loc;

    let hashkey = (x, y, from);
    if visited.contains(&hashkey) {
        return 0;
    }
    visited.insert(hashkey);

    match from {
        Direction::Up => {
            match grid[y as usize][x as usize] {
                '/' => travel_path(grid, visited, Direction::Right, (x - 1, y)),
                '\\' => travel_path(grid, visited, Direction::Left, (x + 1, y)),
                '|' => travel_path(grid, visited, Direction::Up, (x, y + 1)),
                '-' => {
                    travel_path(grid, visited, Direction::Left, (x + 1, y))
                        + travel_path(grid, visited, Direction::Right, (x - 1, y))
                }
                '.' => travel_path(grid, visited, Direction::Up, (x, y + 1)),
                _ => panic!("Invalid character"),
            };
        }
        Direction::Down => {
            match grid[y as usize][x as usize] {
                '/' => travel_path(grid, visited, Direction::Left, (x + 1, y)),
                '\\' => travel_path(grid, visited, Direction::Right, (x - 1, y)),
                '|' => travel_path(grid, visited, Direction::Down, (x, y - 1)),
                '-' => {
                    travel_path(grid, visited, Direction::Left, (x + 1, y))
                        + travel_path(grid, visited, Direction::Right, (x - 1, y))
                }
                '.' => travel_path(grid, visited, Direction::Down, (x, y - 1)),
                _ => panic!("Invalid character"),
            };
        }
        Direction::Left => {
            match grid[y as usize][x as usize] {
                '/' => travel_path(grid, visited, Direction::Down, (x, y - 1)),
                '\\' => travel_path(grid, visited, Direction::Up, (x, y + 1)),
                '|' => {
                    travel_path(grid, visited, Direction::Up, (x, y + 1))
                        + travel_path(grid, visited, Direction::Down, (x, y - 1))
                }
                '-' => travel_path(grid, visited, Direction::Left, (x + 1, y)),
                '.' => travel_path(grid, visited, Direction::Left, (x + 1, y)),
                _ => panic!("Invalid character"),
            };
        }
        Direction::Right => {
            match grid[y as usize][x as usize] {
                '/' => travel_path(grid, visited, Direction::Up, (x, y + 1)),
                '\\' => travel_path(grid, visited, Direction::Down, (x, y - 1)),
                '|' => {
                    travel_path(grid, visited, Direction::Up, (x, y + 1))
                        + travel_path(grid, visited, Direction::Down, (x, y - 1))
                }
                '-' => travel_path(grid, visited, Direction::Right, (x - 1, y)),
                '.' => travel_path(grid, visited, Direction::Right, (x - 1, y)),
                _ => panic!("Invalid character"),
            };
        }
    };

    0
}

fn calc(start: (i32, i32), from: Direction, mut grid: Vec<Vec<char>>) -> usize {
    let mut visited = HashSet::new();

    travel_path(&grid, &mut visited, from, start);

    for (x, y, _) in visited.iter() {
        grid[*y as usize][*x as usize] = '#';
    }

    // count number of #
    grid.iter()
        .map(|row| row.iter().filter(|c| **c == '#').count())
        .sum::<usize>()
}

fn solution(input: &str) -> usize {
    let grid = input
        .lines()
        .map(|line| line.chars().collect::<Vec<char>>())
        .collect::<Vec<Vec<char>>>();
    let mut max = 0;

    // Top edge
    for x in 0..grid[0].len() {
        let res = calc((x as i32, 0), Direction::Up, grid.clone());
        if res > max {
            max = res;
        }
    }

    // Bottom edge
    for x in 0..grid[0].len() {
        let res = calc((x as i32, grid.len() as i32), Direction::Down, grid.clone());
        if res > max {
            max = res;
        }
    }

    // Left edge
    for y in 0..grid.len() {
        let res = calc((0, y as i32), Direction::Left, grid.clone());
        if res > max {
            max = res;
        }
    }

    // Right edge
    for y in 0..grid.len() {
        let res = calc(
            (grid[0].len() as i32, y as i32),
            Direction::Right,
            grid.clone(),
        );
        if res > max {
            max = res;
        }
    }

    max
}
