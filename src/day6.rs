pub fn day6() {
    let _inp = "Time:      7  15   30
Distance:  9  40  200";
    let inp = std::fs::read_to_string("day6.input.txt").unwrap();

    let lines: Vec<_> = inp.lines().collect();
    let times: Vec<u64> = lines[0]
        .split_whitespace()
        .map(|x| x.trim())
        .skip(1)
        .map(|x| x.parse().unwrap())
        .collect();
    let distances: Vec<u64> = lines[1]
        .split_whitespace()
        .map(|x| x.trim())
        .skip(1)
        .map(|x| x.parse().unwrap())
        .collect();

    let mut counts = vec![];
    for i in 0..times.len() {
        let mut count = 0u64;
        let max_time = times[i];
        for t in 0..max_time + 1 {
            let dist = (max_time - t) * t;
            if dist > distances[i] {
                count += 1;
            }
        }
        counts.push(count);
    }

    println!("product = {}", counts.iter().product::<u64>());

    day6_2();
}

pub fn day6_2() {
    let _inp = "Time:      7  15   30
Distance:  9  40  200";
    let inp = std::fs::read_to_string("day6.input.txt").unwrap();

    let lines: Vec<_> = inp.lines().collect();
    let max_time: u64 = lines[0]
        .split(':')
        .last()
        .unwrap()
        .replace(' ', "")
        .parse()
        .unwrap();
    let distance: u64 = lines[1]
        .split(':')
        .last()
        .unwrap()
        .replace(' ', "")
        .parse()
        .unwrap();
    let mut count: u64 = 0;
    for t in 0..max_time + 1 {
        let dist = (max_time - t) * t;
        if dist > distance {
            count += 1;
        }
    }
    println!("count = {}", count);
}
