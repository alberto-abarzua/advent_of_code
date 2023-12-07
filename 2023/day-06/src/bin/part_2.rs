fn main() {
    let input = include_str!("../input_1.txt");
    let output = solution(input);
    println!("Output is: \n {output}");
}

fn is_winner(ref_time: i64, ref_dist: i64, time_holding: i64) -> bool {
    let time_remaining = ref_time - time_holding;
    let dist_achieved = time_remaining * time_holding;

    dist_achieved > ref_dist
}

fn solution(input: &str) -> i32 {
    let values: Vec<i64> = input
        .lines()
        .map(|line| {
            line.split_once(":")
                .unwrap()
                .1
                .replace(" ", "")
                .parse()
                .unwrap()
        })
        .collect();

    let time = values[0];
    let dist = values[1];

    let mut total = 0;
    for i in 0..time {
        if is_winner(time, dist, i) {
            total += 1;
        }
    }
    total
}
