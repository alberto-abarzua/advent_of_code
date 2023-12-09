fn main() {
    let input = include_str!("../input.txt");
    let output = solution(input);
    println!("Solution:\n{output}");
}

fn solution(input: &str) -> i32 {
    let mut total = 0;

    for line in input.lines() {
        let values = line
            .split_whitespace()
            .map(|x| x.parse::<i32>().unwrap())
            .collect::<Vec<i32>>();

        let mut cur_values: Vec<i32> = values.clone();

        let mut first_values: Vec<i32> = Vec::new();

        first_values.push(*cur_values.first().unwrap());

        while !cur_values.iter().all(|&x| x == 0) {
            let mut temp: Vec<i32> = Vec::new();
            for i in 0..(cur_values.len() - 1) {
                temp.push(cur_values[i + 1] - cur_values[i]);
            }

            cur_values = temp.clone();

            first_values.push(*temp.first().unwrap());
        }

        let cur = first_values.iter().rev().fold(0, |acc, &num| num - acc);

        total += cur;
    }

    total
}
