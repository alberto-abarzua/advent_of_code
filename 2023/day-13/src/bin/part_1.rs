fn main() {
    println!("Solution is:\n{:?}", solution(include_str!("../input.txt")));
}

fn is_col_of_reflection(grid: &Vec<Vec<char>>, col: usize) -> bool {
    let num_cols = grid[0].len();

    if col == 0 || col == num_cols {
        return false;
    }

    let cols_to_the_right = num_cols - col;
    let cols_to_the_left = col;

    let num_to_check = std::cmp::min(cols_to_the_left, cols_to_the_right);
    if num_to_check == 0 {
        return false;
    }

    let left = col - 1;
    let right = col;
    for num in 0..num_to_check {
        let right_col = grid
            .iter()
            .map(|row| row[right + num])
            .collect::<Vec<char>>();
        let left_col = grid
            .iter()
            .map(|row| row[left - num])
            .collect::<Vec<char>>();
        if right_col != left_col {
            return false;
        }
    }

    return true;
}

fn is_row_of_reflection(grid: &Vec<Vec<char>>, row: usize) -> bool {
    let num_rows = grid.len();

    if row == 0 || row == num_rows {
        return false;
    }

    let rows_below = num_rows - row;
    let rows_above = row;

    let num_to_check = std::cmp::min(rows_below, rows_above);
    if num_to_check == 0 {
        return false;
    }

    let above = row - 1;
    let below = row;
    for num in 0..num_to_check {
        let below_row = &grid[below + num];
        let above_row = &grid[above - num];
        if below_row != above_row {
            return false;
        }
    }

    return true;
}

fn solution(input: &str) -> i32 {
    let mut total = 0;
    let patterns = input.split("\n\n").collect::<Vec<&str>>();
    for pattern in patterns {
        let mut found = false;
        let grid = pattern
            .lines()
            .map(|line| line.chars().collect::<Vec<char>>())
            .collect::<Vec<Vec<char>>>();

        let num_rows = grid.len();
        let num_cols = grid[0].len();

        for row in 0..num_rows {
            if is_row_of_reflection(&grid, row) {
                total += row * 100;
                found = true;
                break;
            }
        }

        if found {
            continue;
        }
        for col in 0..num_cols {
            if is_col_of_reflection(&grid, col) {
                total += col;
                found = true;
                break;
            }
        }
        // if found is stil false panic
        if !found {
            panic!("No reflection found");
        }
    }
    total as i32
}
