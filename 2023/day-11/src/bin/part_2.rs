fn main() {
    println!("Solution is:\n{:?}", solution(include_str!("../input.txt")));
}

fn solution(input: &str) -> i64 {
    let mut grid: Vec<Vec<char>> = Vec::new();
    for line in input.lines() {
        grid.push(line.chars().collect());
    }
    let empty_rows: Vec<usize> = grid
        .iter()
        .enumerate()
        .filter(|(_, row)| row.iter().all(|&ch| ch == '.'))
        .map(|(i, _)| i)
        .collect();

    let empty_cols: Vec<usize> = (0..grid[0].len())
        .filter(|&c| grid.iter().all(|row| row[c] == '.'))
        .collect();

    let points: Vec<(usize, usize)> = grid
        .iter()
        .enumerate()
        .flat_map(|(r, row)| {
            row.iter()
                .enumerate()
                .filter(|&(_, ch)| *ch == '#')
                .map(move |(c, _)| (r, c))
        })
        .collect();

    let mut total = 0;
    let scale:i64 = 1000000;

    for (i, (r1, c1)) in points.iter().enumerate() {
        for (r2, c2) in points.iter().take(i) {
            for r in std::cmp::min(*r1, *r2)..std::cmp::max(*r1, *r2) {
                total += if empty_rows.contains(&r) { scale } else { 1 };
            }
            for c in std::cmp::min(*c1, *c2)..std::cmp::max(*c1, *c2) {
                total += if empty_cols.contains(&c) { scale } else { 1 };
            }
        }
    }

    total
}
