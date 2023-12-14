fn main() {
    println!("Solution:\n{:?}", solution(include_str!("../input.txt")));
}

fn transpose(grid: &Vec<Vec<char>>) -> Vec<Vec<char>> {
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

fn roll_right(grid: &mut Vec<Vec<char>>) {
    for row in grid {
        for idx in 0..row.len() {
            let i = row.len() - idx - 1;
            if row[i] == 'O' {
                // we need to move this up until the edge or if we find a #
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
}

fn reverse(grid: &mut Vec<Vec<char>>) {
    for row in grid {
        row.reverse();
    }
}

fn solution(input: &str) -> i32 {
    let grid: Vec<Vec<char>> = input.lines().map(|l| l.chars().collect()).collect();

    let mut transposed = transpose(&grid);
    reverse(&mut transposed);

    roll_right(&mut transposed);

    let mut total = 0;
    let transposed = transpose(&transposed);
    for (r, row) in transposed.iter().enumerate() {
        let num_o = row.iter().filter(|&c| *c == 'O').count();
        total += num_o * (r + 1);
    }

    total as i32
}
