use std::{collections::HashMap, i128};
fn main() {
    println!("Solution:\n{:?}", solution(include_str!("../sample.txt")));
}

fn solution(input: &str) -> i128 {
    let (block1, _) = input.split_once("\n\n").unwrap();
    let mut workflows = HashMap::new();

    for line in block1.lines() {
        let (name, rest) = line.split_once("{").unwrap();
        let rest = rest.strip_suffix("}").unwrap();
        let mut rules = rest.split(",").collect::<Vec<_>>();
        let fallback = rules.pop().unwrap();
        let mut rule_vec = Vec::new();
        for rule in rules {
            let (comparison, target) = rule.split_once(":").unwrap();
            let key = comparison.chars().next().unwrap();
            let cmp = comparison.chars().nth(1).unwrap();
            let n = comparison[2..].parse::<i32>().unwrap();
            rule_vec.push((key, cmp, n, target));
        }
        workflows.insert(name, (rule_vec, fallback));
    }

    count(
        &workflows,
        &mut ['x', 'm', 'a', 's']
            .iter()
            .map(|&c| (c, (1, 4000)))
            .collect::<HashMap<_, _>>(),
        "in",
    )
}

fn count(
    workflows: &HashMap<&str, (Vec<(char, char, i32, &str)>, &str)>,
    ranges: &mut HashMap<char, (i32, i32)>,
    name: &str,
) -> i128 {
    if name == "R" {
        return 0;
    }

    if name == "A" {
        let mut product = 1;
        for (lo, hi) in ranges.values() {
            product *= (hi - lo + 1) as i128;
        }
        return product;
    }

    let (rules, fallback) = &workflows[name];
    let mut total = 0;

    for &(key, cmp, n, target) in rules {
        let (lo, hi) = *ranges.entry(key).or_insert((1, 4000));
        let (t, f) = match cmp {
            '<' => ((lo, i32::min(n - 1, hi)), (i32::max(n, lo), hi)),
            '>' => ((i32::max(n + 1, lo), hi), (lo, i32::min(n, hi))),
            _ => unreachable!(),
        };

        if t.0 <= t.1 {
            let mut copy = ranges.clone();
            copy.insert(key, t);
            total += count(workflows, &mut copy, target);
        }
        if f.0 <= f.1 {
            ranges.insert(key, f);
        } else {
            break;
        }
    }

    total += count(workflows, ranges, fallback);

    total
}
