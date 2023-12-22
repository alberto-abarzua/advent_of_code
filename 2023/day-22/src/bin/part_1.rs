use std::collections::HashMap;
fn main() {
    println!("Solution:\n{:?}", solution(include_str!("../input.txt")));
}

#[derive(Debug, Clone, Copy)]
struct Brick {
    start: (usize, usize, usize), //x,y,z
    end: (usize, usize, usize),
}

impl Brick {
    fn new(start: (usize, usize, usize), end: (usize, usize, usize)) -> Self {
        Brick { start, end }
    }
    fn lowest_z(&self) -> usize {
        self.start.2.min(self.end.2)
    }

    fn highest_z(&self) -> usize {
        self.start.2.max(self.end.2)
    }

    fn xy_overlap(&self, other: &Brick) -> bool {
        let x_overlap = self.start.0 <= other.end.0 && other.start.0 <= self.end.0;
        let y_overlap = self.start.1 <= other.end.1 && other.start.1 <= self.end.1;
        x_overlap && y_overlap
    }

    fn fall(&mut self, index: usize, bricks: &Vec<Brick>) {
        let mut max_z = 1;
        for (i, brick) in bricks.iter().enumerate() {
            if i == index {
                break;
            }
            if self.xy_overlap(brick) {
                max_z = max_z.max(brick.highest_z() + 1);
            }
        }
        let diff = self.lowest_z() - max_z;
        self.start.2 -= diff;
        self.end.2 -= diff;
    }

    fn bricks_one_below(&self, bricks: &Vec<Brick>) -> Vec<usize> {
        let mut result = Vec::new();
        for (i, brick) in bricks.iter().enumerate() {
            if self.xy_overlap(brick) && self.lowest_z() == brick.highest_z() + 1 {
                result.push(i);
            }
        }
        result
    }
}

fn parse_input(input: &str) -> Vec<Brick> {
    let mut bricks: Vec<_> = input
        .lines()
        .map(| line| {
            let line_slice = line
                .split("~")
                .map(|val| {
                    val.split(",")
                        .map(|val| val.parse::<usize>().unwrap())
                        .collect::<Vec<_>>()
                })
                .collect::<Vec<_>>();

            let (start, end) = match line_slice.as_slice() {
                [start, end] => (start, end),
                _ => panic!("Invalid input"),
            };
            Brick::new(
                (start[0], start[1], start[2]),
                (end[0], end[1], end[2]),
            )
        })
        .collect();

    bricks.sort_by_key(|brick| brick.lowest_z());
    bricks
}

fn solution(input: &str) -> usize {
    let mut bricks = parse_input(input);

    for i in 0..bricks.len() {
        let mut brick = bricks.remove(i);

        brick.fall(i, &bricks);

        bricks.insert(i, brick);
    }

    bricks.sort_by_key(|brick| brick.lowest_z());

    let mut bellow_map: HashMap<usize, Vec<usize>> = HashMap::new();

    for i in 0..bricks.len() {
        let brick = bricks.remove(i);

        bellow_map.insert(i, brick.bricks_one_below(&bricks));

        bricks.insert(i, brick);
    }

    let mut total = 0;

    for i in 0..bricks.len() {
        let mut can_be_removed = true;

        for (_key, value) in bellow_map.iter() {
            if value.contains(&i) && value.len() == 1 {
                can_be_removed = false;
                break;
            }
        }
        if can_be_removed {
            total += 1;
        }
    }

    total
}
