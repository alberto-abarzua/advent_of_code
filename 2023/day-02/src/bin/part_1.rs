use std::collections::HashMap;
fn main() {
    let input = include_str!("../input_1.txt");
    let output = part1(input);
    dbg!(output);
}

fn part1(input: &str) -> String {
    let mut total: i32 = 0;
    for line in input.lines() {
        let game_id: i32;
        let mut possible: bool = true;
        let content: &str;
        let mut map_amounts: HashMap<&str, i32> = HashMap::new();
        map_amounts.insert("red", 12);
        map_amounts.insert("green", 13);
        map_amounts.insert("blue", 14);

        match line.split_once(": ") {
            Some((g, c)) => {
                if let Some((_, id)) = g.split_once(" ") {
                    game_id = id.parse().unwrap();
                } else {
                    game_id = 0;
                }
                content = c;
            }
            None => {
                game_id = 0;
                content = "no content";
            }
        }

        for round in content.split("; ") {
            for pair in round.split(", ") {
                if let Some((num, key)) = pair.split_once(" ") {
                    let amount = map_amounts.get(key).unwrap();
                    let num_int: i32 = num.parse().unwrap();
                    if num_int > *amount {
                        possible = false;
                    }
                }
            }
        }
        if possible {
            total += game_id;
        }
    }

    return total.to_string();
}
