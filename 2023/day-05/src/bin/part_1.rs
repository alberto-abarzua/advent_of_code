fn main() {
    let input = include_str!("../input_1.txt");
    let output = part1(input);
    println!("Ouput is:\n{}", output);
}

#[derive(Debug)]
struct MapEntry {
    dest_init: i64,
    origin_init: i64,
    range: i64,
}

impl MapEntry {
    fn new(line: &str) -> MapEntry {
        let vals: Vec<i64> = line.split(' ').map(|num| num.parse().unwrap()).collect();
        MapEntry {
            dest_init: vals[0],
            origin_init: vals[1],
            range: vals[2],
        }
    }

    fn in_origin_range(&self, num: i64) -> bool {
        let origin_range = self.origin_init..(self.origin_init + self.range);

        origin_range.contains(&num)
    }

    fn get_dest(&self, origin: i64) -> i64 {
        if self.in_origin_range(origin) {
            return (origin - self.origin_init) + self.dest_init;
        } else {
            return origin;
        }
    }
}

fn part1(input: &str) -> String {
    let lines: Vec<&str> = input.lines().collect();
    let seeds: Vec<i64> = lines[0]
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
    }

    let mut min_loc: i64 = i64::MAX;

    for seed in seeds {
        let mut cur_val: i64 = seed;
        for map in maps.iter() {
            for entry in map.iter() {
                if entry.in_origin_range(cur_val) {
                    cur_val = entry.get_dest(cur_val);
                    break;
                }
            }
        }
        if cur_val < min_loc {
            min_loc = cur_val;
        }
    }

    return min_loc.to_string();
}
