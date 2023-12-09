use std::collections::{HashMap, HashSet};
use std::fs;

pub fn day3() {
    let inp = fs::read_to_string("day3.input.txt").unwrap();
    //     let inp = "467..114..
    // ...*......
    // ..35..633.
    // ......#...
    // 617*......
    // .....+.58.
    // ..592.....
    // ......755.
    // ...$.*....
    // .664.598..";
    let mut symbols = vec![];
    let mut r = 0;
    for line in inp.lines() {
        let mut c = -1;
        for x in line.trim().chars() {
            c += 1;
            if x.is_ascii_digit() || x == '.' {
                continue;
            }
            symbols.push((r, c));
        }
        r += 1;
    }

    let mut nums = vec![];
    for (r, line) in inp.lines().enumerate() {
        let mut c = -1;
        let mut adjacent = false;
        let mut in_number = false;
        let mut num = 0u64;
        for x in line.chars() {
            c += 1;
            if !in_number && x.is_ascii_digit() {
                in_number = true;
            }
            if in_number {
                if x.is_ascii_digit() {
                    for rr in r - 1..r + 2 {
                        for cc in c - 1..c + 2 {
                            adjacent = adjacent || symbols.contains(&(rr, cc));
                        }
                    }
                    num = num * 10 + x.to_digit(10).unwrap() as u64;
                }

                if !x.is_ascii_digit() || c == (line.len() - 1) as i32 {
                    if adjacent {
                        nums.push(num);
                    }
                    adjacent = false;
                    in_number = false;
                    num = 0;
                }
            }
        }
    }
    println!("sum = {}", nums.iter().sum::<u64>());
    day3_2();
}

fn day3_2() {
    let inp = fs::read_to_string("day3.input.txt").unwrap();
    //     let inp = "467..114..
    // ...*......
    // ..35..633.
    // ......#...
    // 617*......
    // .....+.58.
    // ..592.....
    // ......755.
    // ...$.*....
    // .664.598..";
    let mut gears = vec![];
    for (r, line) in inp.lines().enumerate() {
        let mut c = -1;
        for x in line.trim().chars() {
            c += 1;
            if x == '*' {
                gears.push((r, c));
            }
        }
    }

    let mut gear_nums: HashMap<(usize, i32), Vec<u64>> = HashMap::new();

    for (r, line) in inp.lines().enumerate() {
        let mut c = -1;
        let mut adjacent = HashSet::new();
        let mut in_number = false;
        let mut num = 0u64;
        for x in line.chars() {
            c += 1;
            if !in_number && x.is_ascii_digit() {
                in_number = true;
            }
            if in_number {
                if x.is_ascii_digit() {
                    for rr in r - 1..r + 2 {
                        for cc in c - 1..c + 2 {
                            if gears.contains(&(rr, cc)) {
                                adjacent.insert((rr, cc));
                            }
                        }
                    }
                    num = num * 10 + x.to_digit(10).unwrap() as u64;
                }

                if !x.is_ascii_digit() || c == (line.len() - 1) as i32 {
                    if !adjacent.is_empty() {
                        //nums.push(num);
                        for x in adjacent.iter() {
                            if gear_nums.contains_key(x) {
                                gear_nums.get_mut(x).unwrap().push(num);
                            } else {
                                gear_nums.insert(*x, vec![num]);
                            }
                        }
                    }
                    adjacent.clear();
                    in_number = false;
                    num = 0;
                }
            }
        }
    }
    let mut s = 0u64;
    for (_, nums) in gear_nums.iter() {
        if nums.len() == 2 {
            s += nums[0] * nums[1];
        }
    }
    println!("gear sum = {}", s);
}
