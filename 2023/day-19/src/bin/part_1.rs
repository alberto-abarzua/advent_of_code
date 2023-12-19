use std::collections::HashMap;
fn main() {
    println!("Solution:\n{:?}", solution(include_str!("../input.txt")));
}

#[derive(Debug, Clone)]
struct Part {
    x: i32,
    m: i32,
    a: i32,
    s: i32,
}

fn solution(input: &str) -> i32 {
    let (workflows, parts) = input.split_once("\n\n").unwrap();
    let mut wmap: HashMap<&str, Vec<(Box<dyn Fn(Part) -> bool>, &str)>> = HashMap::new();

    for line in workflows.lines() {
        let (name, rest) = line.split_once("{").unwrap();
        let conditions: Vec<&str> = rest.strip_suffix("}").unwrap().split(",").collect();

        for condition in conditions {
            let less = condition.contains("<");
            let more = condition.contains(">");

            if !less && !more {
                let (closure, dest): (Box<dyn Fn(Part) -> bool>, &str) = match condition {
                    "A" => (Box::new(move |_p| true), "A"),
                    "R" => (Box::new(move |_p| false), "R"),
                    _ => (Box::new(move |_p| true), condition),
                };
                wmap.entry(name).or_insert(Vec::new()).push((closure, dest));
                continue;
            }

            let (cat, val) = if less {
                condition.split_once("<").unwrap()
            } else {
                condition.split_once(">").unwrap()
            };

            let (num, dest) = val.split_once(":").unwrap();
            let num = num.parse::<i32>().unwrap();

            let closure: Box<dyn Fn(Part) -> bool> = match cat {
                "x" => Box::new(move |p| less && p.x < num || !less && p.x > num),
                "m" => Box::new(move |p| less && p.m < num || !less && p.m > num),
                "a" => Box::new(move |p| less && p.a < num || !less && p.a > num),
                "s" => Box::new(move |p| less && p.s < num || !less && p.s > num),
                _ => continue,
            };
            wmap.entry(name).or_insert(Vec::new()).push((closure, dest));
        }
    }

    let mut part_vec: Vec<Part> = Vec::new();

    for part in parts.lines() {
        // remove { and } and end and start
        let part = part.strip_prefix("{").unwrap().strip_suffix("}").unwrap();
        let part_vallues = part.split(",");
        let values = part_vallues
            .map(|v| v.split_once("=").unwrap().1.parse::<i32>().unwrap())
            .collect::<Vec<i32>>();
        part_vec.push(Part {
            x: values[0],
            m: values[1],
            a: values[2],
            s: values[3],
        });
    }


    let mut total = 0;

    for part in part_vec.iter_mut() {
        // we should start at the workflow in
        let mut prev_workflow = "in";
        let mut cur_workflow = "in";

        loop {
            let mut accepted = None;

            let cur_closures = wmap.get(cur_workflow).unwrap();

            for (closure, dest) in cur_closures.iter() {
                let result = closure(part.clone());
                if result {
                    match dest {
                        &"A" => accepted = Some(true),
                        &"R" => accepted = Some(false),
                        _ => cur_workflow = dest,
                    }

                    if accepted.is_some() {
                        if accepted.unwrap() {
                            total += part.x + part.m + part.a + part.s;
                        }
                    }
                    break;
                }
            }
            if prev_workflow == cur_workflow || accepted.is_some() {
                break;
            }

            prev_workflow = cur_workflow;
        }
    }

    total
}
