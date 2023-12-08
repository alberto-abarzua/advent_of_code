use num_integer::Integer;
use std::{cmp::Ordering, collections::HashMap};

fn main() {
    let input = include_str!("../input.txt");
    let output = solution(input);
    println!("Solution:\n{output}");
}
#[derive(Debug)]
struct Path {
    cur: String,
    starting_point: String,
    first_z: String,
    step_to_z: i32,
    loop_lenght: i32,
}

fn lcm_of_array(numbers: &[i64]) -> i64 {
    numbers.iter().copied().fold(1, |lcm, num| lcm.lcm(&num))
}

fn solution(input: &str) -> i64 {
    let (instructions, content) = input.split_once("\n\n").unwrap();
    let mut map: HashMap<&str, (&str, &str)> = HashMap::new();

    for line in content.lines() {
        let (key, values) = line.split_once(" = ").unwrap();
        let (left, right) = values
            .get(1..(values.len() - 1))
            .unwrap()
            .split_once(", ")
            .unwrap();

        map.insert(key, (left, right));
    }

    let mut starting: Vec<Path> = Vec::new();
    for key in map.keys() {
        if key.chars().last().unwrap() == 'A' {
            let path = Path {
                cur: key.to_string(),
                starting_point: key.to_string(),
                first_z: "".to_string(),
                step_to_z: 0,
                loop_lenght: 0,
            };
            starting.push(path);
        }
    }

    let mut cur_instruction_idx = 0;
    let mut num_steps = 0;

    while starting.iter().any(|path| path.loop_lenght == 0) {
        let instruction = instructions.chars().nth(cur_instruction_idx).unwrap();
        for cur in starting.iter_mut() {
            let (left, right) = map.get(cur.cur.as_str()).unwrap();

            if instruction == 'R' {
                cur.cur = right.to_string();
            } else {
                cur.cur = left.to_string();
            }
            if cur.cur.ends_with('Z') {
                if cur.first_z.len() == 0 {
                    cur.first_z = cur.cur.clone();
                    cur.step_to_z = num_steps;
                } else {
                    if cur.loop_lenght == 0 {
                        cur.loop_lenght = num_steps - cur.step_to_z;
                    }
                }
            }
            if num_steps != 0 && cur.cur.cmp(&cur.starting_point) == Ordering::Equal {}
        }
        cur_instruction_idx = (cur_instruction_idx + 1) % instructions.len();
        num_steps += 1;
    }
    let cycles: Vec<i64> = starting
        .iter()
        .map(|path| (path.loop_lenght as i64))
        .collect();

    lcm_of_array(&cycles)
}
