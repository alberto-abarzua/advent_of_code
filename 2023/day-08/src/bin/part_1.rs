use std::collections::HashMap;

fn main() {
    let input = include_str!("../input.txt");
    let output = solution(input);
    println!("Solution:\n{output}");
}

fn solution(input: &str) -> i32 {
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

    let mut cur = "AAA";
    let mut cur_instruction_idx = 0;
    let mut num_steps = 0;

    while cur != "ZZZ" {
        let instruction = instructions.chars().nth(cur_instruction_idx).unwrap();
        let val = map.get(cur);

        let (left, right) = val.unwrap();
        if instruction == 'R' {
            cur = right;
        } else {
            cur = left;
        }
        num_steps += 1;

        cur_instruction_idx = (cur_instruction_idx + 1) % instructions.len();
    }

    num_steps
}
