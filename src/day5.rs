pub fn day5() {
    let _inp = "seeds: 79 14 55 13

seed-to-soil map:
50 98 2
52 50 48

soil-to-fertilizer map:
0 15 37
37 52 2
39 0 15

fertilizer-to-water map:
49 53 8
0 11 42
42 0 7
57 7 4

water-to-light map:
88 18 7
18 25 70

light-to-temperature map:
45 77 23
81 45 19
68 64 13

temperature-to-humidity map:
0 69 1
1 0 69

humidity-to-location map:
60 56 37
56 93 4";
    let inp = std::fs::read_to_string("day5.input.txt").unwrap();

    let lines: Vec<_> = inp.lines().collect();

    let seeds: Vec<u64> = lines[0]
        .split(':')
        .last()
        .unwrap()
        .split_whitespace()
        .map(|x| x.parse().unwrap())
        .collect();

    let mut current = seeds;
    let mut next = current.clone();

    for line in lines.iter().skip(2) {
        if line.trim().is_empty() {
            current = next.clone();
            continue;
        }
        if line.contains("map") {
            continue;
        }
        let parts: Vec<u64> = line
            .split_whitespace()
            .map(|x| x.parse().unwrap())
            .collect();
        let dest = parts[0];
        let src = parts[1];
        let length = parts[2];

        for i in 0..current.len() {
            if current[i] >= src && current[i] <= src + length {
                let m = current[i] - src + dest;
                next[i] = m;
            }
        }
    }
    println!("min = {}", next.iter().min().unwrap());
}

pub fn day5_2() {
    let _inp = "seeds: 79 14 55 13

seed-to-soil map:
50 98 2
52 50 48

soil-to-fertilizer map:
0 15 37
37 52 2
39 0 15

fertilizer-to-water map:
49 53 8
0 11 42
42 0 7
57 7 4

water-to-light map:
88 18 7
18 25 70

light-to-temperature map:
45 77 23
81 45 19
68 64 13

temperature-to-humidity map:
0 69 1
1 0 69

humidity-to-location map:
60 56 37
56 93 4";
    let inp = std::fs::read_to_string("day5.input.txt").unwrap();

    let lines: Vec<_> = inp.lines().collect();

    let seeds: Vec<u64> = lines[0]
        .split(':')
        .last()
        .unwrap()
        .split_whitespace()
        .map(|x| x.parse().unwrap())
        .collect();

    // let mut all_nums = vec![];
    //
    // for line in lines.iter().skip(2) {
    //     if line.trim().is_empty() {
    //         continue;
    //     }
    //     if line.contains("map") {
    //         continue;
    //     }
    //     let parts: Vec<u64> = line
    //         .split_whitespace()
    //         .map(|x| x.parse().unwrap())
    //         .collect();
    //     let dest = parts[0];
    //     let src = parts[1];
    //     let length = parts[2];
    //     all_nums.push(src);
    //     if src > 0 {
    //         all_nums.push(src - 1);
    //     }
    //     all_nums.push(src + 1);
    //     all_nums.push(src + length - 1);
    //     all_nums.push(src + length);
    //     all_nums.push(src + length + 1);
    //     if dest > 0 {
    //         all_nums.push(dest - 1);
    //     }
    //     all_nums.push(dest);
    //     all_nums.push(dest + 1);
    //     all_nums.push(dest + length - 1);
    //     all_nums.push(dest + length);
    //     all_nums.push(dest + length + 1);
    // }

    // let mut current = vec![];
    // for i in 0..seeds.len()/2 {
    //     let a = seeds[2 * i];
    //     let b = a + seeds[2 * i + 1] - 1;
    //     current.push(a);
    //     current.push(b);
    //     for num in all_nums.iter() {
    //         if *num > a && *num < b {
    //             current.push(*num);
    //         }
    //     }
    // }
    // let mut next = current.clone();

    // for line in lines.iter().skip(2) {
    //     if line.trim().is_empty() {
    //         current = next.clone();
    //         continue;
    //     }
    //     if line.contains("map") {
    //         continue;
    //     }
    //     let parts: Vec<u64> = line
    //         .split_whitespace()
    //         .map(|x| x.parse().unwrap())
    //         .collect();
    //     let dest = parts[0];
    //     let src = parts[1];
    //     let length = parts[2];
    //
    //     for i in 0..current.len() {
    //         if current[i] >= src && current[i] < src + length {
    //             let m = current[i] - src + dest;
    //             next[i] = m;
    //         }
    //     }
    // }
    // println!("min = {}", next.iter().min().unwrap());

    let mut maps = vec![];
    let mut current_map = vec![];
    for line in lines.iter().skip(2) {
        if line.trim().is_empty() {
            continue;
        }
        if line.contains("map") {
            maps.push(current_map.clone());
            current_map.clear();
            continue;
        }
        let parts: Vec<u64> = line
            .split_whitespace()
            .map(|x| x.parse().unwrap())
            .collect();
        let dest = parts[0];
        let src = parts[1];
        let length = parts[2];
        current_map.push((dest, src, length));
    }
    if !current_map.is_empty() {
        maps.push(current_map.clone());
    }

    let mut min = u64::MAX;
    for i in 0..seeds.len() / 2 {
        let a = seeds[2 * i];
        let _b = a + seeds[2 * i + 1] - 1;
        println!("Checking {} numbers", seeds[2 * i + 1]);
        for a in seeds[2 * i]..seeds[2 * i] + seeds[2 * i + 1] {
            let mut current = a;
            for map in maps.iter() {
                for (dest, src, length) in map.iter() {
                    if current >= *src && current < *src + *length {
                        current = current - *src + *dest;
                        break;
                    }
                }
            }
            if current < min {
                min = current;
            }
        }
        println!("Current min = {}", min);
    }
    println!("min = {}", min);
}
