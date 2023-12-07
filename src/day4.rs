use std::collections::HashSet;

pub fn day4() {
    let _inp = "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11";
    let inp = std::fs::read_to_string("day4.input.txt").unwrap();
    let mut total = 0u64;
    for line in inp.lines() {
        let split = line.split(":");
        let nums: Vec<_> = split.last().unwrap().trim().split("|").collect();
        let winning: HashSet<_> = nums[0]
            .trim()
            .split_whitespace()
            .map(|x| x.parse::<u64>().unwrap())
            .collect();
        let score = nums[1]
            .trim()
            .split_whitespace()
            .map(|x| x.parse::<u64>().unwrap())
            .filter(|x| winning.contains(x))
            .count();
        if score > 0 {
            total += 1 << (score - 1);
        }
    }
    println!("score = {}", total);
    day4_2();
}

pub fn day4_2() {
    let _inp = "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11";
    let inp = std::fs::read_to_string("day4.input.txt").unwrap();
    let mut card = 0;
    let mut copies = vec![1u64; inp.lines().collect::<Vec<_>>().len() + 1];
    copies[0] = 0;
    for line in inp.lines() {
        card += 1;
        let split = line.split(":");
        let nums: Vec<_> = split.last().unwrap().trim().split("|").collect();
        let winning: HashSet<_> = nums[0]
            .trim()
            .split_whitespace()
            .map(|x| x.parse::<u64>().unwrap())
            .collect();
        let matches = nums[1]
            .trim()
            .split_whitespace()
            .map(|x| x.parse::<u64>().unwrap())
            .filter(|x| winning.contains(x))
            .count();
        for i in 1..matches + 1 {
            copies[card + i] += copies[card];
        }
    }
    println!("copies = {}", copies.iter().sum::<u64>());
}
