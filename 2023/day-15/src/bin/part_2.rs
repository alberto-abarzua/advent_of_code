fn main() {
    println!("Solution:\n{:?}", solution(include_str!("../input.txt")));
}

fn hash(value: &str) -> i32 {
    let mut current_value = 0;
    for c in value.chars() {
        let ascii = c as u8;
        current_value += ascii as i32;
        current_value *= 17;
        current_value %= 256;
    }
    current_value
}

fn solution(input: &str) -> i32 {
    let values = input.strip_suffix("\n").unwrap().split(",");
    let mut boxes: Vec<Vec<(&str, usize)>> = vec![Vec::new(); 256];

    let mut total = 0;
    for value in values {
        // if - in the string
        if value.contains("-") {
            let label = value.strip_suffix("-").unwrap();
            boxes[hash(label) as usize].retain(|(l, _)| l != &label);

        } else if value.contains("=") {
            let (label, number) = value.split_once("=").unwrap();
            let number = number.parse::<usize>().unwrap();
            if let Some((_, fc)) = boxes[hash(label) as usize]
                .iter_mut()
                .find(|(l, _)| l == &label)
            {
                *fc = number;
            } else {
                boxes[hash(label) as usize].push((label, number));
            }

        }else{
            panic!("Invalid input") ;
        }
    }


    for (index, arr) in boxes.iter().enumerate() {
        if arr.is_empty() {
            continue;
        }
        for (sub_idx, (_label, fc)) in arr.iter().enumerate() {
            let mut sub_total = 1;
            sub_total *= index + 1;
            sub_total *= sub_idx + 1;
            sub_total *= *fc as usize;
            total += sub_total as i32;
        }
    }

    total
}
