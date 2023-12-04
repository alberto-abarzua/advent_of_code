use std::collections::HashSet;

fn main() {
    let input = include_str!("../input_1.txt");
    let output = part2(input);
    println!("Ouput is:\n{}", output);
}

struct Card {
    num_winner: i32,
}

impl Card {
    fn new(line: &str) -> Card {
        let (_, content) = line.split_once(": ").unwrap();
        let (mut winner, mut actual) = content.split_once(" | ").unwrap();
        if winner.chars().nth(0).unwrap() == ' ' {
            winner = winner.get(1..).unwrap();
        }
        if actual.chars().nth(0).unwrap() == ' ' {
            actual = actual.get(1..).unwrap();
        }

        let winner: HashSet<i32> = winner
            .replace("  ", " ")
            .split(" ")
            .map(|num| num.parse().unwrap())
            .collect();
        let actual: Vec<i32> = actual
            .replace("  ", " ")
            .split(" ")
            .map(|num| num.parse().unwrap())
            .collect();

        let mut num_winner = 0;

        for num in actual.iter() {
            if winner.contains(num) {
                num_winner += 1;
            }
        }

        Card { num_winner }
    }
}

fn handle_card(cards: &Vec<Card>, index: usize) -> i32 {
    let card = cards.get(index).unwrap();
    let bottom_idx = index + 1;
    let top_idx = index + 1 + card.num_winner as usize;
    let mut res: i32 = 0;

    for i in bottom_idx..top_idx {
        res += handle_card(cards, i);
    }

    return res + 1;
}

fn part2(input: &str) -> String {
    let mut total: i32 = 0;
    let mut cards: Vec<Card> = Vec::new();

    for line in input.lines() {
        let card = Card::new(line);
        cards.push(card);
    }

    for index in 0..cards.len() {
        total += handle_card(&cards, index);
    }

    return total.to_string();
}
