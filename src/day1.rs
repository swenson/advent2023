use std::fs;

pub fn day1() {
    let inp = fs::read_to_string("day1.input.txt").unwrap();
    let mut s: u64 = 0;
    for line in inp.split_whitespace() {
        let digits: Vec<u8> = line
            .as_bytes()
            .iter()
            .filter(|y| y.is_ascii_digit())
            .copied()
            .collect();
        let num = (digits.first().unwrap() - 48) * 10 + digits.last().unwrap() - 48;
        s += num as u64;
    }
    println!("sum = {}", s);
}

pub fn day1_2() {
    let inp = fs::read_to_string("day1.input.txt").unwrap();
    let mut s: u64 = 0;
    for line in inp.split_whitespace() {
        let line = line.replace("one", "one1one");
        let line = line.replace("two", "two2two");
        let line = line.replace("three", "three3three");
        let line = line.replace("four", "four4four");
        let line = line.replace("five", "five5five");
        let line = line.replace("six", "six6six");
        let line = line.replace("seven", "seven7seven");
        let line = line.replace("eight", "eight8eight");
        let line = line.replace("nine", "nine9nine");
        let digits: Vec<u8> = line
            .as_bytes()
            .iter()
            .filter(|y| y.is_ascii_digit())
            .copied()
            .collect();
        let num = (digits.first().unwrap() - 48) * 10 + digits.last().unwrap() - 48;
        s += num as u64;
    }
    println!("part 2 sum = {}", s);
}
