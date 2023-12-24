use std::collections::{HashMap, HashSet};
fn main() {
    println!("Solution:\n{:?}", solution(include_str!("../input.txt")));
}

fn solution(input: &str) -> i64 {
    let grid: Vec<Vec<char>> = input.lines().map(|l| l.chars().collect()).collect();
    let start: (usize, usize) = (0, grid[0].iter().position(|&c| c == '.').unwrap());
    let end: (usize, usize) = (
        grid.len() - 1,
        grid[grid.len() - 1].iter().position(|&c| c == '.').unwrap(),
    );

    let mut points: Vec<(usize, usize)> = vec![start, end];

    for r in 0..grid.len() {
        for c in 0..grid[r].len() {
            if grid[r][c] == '#' {
                continue;
            }
            let dirs = [(0, 1), (0, -1), (1, 0), (-1, 0)];
            let mut num_neighbors = 0;
            for (dr, dc) in dirs {
                let (nr, nc) = (r as i64 + dr, c as i64 + dc);
                if nr >= 0
                    && nr < grid.len() as i64
                    && nc >= 0
                    && nc < grid[r].len() as i64
                    && grid[nr as usize][nc as usize] != '#'
                {
                    num_neighbors += 1;
                }
            }
            if num_neighbors >= 3 {
                points.push((r, c));
            }
        }
    }


    let mut graph: HashMap<(usize, usize), HashMap<(usize, usize), usize>> = HashMap::new();

    for (sr, sc) in points.iter() {
        let (sr, sc) = (*sr, *sc);
        let mut stack = vec![(0, sr, sc)];
        let mut visited: HashSet<(usize, usize)> = HashSet::new();

        while stack.len() > 0 {
            let (n, r, c) = stack.pop().unwrap();

            if n != 0 && points.contains(&(r, c)) {
                graph
                    .entry((sr, sc))
                    .or_insert(HashMap::new())
                    .insert((r, c), n);
                continue;
            }

            let dirs = [(0, 1), (0, -1), (1, 0), (-1, 0)];

            for (dr, dc) in dirs.iter() {
                let (nr, nc) = (r as i64 + dr, c as i64 + dc);

                if nr >= 0 && nr < grid.len() as i64 && nc >= 0 && nc < grid[r].len() as i64 {
                    let (nr, nc) = (nr as usize, nc as usize);

                    if grid[nr][nc] == '#' || visited.contains(&(nr, nc)) {
                        continue;
                    }
                    stack.push((n + 1, nr, nc));
                    visited.insert((nr, nc));
                }
            }
        }
    }

    dbg!(&graph.len());
    longest_path(&graph, start, end, &mut HashSet::new())
}

fn longest_path(
    graph: &HashMap<(usize, usize), HashMap<(usize, usize), usize>>,
    start: (usize, usize),
    end: (usize, usize),
    seen: &mut HashSet<(usize, usize)>,
) -> i64 {
    if start == end {
        return 0;
    }
    let mut max = i64::MIN;

    seen.insert(start);

    let neighbors = &graph[&start];
    for nx in neighbors.keys() {
        if seen.contains(nx) {
            continue;
        }

        let path_len: i64 = neighbors[nx] as i64 + longest_path(graph, *nx, end, seen);
        max = std::cmp::max(max, path_len);
    }
    seen.remove(&start);

    max
}
