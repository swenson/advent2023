use memoize::memoize;

pub fn day12() {
    let _inp = "???.### 1,1,3
.??..??...?##. 1,1,3
?#?#?#?#?#?#?#? 1,3,1,6
????.#...#... 4,1,1
????.######..#####. 1,6,5
?###???????? 3,2,1";
    let inp = std::fs::read_to_string("day12.input.txt").unwrap();
    let inp = inp.trim();
    let inp = inp.replace('.', "x");

    let mut sum = 0;
    for line in inp.lines() {
        let line = line.trim_matches('x');
        let line = line.replace("xx", "x");
        sum += num_ways_start(&line);
    }

    println!("sum = {}", sum);

    day12_2();
}

fn num_ways_start(line: &str) -> u64 {
    let v: Vec<_> = line.split(' ').collect();
    let pattern = v[0];
    let nums: Vec<usize> = v[1].split(',').map(|x| x.parse().unwrap()).collect();

    num_ways_pieces(pattern.to_string(), nums)
}

#[memoize]
fn num_ways_pieces(pattern: String, nums: Vec<usize>) -> u64 {
    let pattern = pattern.as_str();
    // can we pull off a piece at the front?
    if let Some(stripped) = pattern.strip_prefix('x') {
        return num_ways_pieces(stripped.to_string(), nums);
    }
    if nums.is_empty() {
        if pattern.contains('#') {
            return 0;
        }
        return 1;
    }

    if pattern.starts_with('#') {
        let mut x = 1;
        while x < pattern.len() && pattern.as_bytes()[x] == u8::try_from('#').unwrap() {
            x += 1;
        }
        // too many #
        if x > nums[0] {
            return 0;
        }
    }

    let total: usize = nums.iter().copied().sum();
    let mut left = 0;
    for x in pattern.chars() {
        if x == '?' || x == '#' {
            left += 1;
        }
    }
    // not enough choices left
    if left == total {
        let p2 = pattern.replace('?', "#");
        if matches(&p2, &nums) {
            return 1;
        } else {
            return 0;
        }
    }
    if left < total {
        return 0;
    }
    if let Some(x) = pattern.find('x') {
        if !&pattern[0..x].contains('?') {
            if nums[0] == x {
                return num_ways_pieces((pattern[x + 1..]).to_string(), nums[1..].to_vec());
            } else {
                return 0;
            }
        }
    }
    if matches(pattern, &nums) {
        return 1;
    }
    if !pattern.contains('?') {
        return 0;
    }
    let b = pattern.replacen('?', "#", 1);
    let y = num_ways_pieces(b, nums.clone());
    let a = pattern.replacen('?', "x", 1);
    let x = num_ways_pieces(a, nums);
    x + y
}

fn matches(pattern: &str, nums: &[usize]) -> bool {
    if pattern.contains('?') {
        return false;
    }
    nums == pattern
        .split('x')
        .map(|x| x.len())
        .filter(|&x| x != 0)
        .collect::<Vec<_>>()
}

pub fn day12_2() {
    let _inp = "???.### 1,1,3
.??..??...?##. 1,1,3
?#?#?#?#?#?#?#? 1,3,1,6
????.#...#... 4,1,1
????.######..#####. 1,6,5
?###???????? 3,2,1";
    let inp = std::fs::read_to_string("day12.input.txt").unwrap();
    let inp = inp.trim();

    let mut sum = 0;
    for line in inp.lines() {
        let n = num_ways_start_unfold(line);
        sum += n;
    }

    println!("sum = {}", sum);
}

fn num_ways_start_unfold(line: &str) -> u64 {
    let v: Vec<_> = line.split(' ').collect();
    let pattern = v[0].to_string() + "?" + v[0] + "?" + v[0] + "?" + v[0] + "?" + v[0];
    let pattern = pattern.replace('.', "x");
    let nums: Vec<usize> = v[1].split(',').map(|x| x.parse().unwrap()).collect();
    let nums = nums.repeat(5);
    num_ways_pieces(pattern, nums.to_vec())
}
