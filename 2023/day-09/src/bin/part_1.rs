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

        let mut last_vals: Vec<i32> = Vec::new();
        last_vals.push(*cur_values.last().unwrap());

        while !cur_values.iter().all(|&x| x == 0) {
            let mut temp: Vec<i32> = Vec::new();

            for i in 0..(cur_values.len() - 1) {
                temp.push(cur_values[i + 1] - cur_values[i]);
            }
            cur_values = temp.clone();
            last_vals.push(*cur_values.last().unwrap());
        }

        total += last_vals.iter().fold(0, |acc, &num| acc + num);
    }

    total
}
