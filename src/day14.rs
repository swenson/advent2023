use memoize::memoize;

pub fn day14() {
    let _inp = "O....#....
O.OO#....#
.....##...
OO.#O....O
.O.....O#.
O.#..O.#.#
..O..#O..O
.......O..
#....###..
#OO..#....";
    let inp = std::fs::read_to_string("day14.input.txt").unwrap();
    let inp = inp.trim();

    let lines: Vec<_> = inp
        .lines()
        .map(|x| x.chars().collect::<Vec<char>>())
        .collect();

    let lines = tilt_north(lines);
    let score = weight(lines);
    println!("score = {}", score);

    day14_2();
}

fn weight(v: Vec<Vec<char>>) -> usize {
    let mut s = 0;
    for i in 0..v.len() {
        let x = v[i].iter().filter(|x| **x == 'O').count();
        let w = v.len() - i;
        s += x * w;
    }
    s
}

fn tilt_north(v: Vec<Vec<char>>) -> Vec<Vec<char>> {
    let start = v.clone();
    let tilted = tilt_north_once(v);

    if tilted != start {
        return tilt_north(tilted);
    }
    tilted
}

fn tilt_north_once(v: Vec<Vec<char>>) -> Vec<Vec<char>> {
    let mut w = v.clone();
    for i in 0..w.len() - 1 {
        for j in 0..w[i].len() {
            if w[i][j] == '.' && w[i + 1][j] == 'O' {
                w[i + 1][j] = '.';
                w[i][j] = 'O';
            }
        }
    }
    w
}

pub fn day14_2() {
    let _inp = "O....#....
O.OO#....#
.....##...
OO.#O....O
.O.....O#.
O.#..O.#.#
..O..#O..O
.......O..
#....###..
#OO..#....";
    let inp = std::fs::read_to_string("day14.input.txt").unwrap();
    let inp = inp.trim();

    let lines: Vec<_> = inp
        .lines()
        .map(|x| x.chars().collect::<Vec<char>>())
        .collect();

    let lines = cycle(lines, 1000000000);
    let score = weight(lines);
    println!("score = {}", score);
}

#[memoize]
fn cycle(lines: Vec<Vec<char>>, cycle_count: usize) -> Vec<Vec<char>> {
    if cycle_count == 0 {
        lines
    } else if cycle_count == 1 {
        tilt_east(tilt_south(tilt_west(tilt_north(lines))))
    } else if cycle_count == 2 {
        cycle(cycle(lines, 1), 1)
    } else if cycle_count % 2 == 0 {
        cycle(cycle(lines, cycle_count / 2), cycle_count / 2)
    } else {
        cycle(cycle(lines, cycle_count - 1), 1)
    }
}

fn tilt_west(v: Vec<Vec<char>>) -> Vec<Vec<char>> {
    let start = v.clone();
    let tilted = tilt_west_once(v);

    if tilted != start {
        return tilt_west(tilted);
    }
    tilted
}

fn tilt_west_once(v: Vec<Vec<char>>) -> Vec<Vec<char>> {
    let mut w = v.clone();

    for j in 0..w[0].len() - 1 {
        for wi in &mut w {
            if wi[j] == '.' && wi[j + 1] == 'O' {
                wi[j + 1] = '.';
                wi[j] = 'O';
            }
        }
    }
    w
}

fn tilt_south(v: Vec<Vec<char>>) -> Vec<Vec<char>> {
    let start = v.clone();
    let tilted = tilt_south_once(v);

    if tilted != start {
        return tilt_south(tilted);
    }
    tilted
}

fn tilt_south_once(v: Vec<Vec<char>>) -> Vec<Vec<char>> {
    let mut w = v.clone();

    for i in (1..w.len()).rev() {
        for j in 0..w[i].len() {
            if w[i][j] == '.' && w[i - 1][j] == 'O' {
                w[i - 1][j] = '.';
                w[i][j] = 'O';
            }
        }
    }
    w
}

fn tilt_east(v: Vec<Vec<char>>) -> Vec<Vec<char>> {
    let start = v.clone();
    let tilted = tilt_east_once(v);

    if tilted != start {
        return tilt_east(tilted);
    }
    tilted
}

fn tilt_east_once(v: Vec<Vec<char>>) -> Vec<Vec<char>> {
    let mut w = v.clone();
    for j in (1..w[0].len()).rev() {
        for wi in &mut w {
            if wi[j] == '.' && wi[j - 1] == 'O' {
                wi[j - 1] = '.';
                wi[j] = 'O';
            }
        }
    }
    w
}
