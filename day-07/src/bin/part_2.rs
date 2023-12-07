use std::cmp::Ordering;
use std::collections::HashMap;

fn main() {
    let input = include_str!("../input.txt");
    let output = solution(input);
    println!("Output is:\n{output}");
}

fn get_rank(arr: &mut [i32]) -> i32 {
    let rank;

    let max = arr[0];
    let second_max = arr[1];
    if max >= 4 {
        rank = max + 4;
    } else if max == 3 {
        if second_max == 2 {
            rank = 5;
        } else {
            rank = 4;
        }
    } else if max == 2 && second_max == 2 {
        rank = 3;
    } else if max == 2 && second_max == 1 {
        rank = 2;
    } else {
        rank = 1;
    }
    rank
}

fn hand_compare(hand_a: &Vec<i32>, hand_b: &Vec<i32>) -> Ordering {
    let mut arrs: [[i32; 13]; 2] = [[0; 13]; 2];
    for (a, b) in hand_a.iter().zip(hand_b.iter()) {
        arrs[0][*a as usize - 2] += 1;
        arrs[1][*b as usize - 2] += 1;
    }

    let mut ranks: [i32; 2] = [0; 2];

    for (idx, arr) in arrs.iter_mut().enumerate() {
        let num_j_total = arr[9];
        arr[9] = 0;
        arr.sort();
        arr.reverse();
        arr[0] += num_j_total;
        ranks[idx] = get_rank(arr)
    }

    if ranks[0].cmp(&ranks[1]) == Ordering::Equal {
        let hand_a: Vec<i32> = hand_a
            .clone()
            .iter()
            .map(|elem| if *elem != 11 { *elem } else { 0 })
            .collect();

        let hand_b: Vec<i32> = hand_b
            .clone()
            .iter()
            .map(|elem| if *elem != 11 { *elem } else { 0 })
            .collect();

        return hand_a.cmp(&hand_b);
    }

    ranks[0].cmp(&ranks[1])
}

fn solution(input: &str) -> i32 {
    let mut map_value: HashMap<char, i32> = HashMap::new();

    map_value.insert('A', 14);
    map_value.insert('K', 13);
    map_value.insert('Q', 12);
    map_value.insert('J', 11);
    map_value.insert('T', 10);
    map_value.insert('9', 9);
    map_value.insert('8', 8);
    map_value.insert('7', 7);
    map_value.insert('6', 6);
    map_value.insert('5', 5);
    map_value.insert('4', 4);
    map_value.insert('3', 3);
    map_value.insert('2', 2);

    let mut hands: Vec<(Vec<i32>, i32)> = Vec::new();

    for line in input.lines() {
        let (hand, bid) = line.split_once(" ").unwrap();
        let hand = hand
            .chars()
            .map(|ch| *map_value.get(&ch).unwrap())
            .collect();

        let bid = bid.parse::<i32>().unwrap();
        hands.push((hand, bid));
    }

    hands.sort_by(|a, b| hand_compare(&a.0, &b.0));

    let mut total = 0;
    for (index, (_, bid)) in hands.iter().enumerate() {
        total += (index as i32 + 1) * bid;
    }
    total
}
