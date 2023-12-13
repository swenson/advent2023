pub fn day13() {
    let _inp = "#.##..##.
..#.##.#.
##......#
##......#
..#.##.#.
..##..##.
#.#.##.#.

#...##..#
#....#..#
..##..###
#####.##.
#####.##.
..##..###
#....#..#";
    let inp = std::fs::read_to_string("day13.input.txt").unwrap();
    let inp = inp.trim();

    let mut sum = 0;
    for pattern in inp.split("\n\n") {
        sum += score(pattern, 0);
    }

    println!("score = {}", sum);

    day13_2();
}

fn score(pattern: &str, ignore: usize) -> usize {
    let lines: Vec<_> = pattern.lines().collect();

    let ignore_first = if ignore >= 100 { ignore / 100 } else { 0 };
    let s = score_lines(&lines, ignore_first);
    if let Some(s) = s {
        return s * 100;
    }

    let mut cols = vec![];
    for c in 0..lines[0].len() {
        let mut x = vec![];
        for l in &lines {
            x.push(l.as_bytes()[c]);
        }
        let s = String::from_utf8(x).unwrap();
        cols.push(s);
    }
    let cols = cols.iter().map(|x| x.as_str()).collect();
    let ignore_second = if ignore >= 100 { 0 } else { ignore };
    score_lines(&cols, ignore_second).unwrap_or(0)
}

fn score_lines(lines: &Vec<&str>, ignore: usize) -> Option<usize> {
    for cut in 0..lines.len() - 1 {
        let mut equal = true;
        for x in 0..cut + 1 {
            let a = lines.get(cut - x);
            let b = lines.get(cut + x + 1);
            if a.is_some() && b.is_some() {
                if a.unwrap() != b.unwrap() {
                    equal = false;
                    break;
                }
            } else {
                if cut + 1 == ignore {
                    equal = false;
                    break;
                }
                return Some(cut + 1);
            }
        }
        if equal && cut + 1 != ignore {
            return Some(cut + 1);
        }
    }
    None
}

pub fn day13_2() {
    let _inp = "#.##..##.
..#.##.#.
##......#
##......#
..#.##.#.
..##..##.
#.#.##.#.

#...##..#
#....#..#
..##..###
#####.##.
#####.##.
..##..###
#....#..#";
    let inp = std::fs::read_to_string("day13.input.txt").unwrap();
    let _inp = inp.trim();

    let mut sum = 0;
    for pattern in inp.split("\n\n") {
        sum += score_modified(pattern).unwrap_or(score(pattern, 0));
    }

    println!("score = {}", sum);
}

fn score_modified(pattern: &str) -> Option<usize> {
    let original = score(pattern, 0);

    let mut patterns = vec![];
    for x in 0..pattern.len() {
        let mut p: Vec<u8> = pattern.as_bytes().to_vec();
        if p[x] == u8::try_from('.').unwrap() {
            p[x] = u8::try_from('#').unwrap();
            let s = String::from_utf8(p).unwrap();
            patterns.push(s);
        } else if p[x] == u8::try_from('#').unwrap() {
            p[x] = u8::try_from('.').unwrap();
            let s = String::from_utf8(p).unwrap();
            patterns.push(s);
        }
    }

    for p in &patterns {
        let new = score(p, original);
        if new == 0 {
            continue;
        }
        if new != original {
            return Some(new);
        }
    }
    None
}
