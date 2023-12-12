use cached::proc_macro::cached;
fn main() {
    println!("Solution is:\n{:?}", solution(include_str!("../input.txt")));
}
#[cached(size = 128000)]
fn get_arrangements(line: Vec<char>) -> Vec<i32> {
    let mut res: Vec<i32> = Vec::new();

    let mut broken_count = 0;

    for ch in line.iter() {
        if *ch == '#' {
            broken_count += 1;
        } else if *ch == '.' {
            if broken_count != 0 {
                res.push(broken_count);
            }
            broken_count = 0;
        }
    }
    if broken_count != 0 {
        res.push(broken_count);
    }
    res
}
#[cached(size = 128000)]
fn get_num_possible(line: Vec<char>, expected: Vec<i32>) -> i64 {
    if line.is_empty() {
        return if expected.is_empty() { 1 } else { 0 };
    }

    if expected.is_empty() {
        return if line.contains(&'#') { 0 } else { 1 };
    }

    let mut result = 0;
    if line[0] == '.' || line[0] == '?' {
        result += get_num_possible(line[1..].to_vec(), expected.clone());
    }

    if line[0] == '#' || line[0] == '?' {
        if expected[0] as usize <= line.len()
            && !line[..expected[0] as usize].contains(&'.')
            && (expected[0] as usize == line.len() || line[expected[0] as usize] != '#')
        {
            let new_line = if expected[0] as usize == line.len() {
                vec![]
            } else {
                line[(expected[0] as usize + 1)..].to_vec()
            };
            let new_expected = if expected.len() > 1 {
                expected[1..].to_vec()
            } else {
                vec![]
            };
            result += get_num_possible(new_line, new_expected);
        }
    }

    result
}
fn solution(input: &str) -> i64 {
    let mut lines: Vec<(Vec<char>, Vec<i32>)> = input
        .lines()
        .map(|line| {
            let (chars, arr) = line.split_once(" ").unwrap();
            let chars: Vec<char> = chars.chars().collect();
            let arr = arr.split(",").map(|s| s.parse::<i32>().unwrap()).collect();
            (chars, arr)
        })
        .collect();

    let mut total: i64 = 0;
    for (line, expected_arrangement) in lines.iter_mut() {
        let mut new_line: Vec<char> = Vec::new();
        let mut new_expected_arrangement: Vec<i32> = Vec::new();
        for i in 0..5 {
            new_line.extend(line.iter().clone());
            if i != 4 {
                new_line.push('?');
            }
            new_expected_arrangement.extend(expected_arrangement.iter());
        }

        let cur_num = get_num_possible(new_line, new_expected_arrangement);
        total += cur_num;
    }

    total
}
