use std::ops::Index;

pub fn day12() {
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
        let line = line.trim_matches('.');
        let line = line.replace("..", ".");
        sum += num_ways_start(&line);
    }

    println!("sum = {}", sum);

    day12_2();
}

fn num_ways_start(line: &str) -> u64 {
    let v: Vec<_> = line.split(' ').collect();
    let mut pattern: Vec<char> = v[0].chars().collect();
    let nums: Vec<usize> = v[1].split(',').map(|x| x.parse().unwrap()).collect();

    num_ways(&mut pattern, &nums)
}

fn num_ways(pattern: &mut Vec<char>, nums: &[usize]) -> u64 {
    // let p: String = pattern.iter().collect();
    // println!("checking {}", p);
    if matches(pattern, nums) {
        // println!("  ok => 1");
        return 1;
    }
    if !pattern.contains(&'?') {
        // println!("  done");
        return 0;
    }
    if !possible(pattern, nums) {
        // println!("  impossible");
        return 0;
    }
    let i = pattern.iter().enumerate().find(|&(_,&c)| c == '?').unwrap().0;
    pattern[i] = '#';
    let x = num_ways(pattern, nums);
    pattern[i] = '.';
    let y = num_ways(pattern, nums);
    pattern[i] = '?';
    x + y
}

// fn pieces(x: &str) -> Vec<&str> {
//     x.split('.').iter().filter(|x| x.len() > 0).collect()
// }

fn possible(pattern: &Vec<char>, nums: &[usize]) -> bool {
    let p: String = pattern.iter().collect();
    let x: String = p.replace('?', ".");
    let y: Vec<_> = x.split('.').filter(|&x| x.len() > 0 && !x.contains('?')).map(|x| x.len()).collect();
    let first_q = p.find("?").unwrap_or(0);
    let first_h = p.find("#").unwrap_or(0);
    let z = *y.first().unwrap_or(&0);
    if first_q > first_h && z > nums[0] {
        //println!("  because first: {:?} vs {:?}", y, nums);
        false
    } else if y.iter().max().unwrap_or(&0) > nums.iter().max().unwrap() {
        //println!("  because max");
        false
    } else {
        true
    }
}

fn matches(pattern: &Vec<char>, nums: &[usize]) -> bool {
    if pattern.contains(&'?') {
        return false;
    }
    nums == pattern.split(|&x| x == '.').map(|x| x.len()).filter(|&x| x != 0).collect::<Vec<_>>()
}

pub fn day12_2() {
    let inp = "???.### 1,1,3
.??..??...?##. 1,1,3";
// ?#?#?#?#?#?#?#? 1,3,1,6
// ????.#...#... 4,1,1
// ????.######..#####. 1,6,5
// ?###???????? 3,2,1";
    let _inp = std::fs::read_to_string("day12.input.txt").unwrap();
    let inp = inp.trim();

    let mut sum = 0;
    for line in inp.lines() {
        sum += num_ways_start_unfold(&line);
        println!("temp sum = {}", sum);
    }

    println!("sum = {}", sum);
}

fn num_ways_start_unfold(line: &str) -> u64 {
    let v: Vec<_> = line.split(' ').collect();
    let pattern = v[0];
    let mut p2 = vec![];
    for i in 0..5 {
        for c in pattern.chars() {
            p2.push(c);
        }
        if i != 4 {
            p2.push('?');
        }
    }
    let mut pattern = p2;
    //let mut pattern = pattern.repeat(5).trim_matches('.').chars().collect();
    let nums: Vec<usize> = v[1].split(',').map(|x| x.parse().unwrap()).collect();
    let nums = nums.repeat(5);
    num_ways(&mut pattern, &nums)
}
