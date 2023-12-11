use std::collections::HashSet;
fn main() {
    let input = include_str!("../input.txt");
    println!("Solution is:\n{}", solution(input));
}

#[derive(Debug, Copy, Clone)]
struct Pipe {
    row: usize,
    col: usize,
    ch: char,
    border: bool,
}

#[derive(Debug, Copy, Clone)]
enum Direction {
    North,
    South,
    East,
    West,
}

impl Pipe {
    fn next(&self, grid: &Vec<Vec<Pipe>>, from: Direction) -> (Option<Pipe>, Direction) {
        match from {
            Direction::North => match self.ch {
                '|' => (
                    self.get_from_direction(grid, Direction::South),
                    Direction::North,
                ),
                'L' => (
                    self.get_from_direction(grid, Direction::East),
                    Direction::West,
                ),
                'J' => (
                    self.get_from_direction(grid, Direction::West),
                    Direction::East,
                ),
                _ => (Option::None, Direction::North),
            },
            Direction::South => match self.ch {
                '7' => (
                    self.get_from_direction(grid, Direction::West),
                    Direction::East,
                ),
                'F' => (
                    self.get_from_direction(grid, Direction::East),
                    Direction::West,
                ),
                '|' => (
                    self.get_from_direction(grid, Direction::North),
                    Direction::South,
                ),
                _ => (Option::None, Direction::North),
            },
            Direction::East => match self.ch {
                'L' => (
                    self.get_from_direction(grid, Direction::North),
                    Direction::South,
                ),
                'F' => (
                    self.get_from_direction(grid, Direction::South),
                    Direction::North,
                ),
                '-' => (
                    self.get_from_direction(grid, Direction::West),
                    Direction::East,
                ),
                _ => (Option::None, Direction::North),
            },
            Direction::West => match self.ch {
                '7' => (
                    self.get_from_direction(grid, Direction::South),
                    Direction::North,
                ),
                'J' => (
                    self.get_from_direction(grid, Direction::North),
                    Direction::South,
                ),
                '-' => (
                    self.get_from_direction(grid, Direction::East),
                    Direction::West,
                ),
                _ => (Option::None, Direction::North),
            },
        }
    }

    fn get_from_direction(&self, grid: &Vec<Vec<Pipe>>, dir: Direction) -> Option<Pipe> {
        match dir {
            Direction::North => {
                if self.row > 0 {
                    Some(grid[self.row - 1][self.col])
                } else {
                    None
                }
            }
            Direction::South => {
                if self.row < grid.len() - 1 {
                    Some(grid[self.row + 1][self.col])
                } else {
                    None
                }
            }
            Direction::East => {
                if self.col < grid[self.row].len() - 1 {
                    Some(grid[self.row][self.col + 1])
                } else {
                    None
                }
            }
            Direction::West => {
                if self.col > 0 {
                    Some(grid[self.row][self.col - 1])
                } else {
                    None
                }
            }
        }
    }
}

fn is_inside(grid: &Vec<Vec<Pipe>>) -> i32 {
    let mut outside = std::collections::HashSet::new();
    let mut loop_set = std::collections::HashSet::new();

    for (r, row) in grid.iter().enumerate() {
        for (c, pipe) in row.iter().enumerate() {
            if pipe.border {
                loop_set.insert((r, c));
            }
        }
    }

    for (r, row) in grid.iter().enumerate() {
        let mut within = false;
        let mut up = None;

        for (c, pipe) in row.iter().enumerate() {
            match pipe.ch {
                '|' => {
                    within = !within;
                }
                '-' => {}
                'L' | 'F' => {
                    up = Some(pipe.ch == 'L');
                }
                '7' | 'J' => {
                    if pipe.ch != (if up.unwrap_or(false) { 'J' } else { '7' }) {
                        within = !within;
                    }
                    up = None;
                }
                '.' => {}
                _ => panic!("unexpected character (horizontal): {}", pipe.ch),
            }
            if !within {
                outside.insert((r, c));
            }
        }
    }

    let total_cells = grid.len() * grid[0].len();

    let outside_or_loop = outside.union(&loop_set).collect::<HashSet<_>>().len();

    (total_cells as i32) - (outside_or_loop as i32)
}

fn solution(input: &str) -> i32 {
    let mut grid: Vec<Vec<Pipe>> = Vec::new();
    let (mut s_row, mut s_col) = (0, 0);
    let mut tiles: Vec<Pipe> = Vec::new();

    for (row, line) in input.lines().enumerate() {
        grid.push(Vec::new());
        for (col, char) in line.chars().enumerate() {
            grid[row].push(Pipe {
                row,
                col,
                ch: char,
                border: false,
            });
            if char == 'S' {
                (s_row, s_col) = (row, col);
            }
            if char == '.' {
                tiles.push(grid[row][col].clone());
            }
        }
    }

    let mut start = grid[s_row][s_col];
    start.border = true;

    let directions = vec![
        Direction::North,
        Direction::South,
        Direction::East,
        Direction::West,
    ];

    let possible_chars = vec!['|', '-', 'L', 'J', '7', 'F'];

    'outer: for possible in possible_chars {
        for dir in directions.iter() {
            let mut current_direction = *dir;
            grid[s_row][s_col].ch = possible;
            let mut current = grid[s_row][s_col];
            loop {
                let (next, dir) = current.next(&grid, current_direction);
                if next.is_some() {
                    current = next.unwrap();
                    grid[current.row][current.col].border = true;
                    current_direction = dir;
                } else {
                    break;
                }

                if current.row == s_row && current.col == s_col {
                    break 'outer;
                }
            }
        }
    }

    for row in grid.iter_mut() {
        for pipe in row.iter_mut() {
            if !pipe.border {
                pipe.ch = '.';
            }
        }
    }

    is_inside(&grid)
}
