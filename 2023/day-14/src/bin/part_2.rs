use std::collections::HashMap;
use cached::proc_macro::cached;
fn main() {
    println!("Solution:\n{:?}", solution(include_str!("../input.txt")));
}

#[cached(size = 128000)]
fn roll_right(grid: Vec<Vec<char>>) -> Vec<Vec<char>> {
    let mut grid = grid.clone();
    for row in grid.iter_mut() {
        for idx in 0..row.len() {
            let i = row.len() - idx - 1;
            if row[i] == 'O' {
                let mut cur = i;
                while row[cur] != '#' && cur < row.len() - 1 {
                    if row[cur + 1] == '#' || row[cur + 1] == 'O' {
                        break;
                    }
                    row[cur] = '.';
                    row[cur + 1] = 'O';
                    cur += 1;
                }
            }
        }
    }
    grid
}

fn reverse(grid: &mut Vec<Vec<char>>) {
    for row in grid {
        row.reverse();
    }
}

#[cached(size = 128000)]
fn transpose(grid: Vec<Vec<char>>) -> Vec<Vec<char>> {
    let mut new_grid: Vec<Vec<char>> = Vec::new();
    for i in 0..grid.len() {
        let mut row: Vec<char> = Vec::new();
        for j in 0..grid.len() {
            row.push(grid[j][i]);
        }
        new_grid.push(row);
    }
    new_grid
}

fn get_load(grid: &Vec<Vec<char>>) -> usize {
    let mut total = 0;
    for (r, row) in grid.iter().enumerate() {
        let num_o = row.iter().filter(|&c| *c == 'O').count();
        total += num_o * (row.len() - r);
    }
    total
}

#[cached(size = 128000)]
fn cycle(grid: Vec<Vec<char>>) -> Vec<Vec<char>> {
    // roll North
    let mut grid = transpose(grid);
    reverse(&mut grid);
    grid = roll_right(grid);
    reverse(&mut grid);
    let mut grid = transpose(grid);

    //roll West
    reverse(&mut grid);
    grid = roll_right(grid);
    reverse(&mut grid);

    // roll South
    let mut grid = transpose(grid);
    grid = roll_right(grid);
    let mut grid = transpose(grid);

    // roll East
    grid = roll_right(grid);

    grid
}

fn solution(input: &str) -> usize {
    let mut grid: Vec<Vec<char>> = input.lines().map(|l| l.chars().collect()).collect();

    let num_cycles = 1000000000;
    let mut seen_states = HashMap::new();
    let mut loop_found = false;

    for i in 0..num_cycles {
        if loop_found {
            break;
        }

        let grid_str = grid_to_string(&grid);

        if let Some(&first_occurrence) = seen_states.get(&grid_str) {
            let loop_length = i - first_occurrence;
            let remaining_cycles = (num_cycles - i) % loop_length;
            for _ in 0..remaining_cycles {
                grid = cycle(grid.clone());
            }
            loop_found = true;
        } else {
            seen_states.insert(grid_str, i);
            grid = cycle(grid.clone());
        }
    }

    get_load(&grid)
}

fn grid_to_string(grid: &[Vec<char>]) -> String {
    grid.iter()
        .map(|row| row.iter().collect::<String>())
        .collect::<Vec<String>>()
        .join("\n")
}
