pub fn day9() {
    let _inp = "0 3 6 9 12 15
1 3 6 10 15 21
10 13 16 21 30 45";
    let inp = std::fs::read_to_string("day9.input.txt").unwrap();
    let inp = inp.trim();

    let mut sum = 0;
    for line in inp.lines() {
        let nums: Vec<i64> = line
            .split_whitespace()
            .map(|x| x.parse::<i64>().unwrap())
            .collect();
        sum += predict_next(&nums);
    }
    println!("sum = {}", sum);

    day9_2();
}

fn predict_next(nums: &[i64]) -> i64 {
    let mut diffs = vec![];
    diffs.push(diff(nums));
    while !diffs
        .last()
        .unwrap()
        .iter()
        .all(|x| *x == diffs.last().unwrap()[0])
    {
        diffs.push(diff(diffs.last().unwrap()));
    }
    diffs.iter().map(|x| x.last().unwrap()).sum::<i64>() + nums.last().unwrap()
}

fn diff(nums: &[i64]) -> Vec<i64> {
    if nums.len() <= 1 {
        return vec![0];
    }
    if nums.len() == 2 {
        return vec![nums[1] - nums[0]];
    }
    let mut next = vec![];
    for i in 0..nums.len() - 1 {
        next.push(nums[i + 1] - nums[i]);
    }
    next
}

pub fn day9_2() {
    let _inp = "0 3 6 9 12 15
1 3 6 10 15 21
10 13 16 21 30 45";
    let inp = std::fs::read_to_string("day9.input.txt").unwrap();
    let inp = inp.trim();

    let mut sum = 0;
    for line in inp.lines() {
        let nums: Vec<i64> = line
            .split_whitespace()
            .map(|x| x.parse::<i64>().unwrap())
            .collect();
        sum += predict_back(&nums);
    }
    println!("back = {}", sum);
}

fn predict_back(nums: &[i64]) -> i64 {
    let mut diffs = vec![];
    diffs.push(diff(nums));
    while !diffs
        .last()
        .unwrap()
        .iter()
        .all(|x| *x == diffs.last().unwrap()[0])
    {
        diffs.push(diff(diffs.last().unwrap()));
    }

    let mut last_diff = 0;
    for i in (0..diffs.len()).rev() {
        last_diff = *diffs[i].first().unwrap() - last_diff;
    }
    nums[0] - last_diff
}
