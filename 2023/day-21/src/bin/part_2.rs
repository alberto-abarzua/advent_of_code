use std::collections::{HashSet, VecDeque};

fn main() {
    println!("Solution:\n{:?}", solution(include_str!("../input.txt")));
}
fn solution(input: &str) -> i64 {
    let grid: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();
    let (sr, sc) = find_start(&grid).expect("Start position 'S' not found");

    let size = grid.len();
    let steps = 26501365;

    assert_eq!(sr, size / 2);
    assert_eq!(sc, size / 2);
    assert_eq!(steps % size, size / 2);

    let grid_width = steps / size - 1;
    let odd = (grid_width / 2 * 2 + 1).pow(2);
    let even = ((grid_width + 1) / 2 * 2).pow(2);

    let odd_points = fill(&grid, sr, sc, size * 2 + 1);
    let even_points = fill(&grid, sr, sc, size * 2);

    let corner_t = fill(&grid, size - 1, sc, size - 1);
    let corner_r = fill(&grid, sr, 0, size - 1);
    let corner_b = fill(&grid, 0, sc, size - 1);
    let corner_l = fill(&grid, sr, size - 1, size - 1);

    let small_tr = fill(&grid, size - 1, 0, size / 2 - 1);
    let small_tl = fill(&grid, size - 1, size - 1, size / 2 - 1);
    let small_br = fill(&grid, 0, 0, size / 2 - 1);
    let small_bl = fill(&grid, 0, size - 1, size / 2 - 1);

    let large_tr = fill(&grid, size - 1, 0, size * 3 / 2 - 1);
    let large_tl = fill(&grid, size - 1, size - 1, size * 3 / 2 - 1);
    let large_br = fill(&grid, 0, 0, size * 3 / 2 - 1);
    let large_bl = fill(&grid, 0, size - 1, size * 3 / 2 - 1);

    let result = odd * odd_points
        + even * even_points
        + corner_t
        + corner_r
        + corner_b
        + corner_l
        + (grid_width + 1) * (small_tr + small_tl + small_br + small_bl)
        + grid_width * (large_tr + large_tl + large_br + large_bl);

    result as i64
}

fn find_start(grid: &[Vec<char>]) -> Option<(usize, usize)> {
    grid.iter().enumerate().find_map(|(r, row)| {
        row.iter()
            .enumerate()
            .find_map(|(c, &ch)| if ch == 'S' { Some((r, c)) } else { None })
    })
}

fn fill(grid: &[Vec<char>], sr: usize, sc: usize, ss: usize) -> usize {
    let mut ans = HashSet::new();
    let mut seen = HashSet::new();
    seen.insert((sr, sc));
    let mut queue = VecDeque::new();
    queue.push_back((sr, sc, ss));

    while let Some((r, c, s)) = queue.pop_front() {
        if s % 2 == 0 {
            ans.insert((r, c));
        }
        if s == 0 {
            continue;
        }

        for (dr, dc) in [(1, 0), (-1, 0), (0, 1), (0, -1)] {
            let (nr, nc) = (r as isize + dr, c as isize + dc);
            if nr < 0 || nr >= grid.len() as isize || nc < 0 || nc >= grid[0].len() as isize {
                continue;
            }
            let next_pos = (nr as usize, nc as usize);
            if grid[nr as usize][nc as usize] != '#' && !seen.contains(&next_pos) {
                seen.insert(next_pos);
                queue.push_back((next_pos.0, next_pos.1, s - 1));
            }
        }
    }

    ans.len()
}
