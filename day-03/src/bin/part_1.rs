fn main() {
    let input = include_str!("../input_1.txt");
    let output = part1(input);
    dbg!(output);
    dbg!(is_symbol('$'));
}

fn is_symbol(ch: char) -> bool {
    return !ch.is_numeric() && ch != '.';
}

fn check_adjacent_symbol(lines: &Vec<&str>, index: usize, start: usize, end: usize) -> bool {
    match lines.get(index + 1) {
        Some(line) => {
            let mut upper_left: char = ' ';
            if start != 0 {
                upper_left = match line.chars().nth(start - 1) {
                    Some(char) => char,
                    _ => '.',
                };
            }
            let upper_right = match line.chars().nth(end + 1) {
                Some(char) => char,
                _ => '.',
            };
            if is_symbol(upper_left) || is_symbol(upper_right) {
                return true;
            }
            for i in start..end {
                let cur_char = line.chars().nth(i).unwrap();
                if is_symbol(cur_char) {
                    return true;
                }
            }
        }
        None => {}
    }

    if index != 0 {
        match lines.get(index - 1) {
            Some(line) => {
                let lower_left = match line.chars().nth(start - 1) {
                    Some(char) => char,
                    _ => '.',
                };
                let lower_right = match line.chars().nth(end + 1) {
                    Some(char) => char,
                    _ => '.',
                };
                if is_symbol(lower_left) || is_symbol(lower_right) {
                    return true;
                }
                dbg!(lower_right);
                dbg!(lower_left);

                for i in start..end {
                    let cur_char = line.chars().nth(i).unwrap();
                    if is_symbol(cur_char) {
                        return true;
                    }
                }
            }
            None => {}
        }
    }

    let line = lines.get(index).unwrap();

    let left = match line.chars().nth(start) {
        Some(char) => char,
        _ => '.',
    };

    let right = match line.chars().nth(end + 1) {
        Some(char) => char,
        _ => '.',
    };
    if is_symbol(right) || is_symbol(left) {
        return true;
    }

    return false;
}

fn part1(input: &str) -> String {
    let lines: Vec<&str> = input.lines().collect();
    let mut total: i32 = 0;
    for (index, line) in lines.iter().enumerate() {
        let mut start = 0;
        let mut end = 0;
        while start < line.len() {
            if start >= line.len() {
                break;
            }
            let char = line.chars().nth(start).unwrap();
            if char.is_numeric() {
                for (subidx, subchar) in line.chars().enumerate().skip(start) {
                    if !subchar.is_numeric() {
                        end = subidx - 1;
                        break;
                    }
                }

                let part_number: i32 = line.get(start..end + 1).unwrap().parse().unwrap();

                if check_adjacent_symbol(&lines, index, start, end) {
                    total += part_number;
                    dbg!(part_number);
                } else {
                    // dbg!("------------------");
                    // dbg!(line);
                    // dbg!(part_number);
                    // dbg!("------------------");
                }

                start = end + 1;
                end = start + 1;
            } else {
                start += 1;
                end = start;
            }
        }
    }

    return total.to_string();
}
