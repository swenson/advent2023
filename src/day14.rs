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

    let lines: Vec<_> = inp.lines().map(|x| x.chars().collect::<Vec<char>>()).collect();

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
    for i in 0..w.len()-1 {
        for j in 0..w[i].len() {
            if w[i][j] == '.' {
                if w[i + 1][j] == 'O' {
                    w[i + 1][j] = '.';
                    w[i][j] = 'O';
                }
            }
        }
    }
    w
}

pub fn day14_2() {
    let _inp = "";
    let inp = std::fs::read_to_string("day14.input.txt").unwrap();
    let inp = inp.trim();

}