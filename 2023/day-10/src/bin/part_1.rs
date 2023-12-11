fn main() {
    let input = include_str!("../input.txt");
    println!("Solution is:\n{}", solution(input));
}

#[derive(Debug, Copy, Clone)]
struct Pipe {
    row: usize,
    col: usize,
    ch: char,
}

#[derive(Debug, Copy, Clone)]
enum Direction {
    North,
    South,
    East,
    West,
}
impl Direction {
    fn opposite(&self) -> Direction {
        match self {
            Direction::North => Direction::South,
            Direction::South => Direction::North,
            Direction::East => Direction::West,
            Direction::West => Direction::East,
        }
    }
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

fn solution(input: &str) -> i32 {
    let mut grid: Vec<Vec<Pipe>> = Vec::new();
    let (mut s_row, mut s_col) = (0, 0);

    for (row, line) in input.lines().enumerate() {
        grid.push(Vec::new());
        for (col, char) in line.chars().enumerate() {
            grid[row].push(Pipe { row, col, ch: char });
            if char == 'S' {
                (s_row, s_col) = (row, col);
            }
        }
    }

    let start = grid[s_row][s_col];

    let mut current = start;
    let mut current_direction = Direction::South;

    let directions = vec![
        Direction::North,
        Direction::South,
        Direction::East,
        Direction::West,
    ];

    for top_direction in &directions {
        let next = start.get_from_direction(&grid, *top_direction);
        if next.is_some() {
            for direction in &directions {
                let (temp_next, dir) = next.unwrap().next(&grid, *direction);
                if temp_next.is_some() && temp_next.unwrap().ch == 'S' {
                    current_direction = dir.opposite();
                    current = next.unwrap();
                    break;
                }
            }
        }
    }

    let mut steps = 1;
    loop {
        let (next, dir) = current.next(&grid, current_direction);
        if next.is_some() {
            current = next.unwrap();
            current_direction = dir;

            steps += 1;
        } else {
            break;
        }
    }

    steps / 2
}
