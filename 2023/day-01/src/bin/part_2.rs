use std::collections::HashMap;

fn main() {
    let input = include_str!("../input_1.txt");
    let output = part2(input);
    dbg!(output);
}

fn part2(input: &str) -> String {
    let mut total: i32 = 0;

    let mut map = HashMap::new();

    map.insert("one", "o1ne");
    map.insert("two", "tw2o");
    map.insert("three", "thr3ee");
    map.insert("four", "fou4r");
    map.insert("five", "fi5ve");
    map.insert("six", "si6x");
    map.insert("seven", "sev7en");
    map.insert("eight", "eigh8t");
    map.insert("nine", "nin9e");

    for line in input.lines() {
        let mut new_line = String::from(line);
        for (key, value) in map.iter() {
            new_line = new_line.replace(key, value);
        }

        let mut first_num: char = '0';
        let mut second_num: char = '0';

        for ch in new_line.chars() {
            if ch.is_numeric() {
                first_num = ch;
                break;
            }
        }

        let chars: Vec<char> = new_line.chars().collect();

        for ch in chars.iter().rev() {
            if ch.is_numeric() {
                second_num = *ch;
                break;
            }
        }
        let num = format!("{}{}", first_num, second_num);
        dbg!(new_line);
        dbg!(&num);
        let int_result = num.parse::<i32>();
        match int_result {
            Ok(num) => {
                total += num;
            }
            Err(e) => println!("Error parsing as int {}", e),
        }
    }
    return total.to_string();
}
