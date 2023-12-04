use std::collections::HashSet;

fn main() {
    let input = include_str!("../input_1.txt");
    let output = part1(input);
    println!("Ouput is:\n{}", output);
}

fn part1(input: &str) -> String {
    let mut total: i32 = 0;

    for line in input.lines() {
        let (_, content) = line.split_once(": ").unwrap();
        let (mut winner, mut actual) = content.split_once(" | ").unwrap();
        if winner.chars().nth(0).unwrap() == ' ' {
            winner = winner.get(1..).unwrap();
        }
        if actual.chars().nth(0).unwrap() == ' ' {
            actual = actual.get(1..).unwrap();
        }

        let winner: HashSet<i32> = winner
            .replace("  ", " ")
            .split(" ")
            .map(|num| num.parse().unwrap())
            .collect();
        let actual: Vec<i32> = actual
            .replace("  ", " ")
            .split(" ")
            .map(|num| num.parse().unwrap())
            .collect();
        let mut cur_res = 1;

        for num in actual.iter() {
            if winner.contains(num) {
                cur_res *= 2;
            }
        }

        total += cur_res/2;
    }

    return total.to_string();
}
