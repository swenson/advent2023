use num_integer::Integer;
use std::collections::{HashMap, HashSet};

pub fn day8() {
    let _inp = "RL

AAA = (BBB, CCC)
BBB = (DDD, EEE)
CCC = (ZZZ, GGG)
DDD = (DDD, DDD)
EEE = (EEE, EEE)
GGG = (GGG, GGG)
ZZZ = (ZZZ, ZZZ)";
    let _inp = "LLR

AAA = (BBB, BBB)
BBB = (AAA, ZZZ)
ZZZ = (ZZZ, ZZZ)";
    let inp = std::fs::read_to_string("day8.input.txt").unwrap();
    let inp = inp.trim();

    let lines: Vec<_> = inp.lines().map(|x| x.trim()).collect();
    let dirs: Vec<_> = lines[0].chars().collect();

    let mut map = HashMap::new();

    for li in 2..lines.len() {
        let line = lines[li];
        let parts: Vec<_> = line.split("=").map(|x| x.trim()).collect();
        let from = parts[0];
        let rhs = parts[1];
        let rhs: Vec<_> = rhs[1..rhs.len() - 1].split(",").map(|x| x.trim()).collect();
        let left = rhs[0];
        let right = rhs[1];
        map.insert(from, (left, right));
    }

    let mut count = 0;
    let mut current = "AAA";

    while current != "ZZZ" {
        if dirs[count % dirs.len()] == 'L' {
            current = map[current].0;
        } else {
            current = map[current].1;
        }
        count += 1;
    }
    println!("count = {}", count);

    day8_2();
}

pub fn day8_2() {
    let _inp = "LR

11A = (11B, XXX)
11B = (XXX, 11Z)
11Z = (11B, XXX)
22A = (22B, XXX)
22B = (22C, 22C)
22C = (22Z, 22Z)
22Z = (22B, 22B)
XXX = (XXX, XXX)";
    let inp = std::fs::read_to_string("day8.input.txt").unwrap();
    let inp = inp.trim();

    let lines: Vec<_> = inp.lines().map(|x| x.trim()).collect();
    let dirs: Vec<_> = lines[0].chars().collect();

    let mut keys = HashSet::new();

    for li in 2..lines.len() {
        let line = lines[li];
        let parts: Vec<_> = line.split("=").map(|x| x.trim()).collect();
        let from = parts[0];
        let rhs = parts[1];
        let rhs: Vec<_> = rhs[1..rhs.len() - 1].split(",").map(|x| x.trim()).collect();
        let left = rhs[0];
        let right = rhs[1];
        keys.insert(from);
        keys.insert(left);
        keys.insert(right);
    }

    let dir_mult = dirs.len();
    let mut key_map = HashMap::new();
    for key in &keys {
        key_map.insert(*key, key_map.len());
    }

    let mut map = vec![0usize; keys.len() * dir_mult * 2];
    let mut current = vec![];

    for li in 2..lines.len() {
        let line = lines[li];
        let parts: Vec<_> = line.split("=").map(|x| x.trim()).collect();
        let from = parts[0];
        let rhs = parts[1];
        let rhs: Vec<_> = rhs[1..rhs.len() - 1].split(",").map(|x| x.trim()).collect();
        let left = rhs[0];
        let right = rhs[1];
        let start = from.ends_with("A");
        let from_end = from.ends_with("Z");
        let left_end = left.ends_with("Z");
        let right_end = right.ends_with("Z");
        let from = *key_map.get(from).unwrap();
        let left = *key_map.get(left).unwrap();
        let right = *key_map.get(right).unwrap();
        for d in 0..dir_mult {
            let f = from * dir_mult * 2 + d * 2 + if from_end { 1 } else { 0 };
            let l = left * dir_mult * 2 + (d + 1) % dir_mult * 2 + if left_end { 1 } else { 0 };
            let r = right * dir_mult * 2 + (d + 1) % dir_mult * 2 + if right_end { 1 } else { 0 };
            if dirs[d] == 'L' {
                map[f] = l;
            } else {
                map[f] = r;
            }
        }
        if start {
            current.push(from * dir_mult * 2);
        }
    }

    let mut l = 0;

    // We cheat a little. We know from looking at find_cycle that the
    // cycles are nicely formed, so the solution to the
    // chinese remainder theorem will just be 0 mod the LCM of
    // the cycle lengths.
    for start in current.iter() {
        let (_, _, cycle_len) = find_cycle(&map, *start);
        if l == 0 {
            l = cycle_len;
        } else {
            l = cycle_len.lcm(&l);
        }
    }
    println!("solution = {:?}", l);
}

fn find_cycle(map: &Vec<usize>, start: usize) -> (Vec<usize>, usize, usize) {
    let mut seen = HashMap::new();
    let mut x = start;
    let mut finals = vec![];
    let mut i = 0usize;
    while !(seen.contains_key(&x) && (x & 1 == 1)) {
        seen.insert(x, i);
        x = map[x];
        if x & 1 == 1 {
            finals.push(i);
        }
        i += 1;
    }
    let cycle_start = *seen.get(&x).unwrap();
    let cycle_len = i - cycle_start;
    (finals, cycle_start, cycle_len)
}
