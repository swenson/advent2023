use std::collections::HashSet;

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
enum Direction {
    Right,
    Left,
    Up,
    Down,
}

pub fn day16() {
    let _inp = ".|...\\....
|.-.\\.....
.....|-...
........|.
..........
.........\\
..../.\\\\..
.-.-/..|..
.|....-|.\\
..//.|....";
    let inp = std::fs::read_to_string("day16.input.txt").unwrap();
    let inp = inp.trim();

    let map: Vec<Vec<char>> = inp.lines().map(|l| l.chars().collect()).collect();
    let score = trace(&map, 0, 0, Direction::Right, HashSet::new());
    let score: HashSet<_> = score.iter().map(|&(x, y, _)| (x, y)).collect();
    let score = score.len();
    println!("score = {}", score);
    day16_2();
}

fn trace(
    map: &Vec<Vec<char>>,
    x: isize,
    y: isize,
    dir: Direction,
    current: HashSet<(isize, isize, Direction)>,
) -> HashSet<(isize, isize, Direction)> {
    // memoize
    if current.contains(&(x, y, dir)) {
        return current;
    }
    if x < 0 || x >= map[0].len() as isize || y < 0 || y >= map.len() as isize {
        return current;
    }
    let mut result = current.clone();
    result.insert((x, y, dir));
    let c = map[y as usize][x as usize];
    if c == '.' {
        // empty
        match dir {
            Direction::Left => trace(map, x - 1, y, dir, result),
            Direction::Right => trace(map, x + 1, y, dir, result),
            Direction::Up => trace(map, x, y - 1, dir, result),
            Direction::Down => trace(map, x, y + 1, dir, result),
        }
    } else if c == '\\' || c == '/' {
        // mirror
        match (dir, c) {
            (Direction::Left, '\\') => trace(map, x, y - 1, Direction::Up, result),
            (Direction::Left, '/') => trace(map, x, y + 1, Direction::Down, result),
            (Direction::Right, '\\') => trace(map, x, y + 1, Direction::Down, result),
            (Direction::Right, '/') => trace(map, x, y - 1, Direction::Up, result),
            (Direction::Up, '\\') => trace(map, x - 1, y, Direction::Left, result),
            (Direction::Up, '/') => trace(map, x + 1, y, Direction::Right, result),
            (Direction::Down, '\\') => trace(map, x + 1, y, Direction::Right, result),
            (Direction::Down, '/') => trace(map, x - 1, y, Direction::Left, result),
            _ => panic!("Not possible"),
        }
    } else {
        // split
        match (dir, c) {
            (Direction::Left | Direction::Right, '|') => {
                let result = trace(map, x, y - 1, Direction::Up, result);
                trace(map, x, y + 1, Direction::Down, result)
            }
            (Direction::Up | Direction::Down, '-') => {
                let result = trace(map, x - 1, y, Direction::Left, result);
                trace(map, x + 1, y, Direction::Right, result)
            }
            (Direction::Left, '-') => trace(map, x - 1, y, dir, result),
            (Direction::Right, '-') => trace(map, x + 1, y, dir, result),
            (Direction::Up, '|') => trace(map, x, y - 1, dir, result),
            (Direction::Down, '|') => trace(map, x, y + 1, dir, result),
            _ => panic!("Not possible"),
        }
    }
}

pub fn day16_2() {
    let _inp = ".|...\\....
|.-.\\.....
.....|-...
........|.
..........
.........\\
..../.\\\\..
.-.-/..|..
.|....-|.\\
..//.|....";
    let inp = std::fs::read_to_string("day16.input.txt").unwrap();
    let inp = inp.trim();

    let map: Vec<Vec<char>> = inp.lines().map(|l| l.chars().collect()).collect();
    let mut max_score = 0;
    for x in 0..map[0].len() {
        let score = trace(&map, x as isize, 0, Direction::Down, HashSet::new());
        let score: HashSet<_> = score.iter().map(|&(x, y, _)| (x, y)).collect();
        let score = score.len();
        max_score = max_score.max(score);
        let score = trace(
            &map,
            x as isize,
            map.len() as isize - 1,
            Direction::Up,
            HashSet::new(),
        );
        let score: HashSet<_> = score.iter().map(|&(x, y, _)| (x, y)).collect();
        let score = score.len();
        max_score = max_score.max(score);
    }
    for y in 0..map.len() {
        let score = trace(&map, 0, y as isize, Direction::Right, HashSet::new());
        let score: HashSet<_> = score.iter().map(|&(x, y, _)| (x, y)).collect();
        let score = score.len();
        max_score = max_score.max(score);

        let score = trace(
            &map,
            map[0].len() as isize - 1,
            y as isize,
            Direction::Left,
            HashSet::new(),
        );
        let score: HashSet<_> = score.iter().map(|&(x, y, _)| (x, y)).collect();
        let score = score.len();
        max_score = max_score.max(score);
    }
    println!("score = {}", max_score);
}
