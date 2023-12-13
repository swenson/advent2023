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
        sum += score(pattern);
    }

    println!("score = {}", sum);

    day13_2();
}

fn score(pattern: &str) -> usize {
    let lines: Vec<_> = pattern.lines().collect();

    let s = score_lines(&lines);
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
    score_lines(&cols).unwrap()
}

fn score_lines(lines: &Vec<&str>) -> Option<usize> {
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
                return Some(cut + 1);
            }
        }
        if equal {
            return Some(cut + 1);
        }
    }
    None
}

pub fn day13_2() {
    let inp = "#.##..##.
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
    let _inp = std::fs::read_to_string("day13.input.txt").unwrap();
    let _inp = inp.trim();
}
