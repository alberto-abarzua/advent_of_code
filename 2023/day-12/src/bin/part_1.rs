fn main() {
    println!(
        "Solution is:\n{:?}",
        solution(include_str!("../input.txt"))
    );
}

fn get_arrangements(line: &Vec<char>) -> Vec<i32> {
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

fn get_num_possible(line: &mut Vec<char>, expected: &Vec<i32>, idx: i32) -> i32 {
    if idx == line.len() as i32 {
        let arrangements = get_arrangements(line);
        // println!("{} {:?} {:?}", idx, line,arrangements);
        return if arrangements == *expected {
            1
        } else {
            0
        };
    }

    let mut res =0;
    if line[idx as usize] == '?' {
        line[idx as usize] = '#';
        res += get_num_possible(line, expected, idx + 1);
        line[idx as usize] = '.';
        res += get_num_possible(line, expected, idx + 1);
        line[idx as usize] = '?';
        return res;
    }
    return get_num_possible(line, expected, idx + 1);
}

fn solution(input: &str) -> i32 {
    let mut lines: Vec<(Vec<char>, Vec<i32>)> = input
        .lines()
        .map(|line| {
            let (chars, arr) = line.split_once(" ").unwrap();
            let chars: Vec<char> = chars.chars().collect();
            let arr = arr.split(",").map(|s| s.parse::<i32>().unwrap()).collect();
            (chars, arr)
        })
        .collect();

    let mut total = 0;
    for (line, expected_arrangement) in lines.iter_mut() {
        // println!("\n\nDoing line: {:?} {:?}", line, expected_arrangement);
        let cur_num = get_num_possible(line, expected_arrangement, 0);
        // println!("Num possible: {}\n", cur_num);
        total += cur_num;
    }

    total
}
