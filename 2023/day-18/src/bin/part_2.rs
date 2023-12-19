fn main() {
    println!("Solution:\n{:?}", solution(include_str!("../input.txt")))
}

fn solution(input: &str) -> i64 {
    let values: Vec<(char, usize)> = input
        .lines()
        .map(|line| {
            let (_start, info) = line.split_once("(#").unwrap();
            let hex_info = info.strip_suffix(")").unwrap();
            let hex_number = hex_info.get(0..(hex_info.len() - 1)).unwrap();
            let hex_dir = hex_info.chars().last().unwrap();
            let num = usize::from_str_radix(hex_number, 16).unwrap();
            let char = match hex_dir {
                '0' => 'R',
                '1' => 'D',
                '2' => 'L',
                '3' => 'U',
                _ => panic!("Unknown direction"),
            };

            (char, num)
        })
        .collect();

    let mut cur_point = (0, 0);
    let mut vertices: Vec<(i32, i32)> = vec![];
    let mut num_border_points = 0;
    for (dir, num) in values.iter() {
        match dir {
            'R' => {
                cur_point.0 += *num as i32;
            }
            'L' => {
                cur_point.0 -= *num as i32;
            }
            'D' => {
                cur_point.1 += *num as i32;
            }
            'U' => {
                cur_point.1 -= *num as i32;
            }
            _ => panic!("Unknown direction"),
        }
        num_border_points += *num;
        vertices.push(cur_point.clone());
    }

    let a = get_area(&vertices) as i64;
    let b = num_border_points as i64;

    a + b / 2 + 1
}

fn get_area(vertices: &Vec<(i32, i32)>) -> f64 {
    let mut area = 0.0;

    for i in 0..vertices.len() {
        let (x1, y1) = vertices[i];
        let (x2, y2) = if i + 1 < vertices.len() {
            vertices[i + 1]
        } else {
            vertices[0] // If it's the last vertex, loop back to the first
        };

        area += (x1 as f64 * y2 as f64) - (y1 as f64 * x2 as f64);
    }

    area.abs() / 2.0
}
