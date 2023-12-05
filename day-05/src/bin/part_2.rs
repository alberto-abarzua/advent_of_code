fn main() {
    let input = include_str!("../input_1.txt");
    let output = part2(input);
    println!("Ouput is:\n{}", output);
}

#[derive(Debug)]
struct MapEntry {
    dest_start: i64,
    origin_start: i64,
    origin_end: i64,
}

impl MapEntry {
    fn new(line: &str) -> MapEntry {
        let vals: Vec<i64> = line.split(' ').map(|num| num.parse().unwrap()).collect();
        let range = vals[2];
        let dest_start = vals[0];
        let origin_start = vals[1];

        MapEntry {
            dest_start,
            origin_start,
            origin_end: origin_start + range,
        }
    }

    fn get_dest(&self, origin: i64) -> i64 {
        return origin - self.origin_start + self.dest_start;
    }
}

fn part2(input: &str) -> i64 {
    let lines: Vec<&str> = input.lines().collect();
    let seed_ranges: Vec<i64> = lines[0]
        .split_once(": ")
        .unwrap()
        .1
        .split(" ")
        .map(|num| num.parse().unwrap())
        .collect();

    let mut maps: Vec<Vec<MapEntry>> = Vec::new();

    let mut cur_idx: i32 = -1;
    for line in lines.iter().skip(1) {
        if line.len() == 0 {
            continue;
        }

        if line.chars().last() == Some(':') {
            cur_idx += 1;
            maps.push(Vec::new());
            continue;
        }
        let cur_list: &mut Vec<MapEntry> = maps.get_mut(cur_idx as usize).unwrap();
        let new_entry = MapEntry::new(line);
        cur_list.push(new_entry);
        cur_list.sort_by_key(|e| e.dest_start);
    }

    let mut seeds: Vec<(i64, i64)> = Vec::new();

    for i in (0..seed_ranges.len()).step_by(2) {
        seeds.push((seed_ranges[i], seed_ranges[i] + seed_ranges[i + 1]))
    }

    for map in maps {
        let mut new: Vec<(i64, i64)> = Vec::new();

        while seeds.len() > 0 {
            let (seed_start, seed_end) = seeds.pop().unwrap();
            let mut added = false;
            for entry in map.iter() {
                let range_start = std::cmp::max(seed_start, entry.origin_start);
                let range_end = std::cmp::min(seed_end, entry.origin_end);
                if range_start < range_end {
                    new.push((entry.get_dest(range_start), entry.get_dest(range_end)));
                    if range_start > seed_start {
                        seeds.push((seed_start, range_start))
                    }
                    if seed_end < range_end {
                        seeds.push((range_end, seed_end))
                    }
                    added = true;
                }
            }
            if !added {
                new.push((seed_start, seed_end));
            }
        }
        seeds = new.clone();
    }

    seeds.sort_by_key(|k| k.0);
    return seeds[0].0;
}
