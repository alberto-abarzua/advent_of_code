fn main() {
    println!("Solution:\n{:?}", solution(include_str!("../input.txt")));
}

fn hash(value: &str) ->i32{
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
    let mut total =0;
    for value in values{
        total += hash(value);
    }
    total
}
