fn main() {
    let input = include_str!("../input_1.txt");
    let output = part1(input);
    dbg!(output);
}

fn part1(input: &str) -> String {
    let mut total: i32 = 0;

    for line in input.lines() {
        let mut first_num: char = '0';
        let mut second_num: char = '0';

        for ch in line.chars() {
            if ch.is_numeric() {
                first_num = ch;
                break;
            }
        }

        let chars: Vec<char> = line.chars().collect();

        for ch in chars.iter().rev() {
            if ch.is_numeric() {
                second_num = *ch;
                break;
            }
        }
        let num = format!("{}{}", first_num, second_num);
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

