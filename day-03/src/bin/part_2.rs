use std::collections::HashMap;
fn main() {
    let input = include_str!("../input_1.txt");
    let output = part2(input);
    println!("Ouput is:\n{}", output);
}

fn is_symbol(ch: char) -> bool {
    return !ch.is_numeric() && ch != '.';
}

fn check_adjacent_symbol(
    lines: &Vec<&str>,
    gears: &mut HashMap<String, Vec<i32>>,
    index: usize,
    start: usize,
    end: usize,
    part_num: i32,
) -> bool {
    for i in -1..=1 {
        if index as i32 + i == -1 || index as i32 + i == lines.len() as i32 {
            continue;
        }
        let cur_idx: usize = (index as i32 + i) as usize;
        let cur_line: &str = lines.get(cur_idx).expect("Line out of bounds");

        let init = std::cmp::max(start as i32 - 1, 0) as usize;
        let finish = std::cmp::min(end as i32 + 1, lines.len() as i32 - 1) as usize;

        for i in init..=finish {
            let char = cur_line.chars().nth(i).expect("Char out of bounds");
            let key = format!("{cur_idx}{i}");
            if char == '*' {
                let arr = gears.entry(key).or_insert(Vec::new());
                arr.push(part_num);
            }
            if is_symbol(char) {
                return true;
            }
        }
    }
    return false;
}

fn part2(input: &str) -> String {
    let lines: Vec<&str> = input.lines().collect();
    let mut gears: HashMap<String, Vec<i32>> = HashMap::new();
    let mut total: i32 = 0;
    for (index, line) in lines.iter().enumerate() {
        let mut i = 0;
        while i < line.len() {
            let cur_char = line.chars().nth(i).unwrap();
            if cur_char.is_numeric() {
                let start = i;
                let mut end = i;
                while end < line.len() {
                    let var_char = line.chars().nth(end).unwrap();
                    if !var_char.is_numeric() {
                        break;
                    }

                    end += 1;
                }
                end -= 1;
                i = end + 1;
                let part_number: i32 = line.get(start..=end).unwrap().parse().unwrap();
                check_adjacent_symbol(&lines, &mut gears, index, start, end, part_number);
            } else {
                i += 1;
            }
        }
    }

    for (key, value) in gears {
        dbg!(key, &value);
        if value.len() == 2 {
            total += value.iter().fold(1, |acc, &x| acc * x);
        }
    }

    return total.to_string();
}
