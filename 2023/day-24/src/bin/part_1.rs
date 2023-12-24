use std::{i128, i64};

fn main() {
    println!("Solution:\n{:?}", solution(include_str!("../input.txt")));
}

#[derive(Debug, Clone)]
struct Stone {
    pos: (i64, i64, i64),
    vel: (i64, i64, i64),
    a: i128,
    b: i128,
    c: i128,
}

impl Stone {
    fn new(pos: (i64, i64, i64), vel: (i64, i64, i64)) -> Stone {
        Stone {
            pos,
            vel,
            a: vel.1 as i128,
            b: -vel.0 as i128,
            c: vel.1 as i128 * pos.0 as i128 - vel.0 as i128 * pos.1 as i128,
        }
    }

    fn in_bounds(&self, x: i64, y: i64) -> bool {
        let bounds = 200000000000000..=400000000000000;
        bounds.contains(&x) && bounds.contains(&y)
    }

    fn find_intersect_xy(&self, other: &Stone) -> Option<(i64, i64)> {
        if self.a * other.b == self.b * other.a {
            return None;
        }

        let det = self.a * other.b - self.b * other.a;
        let x = ((self.c * other.b - other.c * self.b) / det) as i64;
        let y = ((other.c * self.a - self.c * other.a) / det) as i64;

        if self.in_bounds(x, y) {
            if (x - self.pos.0) / self.vel.0 < 0 || (x - other.pos.0) / other.vel.0 < 0 {
                return None;
            }
            Some((x, y))
        } else {
            None
        }
    }
}

fn solution(input: &str) -> i64 {
    let stones: Vec<Stone> = input
        .lines()
        .map(|l| {
            let l = l.replace("  ", " ");
            let mut values = l.split(" @ ").map(|s| {
                s.split(", ")
                    .map(|s| s.parse::<i64>().unwrap())
                    .collect::<Vec<i64>>()
            });
            let pos = values.next().unwrap();
            let speed = values.next().unwrap();
            Stone::new((pos[0], pos[1], pos[2]), (speed[0], speed[1], speed[2]))
        })
        .collect();

    let mut intersect = 0;
    for i in 0..stones.len() {
        for j in i + 1..stones.len() {
            if stones[i].find_intersect_xy(&stones[j]).is_some() {
                intersect += 1;
            }
        }
    }
    intersect
}
