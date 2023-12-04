use std::collections::HashMap;
fn main() {
    let input = include_str!("../input_1.txt");
    let output = part1(input);
    dbg!(output);
}

fn part1(input: &str) -> String {
    let mut total: i32 = 0;
    for line in input.lines() {
        let content: &str;
        let mut map_amounts: HashMap<&str, i32> = HashMap::new();
        map_amounts.insert("red", 0);
        map_amounts.insert("green", 0);
        map_amounts.insert("blue", 0);

        match line.split_once(": ") {
            Some((_, c)) => {
                content = c;
            }
            None => {
                content = "no content";
            }
        }

        for round in content.split("; ") {
            for pair in round.split(", ") {
                if let Some((num, key)) = pair.split_once(" ") {
                    let amount = map_amounts.get(key).unwrap();
                    let num_int: i32 = num.parse().unwrap();
                    if num_int > *amount {
                        map_amounts.insert(key, num_int);
                    }
                }
            }
        }

        let mut result: i32 = 1;
        for val in map_amounts.values() {
            result *= val;
        }
        total += result;
    }

    return total.to_string();
}
