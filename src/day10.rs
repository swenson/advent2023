use std::collections::{HashMap, HashSet};
use std::ops::Sub;

pub fn day10() {
    let _inp = ".....
.S-7.
.|.|.
.L-J.
.....";
    let _replace = 'F';
    let _inp = "..F7.
.FJ|.
SJ.L7
|F--J
LJ...";
    let _replace = 'F';

    let inp = std::fs::read_to_string("day10.input.txt").unwrap();
    let replace = '-';
    let inp = inp.trim();

    let lines: Vec<Vec<char>> = inp
        .replace('S', &replace.to_string())
        .lines()
        .map(|x| x.chars().collect())
        .collect();

    let mut sr = 0;
    let mut sc = 0;
    for (r, line) in inp.lines().enumerate() {
        for (c, ch) in line.chars().enumerate() {
            if ch == 'S' {
                sr = r;
                sc = c;
                break;
            }
        }
    }

    let mut dist = HashMap::new();
    let mut done = HashSet::new();
    let mut current_set = HashSet::new();
    current_set.insert((sr, sc));
    dist.insert((sr, sc), 0);

    while !current_set.is_empty() {
        let x = *current_set
            .iter()
            .min_by_key(|x| dist.get(*x).unwrap_or(&i32::MAX))
            .unwrap();
        current_set.remove(&x);
        done.insert(x);
        let current_distance = dist.get(&x).unwrap() + 1;
        let c = lines[x.0][x.1];
        let mut add = vec![];
        match c {
            '7' => {
                add.push((x.0 + 1, x.1));
                add.push((x.0, x.1 - 1));
            }
            '-' => {
                add.push((x.0, x.1 - 1));
                add.push((x.0, x.1 + 1));
            }
            '|' => {
                add.push((x.0 - 1, x.1));
                add.push((x.0 + 1, x.1));
            }
            'F' => {
                add.push((x.0 + 1, x.1));
                add.push((x.0, x.1 + 1));
            }
            'L' => {
                add.push((x.0 - 1, x.1));
                add.push((x.0, x.1 + 1));
            }
            'J' => {
                add.push((x.0 - 1, x.1));
                add.push((x.0, x.1 - 1));
            }
            '.' => panic!("Should not be able to get to ."),
            _ => panic!("Unknown character {}", c),
        }

        for y in add {
            if done.contains(&y) {
                continue;
            }
            current_set.insert(y);
            if dist.contains_key(&y) {
                if *dist.get(&y).unwrap() > current_distance {
                    dist.insert(y, current_distance);
                }
            } else {
                dist.insert(y, current_distance);
            }
        }
    }
    println!("max = {}", dist.values().max().unwrap());
}

pub fn day10_2() {
    let inp = std::fs::read_to_string("day10.input.txt").unwrap();
    let replace = '-';
    let inside = inner(&inp, replace);
    println!("inside = {}", inside);
}

pub fn inner(inp: &str, replace: char) -> usize {
    // Assume without loss of generality that the main loop's interior is
    // the right-hand-side when starting clockwise.
    // (If it is not, then modify the starting point so that it is.)
    // The 'replace' character could be determined programmatically but is
    // easier to eyeball it when setting the input.

    let inp = inp.trim();

    let lines: Vec<Vec<char>> = inp
        .replace('S', &replace.to_string())
        .lines()
        .map(|x| x.chars().collect())
        .collect();
    let rows = lines.len();
    let cols = lines[0].len();

    let mut sr = 0;
    let mut sc = 0;
    for (r, line) in inp.lines().enumerate() {
        for (c, ch) in line.chars().enumerate() {
            if ch == 'S' {
                sr = r;
                sc = c;
                break;
            }
        }
    }

    let mut dir = match replace {
        '-' | 'F' | 'L' => 'R',
        '|' | '7' => 'D',
        'J' => 'L',
        _ => panic!("invalid initial direction {}", replace),
    };
    let mut done = HashSet::new();
    let mut inner_fill = HashSet::new();
    let mut current = (sr, sc);
    loop {
        done.insert(current);
        let next = match dir {
            'R' => (current.0, current.1 + 1),
            'D' => (current.0 + 1, current.1),
            'L' => (current.0, current.1 - 1),
            'U' => (current.0 - 1, current.1),
            _ => panic!("Invalid direction {}", dir),
        };
        let c = lines[current.0][current.1];
        let n = lines[next.0][next.1];
        match dir {
            'R' => {
                if current.0 < rows - 1 {
                    inner_fill.insert((current.0 + 1, current.1));
                }
            }
            'D' => {
                if current.1 > 0 {
                    inner_fill.insert((current.0, current.1 - 1));
                }
            }
            'L' => {
                if current.0 > 0 {
                    inner_fill.insert((current.0 - 1, current.1));
                }
            }
            'U' => {
                if current.1 < cols - 1 {
                    inner_fill.insert((current.0, current.1 + 1));
                }
            }
            _ => panic!("Invalid direction {}", dir),
        };
        dir = match (dir, n) {
            ('R', '7') => 'D',
            ('R', '-') => 'R',
            ('R', 'J') => {
                if current.0 < rows - 1 && current.1 < cols - 1 {
                    inner_fill.insert((current.0 + 1, current.1 + 1));
                }
                'U'
            }

            ('D', '|') => 'D',
            ('D', 'L') => {
                if current.0 < rows - 1 && current.1 > 0 {
                    inner_fill.insert((current.0 + 1, current.1 - 1));
                }
                'R'
            }
            ('D', 'J') => 'L',

            ('L', '-') => 'L',
            ('L', 'L') => {
                if current.0 > 0 && current.1 > 0 {
                    inner_fill.insert((current.0 - 1, current.1 - 1));
                }
                'U'
            }
            ('L', 'F') => 'D',

            ('U', '|') => 'U',
            ('U', '7') => 'L',
            ('U', 'F') => {
                if current.0 > 0 && current.1 < cols - 1 {
                    inner_fill.insert((current.0 - 1, current.1 + 1));
                }
                'R'
            }
            _ => panic!("Invalid direction and next char {} {}", dir, c),
        };

        current = next;
        if current == (sr, sc) {
            break;
        }
    }

    inner_fill = inner_fill.sub(&done);

    let mut inside = 0;
    while !inner_fill.is_empty() {
        inside += 1;
        let x = *inner_fill.iter().take(1).last().unwrap();
        inner_fill.remove(&x);
        done.insert(x);
        if x.0 < rows - 1 && !done.contains(&(x.0 + 1, x.1)) {
            inner_fill.insert((x.0 + 1, x.1));
        }
        if x.1 < cols - 1 && !done.contains(&(x.0, x.1 + 1)) {
            inner_fill.insert((x.0, x.1 + 1));
        }
        if x.0 > 0 && !done.contains(&(x.0 - 1, x.1)) {
            inner_fill.insert((x.0 - 1, x.1));
        }
        if x.1 > 0 && !done.contains(&(x.0, x.1 - 1)) {
            inner_fill.insert((x.0, x.1 - 1));
        }
    }
    inside
}

#[cfg(test)]
mod test {
    use crate::day10::inner;

    #[test]
    fn test_1() {
        let inp = ".....
.S-7.
.|.|.
.L-J.
.....";
        let replace = 'F';
        assert_eq!(1, inner(inp, replace));
    }

    #[test]
    fn test_2() {
        let inp = "..F7.
.FJ|.
SJ.L7
|F--J
LJ...";
        let replace = 'F';
        assert_eq!(1, inner(inp, replace));
    }

    #[test]
    fn test_3() {
        let inp = "..........
.S------7.
.|F----7|.
.||OOOO||.
.||OOOO||.
.|L-7F-J|.
.|II||II|.
.L--JL--J.
..........";
        let replace = 'F';
        assert_eq!(4, inner(inp, replace));
    }

    #[test]
    fn test_4() {
        let inp = ".S----7F7F7F7F-7....
.|F--7||||||||FJ....
.||.FJ||||||||L7....
FJL7L7LJLJ||LJ.L-7..
L--J.L7...LJF7F-7L7.
....F-J..F7FJ|L7L7L7
....L7.F7||L7|.L7L7|
.....|FJLJ|FJ|F7|.LJ
....FJL-7.||.||||...
....L---J.LJ.LJLJ...";
        let replace = 'F';
        assert_eq!(8, inner(inp, replace));
    }

    #[test]
    fn test_5() {
        let inp = "FF7FSF7F7F7F7F7F---7
L|LJ||||||||||||F--J
FL-7LJLJ||||||LJL-77
F--JF--7||LJLJ7F7FJ-
L---JF-JLJ.||-FJLJJ7
|F|F-JF---7F7-L7L|7|
|FFJF7L7F-JF7|JL---7
7-L-JL7||F7|L7F-7F7|
L.L7LFJ|||||FJL7||LJ
L7JLJL-JLJLJL--JLJ.L";
        let replace = '7';
        assert_eq!(10, inner(inp, replace));
    }
}
