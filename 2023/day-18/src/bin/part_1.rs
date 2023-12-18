fn main() {
    println!("Solution:\n{:?}", solution(include_str!("../input.txt")))
}

#[derive(Debug, Clone)]
struct Node {
    char: char,
}

fn solution(input: &str) -> usize {
    let values: Vec<(char, usize, &str)> = input
        .lines()
        .map(|line| {
            let (char, rest) = line.split_once(" ").unwrap();
            let char = char.chars().nth(0).unwrap();
            let (num, rest) = rest.split_once(" ").unwrap();
            let info = rest.strip_prefix("(").unwrap().strip_suffix(")").unwrap();
            let num = num.parse::<usize>().unwrap();
            (char, num, info)
        })
        .collect();

    let grid_size = 3000;

    let mut grid: Vec<Vec<Option<Node>>> = vec![vec![None]; grid_size];

    for i in 0..grid_size {
        grid[i] = vec![None; grid_size];
    }

    let mut cur: (usize, usize) = (grid_size / 2, grid_size / 2);

    for (dir, num, _info) in values.iter() {
        let (mut r, mut c) = cur;
        for _ in 0..*num {
            match &grid[r][c] {
                None => {
                    grid[r][c] = Some(Node { char: 'X' });
                }
                _ => {}
            }

            match dir {
                'R' => {
                    if c < grid_size - 1 {
                        c += 1
                    } else {
                        panic!("Out of bounds in {:?} at {:?}", dir, cur);
                    }
                }
                'L' => {
                    if c > 0 {
                        c -= 1
                    } else {
                        panic!("Out of bounds in {:?} at {:?}", dir, cur);
                    }
                }
                'D' => {
                    if r < grid_size - 1 {
                        r += 1
                    } else {
                        panic!("Out of bounds in {:?} at {:?}", dir, cur);
                    }
                }
                'U' => {
                    if r > 0 {
                        r -= 1
                    } else {
                        panic!("Out of bounds in {:?} at {:?}", dir, cur);
                    }
                }
                _ => panic!("Unknown direction"),
            }
        }
        cur = (r, c);
    }

    let point;
    loop {
        let r = rand::random::<usize>() % grid_size;
        let c = rand::random::<usize>() % grid_size;
        match &grid[r][c] {
            Some(node) => {
                if node.char == 'X' {
                    continue;
                }
            }
            None => {}
        }
        if is_inside(&grid, r as i32, c as i32) {
            point = (r, c);
            break;
        }
    }

    flood_fill(&mut grid, point.0 as i32, point.1 as i32);

    let mut b = 0;
    let mut i = 0;
    for row in grid.iter() {
        for col in row.iter() {
            match col {
                Some(node) => {
                    if node.char == 'X' {
                        b += 1;
                    } else if node.char == 'D' {
                        i += 1;
                    }
                }
                None => {}
            }
        }
    }

    i + b
}

fn is_inside(grid: &Vec<Vec<Option<Node>>>, r: i32, c: i32) -> bool {
    let right_range = c..grid[0].len() as i32;
    let left_range = 0..c;

    let mut crossing = false;
    let mut num_right = 0;
    for i in right_range {
        match &grid[r as usize][i as usize] {
            Some(node) => {
                if node.char == 'X' {
                    if !crossing {
                        num_right += 1;
                    }
                    crossing = true;
                }
            }
            None => {
                crossing = false;
            }
        }
    }

    let mut crossing = false;
    let mut num_left = 0;
    for i in left_range.rev() {
        match &grid[r as usize][i as usize] {
            Some(node) => {
                if node.char == 'X' {
                    if !crossing {
                        num_left += 1;
                    }
                    crossing = true;
                }
            }
            None => {
                crossing = false;
            }
        }
    }

    num_right % 2 == 1 && num_left % 2 == 1
}

fn flood_fill(grid: &mut Vec<Vec<Option<Node>>>, r: i32, c: i32) {
    if r < 0 || r >= grid.len() as i32 || c < 0 || c >= grid[0].len() as i32 {
        return;
    }

    match &grid[r as usize][c as usize] {
        None => {
            grid[r as usize][c as usize] = Some(Node { char: 'D' });
        }
        _ => return,
    }

    flood_fill(grid, r + 1, c);
    flood_fill(grid, r - 1, c);
    flood_fill(grid, r, c + 1);
    flood_fill(grid, r, c - 1);
}
