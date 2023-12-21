use std::{
    collections::{HashMap, VecDeque},
    i64,
};

fn main() {
    println!("Solution:\n{:?}", solution(include_str!("../input.txt")));
}

fn find_start(grid: &[Vec<char>]) -> Option<(usize, usize)> {
    for (i, row) in grid.iter().enumerate() {
        for (j, &col) in row.iter().enumerate() {
            if col == 'S' {
                return Some((i, j));
            }
        }
    }
    None
}

fn solution(input: &str) -> i64 {
    let mut grid: Vec<Vec<char>> = input.lines().map(|l| l.chars().collect()).collect();
    let (r, c) = match find_start(&grid) {
        Some(start) => start,
        None => return 0,
    };
    grid[r][c] = '.';
    let num_steps = 64;

    let explored = bfs(&grid, (r, c), num_steps);

    let mut count = 0;
    for v in explored.values() {
        if *v % 2 == 0 {
            count += 1;
        }
    }
    count as i64
}

fn bfs(grid: &[Vec<char>], start: (usize, usize), distance: i64) -> HashMap<(usize, usize), i64> {
    let mut explored: HashMap<(usize, usize), i64> = HashMap::new();
    let mut queue = VecDeque::new();
    queue.push_back((start, 0));
    explored.insert(start, 0);

    while let Some(((r, c), steps)) = queue.pop_front() {
        if steps >= distance {
            continue;
        }

        for (dr, dc) in [(0, -1), (0, 1), (-1, 0), (1, 0)] {
            let (nr, nc) = (r as i64 + dr, c as i64 + dc);
            if nr < 0 || nr >= grid.len() as i64 || nc < 0 || nc >= grid[0].len() as i64 {
                continue;
            }
            let next_pos = (nr as usize, nc as usize);
            if grid[nr as usize][nc as usize] == '#' || explored.contains_key(&next_pos) {
                continue;
            }
            explored.insert(next_pos, steps + 1);
            queue.push_back((next_pos, steps + 1));
        }
    }
    explored
}
