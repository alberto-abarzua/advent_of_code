fn main() {
    let input = include_str!("../input_1.txt");
    let output = solution(input);
    println!("Output is: \n {output}");
}

fn is_winner(ref_time: i32, ref_dist: i32, time_holding: i32) -> bool {
    let time_remaining = ref_time - time_holding;
    let dist_achieved = time_remaining * time_holding;

    dist_achieved > ref_dist
}

fn solution(input: &str) -> i32 {
    let values: Vec<Vec<i32>> = input
        .lines()
        .map(|line| {
            line.split_whitespace()
                .skip(1)
                .map(|num| num.parse::<i32>().unwrap())
                .collect()
        })
        .collect();

    let times = values[0].clone();
    let dists = values[1].clone();
    dbg!(&times);
    dbg!(&dists);

    let mut total = 1;
    for (index, time) in times.iter().enumerate() {
        let ref_dist = dists[index];
        let mut cur = 0;
        for i in 0..*time {
            if is_winner(*time, ref_dist, i) {
                cur += 1;
            }
        }
        total *= cur;
    }

    total
}
