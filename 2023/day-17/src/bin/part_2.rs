use std::collections::HashMap;

use pathfinding::{
    directed::dijkstra::dijkstra,
    matrix::{directions, Matrix},
};

fn main() {
    println!("Solution:\n{:?}", solution(include_str!("../input.txt")));
}

fn solution(input: &str) -> usize {
    let grid = input
        .lines()
        .map(|l| l.bytes().map(|b| b - b'0'))
        .collect::<Matrix<u8>>();

    let d = dijkstra(
        &((0, 0), directions::E, 0), 
        |&(pos, cdir, cdist)| {
            let mut neigh = Vec::new();
            if cdist >= 4 {
                for d in vec![directions::N, directions::S, directions::E, directions::W] {
                    if let Some(np) = grid.move_in_direction(pos, d) {
                        let wt = *grid.get(np).unwrap() as usize;
                        if d != (cdir.0 * -1, cdir.1 * -1) && d != cdir {
                            neigh.push(((np, d, 1), wt));
                        } else if cdist < 10 && d == cdir {
                            neigh.push(((np, d, cdist + 1), wt));
                        }
                    }
                }
            } else {
                if let Some(np) = grid.move_in_direction(pos, cdir) {
                    let wt = *grid.get(np).unwrap() as usize;
                    neigh.push(((np, cdir, cdist + 1), wt));
                }
            }
            neigh
        },
        |&(pos, _, dist)| pos == (grid.rows - 1, grid.columns - 1) && dist >= 4,
    );

    let mut vis: HashMap<(usize, usize), (isize, isize)> = HashMap::new();

    for (pos, d, _) in d.clone().and_then(|(path, _dist)| Some(path)).unwrap() {
        vis.insert(pos, d);
    }
    d.unwrap().1
}
