use memoize::lazy_static::lazy_static;
use std::collections::{HashMap, HashSet};

pub fn day21() {
    let _inp = "...........
.....###.#.
.###.##..#.
..#.#...#..
....#.#....
.##..S####.
.##..#...#.
.......##..
.##.#.####.
.##..##.##.
...........";
    let inp = std::fs::read_to_string("day21.input.txt").unwrap();
    let inp = inp.trim();

    let lines: Vec<Vec<_>> = inp.lines().map(|x| x.chars().collect()).collect();
    let count = reachable(&lines, 64);
    println!("count = {}", count);
}

fn reachable(map: &Vec<Vec<char>>, steps: usize) -> usize {
    let mut start = (0, 0);
    let mut newmap = vec![];
    for r in 0..map.len() {
        let mut newrow = vec![];
        for c in 0..map[0].len() {
            if map[r][c] == 'S' {
                start = (r, c);
                newrow.push('.');
            } else {
                newrow.push(map[r][c]);
            }
        }
        newmap.push(newrow);
    }
    let map = newmap;
    let set = HashSet::from([start]);
    run_steps(&map, &set, steps).len()
}

fn run_steps(
    map: &Vec<Vec<char>>,
    set: &HashSet<(usize, usize)>,
    steps: usize,
) -> HashSet<(usize, usize)> {
    let maxc = map[0].len() - 1;
    let maxr = map.len() - 1;
    if steps == 1 {
        let mut newset = HashSet::new();
        for &(r, c) in set {
            if r > 0 && map[r - 1][c] == '.' {
                newset.insert((r - 1, c));
            }
            if r < maxr && map[r + 1][c] == '.' {
                newset.insert((r + 1, c));
            }
            if c > 0 && map[r][c - 1] == '.' {
                newset.insert((r, c - 1));
            }
            if c < maxc && map[r][c + 1] == '.' {
                newset.insert((r, c + 1));
            }
        }
        newset
    } else if steps % 2 == 0 {
        run_steps(map, &run_steps(map, set, steps / 2), steps / 2)
    } else {
        run_steps(map, &run_steps(map, set, steps - 1), 1)
    }
}

lazy_static! {
    static ref SMAP: Vec<Vec<char>> = {
        let inp = std::fs::read_to_string("day21.input.txt").unwrap();
        let inp = inp.trim();
        let lines: Vec<Vec<_>> = inp.lines().map(|x| x.chars().collect()).collect();
        lines
    };
    static ref MAP: Vec<Vec<char>> = {
        let inp = std::fs::read_to_string("day21.input.txt").unwrap();
        let inp = inp.trim().replace('S', ".");
        let lines: Vec<Vec<_>> = inp.lines().map(|x| x.chars().collect()).collect();
        lines
    };
}

const MAXC: usize = 131;
//const MAXC: usize = 11;
const MAXR: usize = 131;
//const MAXR: usize = 11;

pub fn day21_2() {
    let _inp = "...........
.....###.#.
.###.##..#.
..#.#...#..
....#.#....
.##..S####.
.##..#...#.
.......##..
.##.#.####.
.##..##.##.
...........";
    let inp = std::fs::read_to_string("day21.input.txt").unwrap();
    let inp = inp.trim();

    let lines: Vec<Vec<_>> = inp.lines().map(|x| x.chars().collect()).collect();
    let count = reachable_wrap(&lines, 26501365);
    println!("count = {}", count);
}

fn reachable_wrap(map: &Vec<Vec<char>>, steps: usize) -> usize {
    let mut start = (0, 0);
    for r in 0..map.len() {
        for c in 0..map[0].len() {
            if map[r][c] == 'S' {
                start = (r, c);
            }
        }
    }
    run_steps_parity(start, steps)
}
fn step1((r, c): (usize, usize)) -> Vec<((usize, usize), (i32, i32))> {
    let maxc = MAP[0].len() - 1;
    let maxr = MAP.len() - 1;
    let mut v = vec![];
    let rm1 = if r > 0 { r - 1 } else { maxr };
    let rp1 = if r < maxr { r + 1 } else { 0 };
    let cm1 = if c > 0 { c - 1 } else { maxc };
    let cp1 = if c < maxc { c + 1 } else { 0 };
    if MAP[rm1][c] == '.' {
        if r == 0 {
            v.push(((rm1, c), (-1, 0)));
        } else {
            v.push(((rm1, c), (0, 0)));
        }
    }
    if MAP[rp1][c] == '.' {
        if rp1 == 0 {
            v.push(((rp1, c), (1, 0)));
        } else {
            v.push(((rp1, c), (0, 0)));
        }
    }
    if MAP[r][cm1] == '.' {
        if c == 0 {
            v.push(((r, cm1), (0, -1)));
        } else {
            v.push(((r, cm1), (0, 0)));
        }
    }
    if MAP[r][cp1] == '.' {
        if cp1 == 0 {
            v.push(((r, cp1), (0, 1)));
        } else {
            v.push(((r, cp1), (0, 0)));
        }
    }
    v
}

type Coord = (usize, usize);
type Plane = (i32, i32);
type Planes = HashSet<Plane>;

fn sum_row(r_even: &[u8], r_odd: &[u8], r: isize, start_c: isize, count: usize) -> usize {
    let rr = [r_even, r_odd];
    if count < 8 * MAXC {
        let mut c = start_c.rem_euclid(MAXC as isize) as usize;
        let mut rri =
            ((start_c.div_floor(MAXC as isize) & 1) ^ (r.div_floor(MAXR as isize) & 1)) as usize;
        let mut sum = 0;
        for _ in 0..count {
            sum += rr[rri][c] as usize;
            c += 1;
            if c == MAXC {
                c = 0;
                rri = 1 - rri;
            }
        }
        sum
    } else {
        let mut c = start_c;
        let next = c.next_multiple_of(2 * MAXC as isize);
        let t = (next - c) as usize;
        let mut sum = sum_row(r_even, r_odd, r, c, t);
        c += t as isize;
        let count = count - t;
        let range = sum_row(r_even, r_odd, r, c, 2 * MAXC);
        let times = count / (2 * MAXC);
        sum += range * times;
        let count = count - times * 2 * MAXC;
        sum += sum_row(r_even, r_odd, r, c, count);
        sum
    }
}

fn run_steps_parity((sr, sc): (usize, usize), steps: usize) -> usize {
    let mut initial_set0 = HashMap::new();
    let mut initial_set1 = HashMap::from([((sr, sc), HashSet::from([(0, 0)]))]);
    for _ in 0..MAXR * 2 + 2 + (steps & 1) {
        let mut newset: HashMap<Coord, Planes> = HashMap::new();
        newset.clear();
        for (coord, planes) in initial_set1.iter() {
            for (new_coord, new_plane) in step1(*coord) {
                for &p in planes {
                    let p = (p.0 + new_plane.0, p.1 + new_plane.1);
                    let entry = newset.entry(new_coord).or_default();
                    entry.insert(p);
                }
            }
        }
        initial_set0 = initial_set1;
        initial_set1 = newset;
    }

    let mut cell_map_even: Vec<Vec<u8>> = vec![];
    let mut cell_map_odd: Vec<Vec<u8>> = vec![];
    for r in 0..MAXR {
        let mut v = vec![];
        let mut w = vec![];
        for c in 0..MAXC {
            if initial_set1
                .get(&(r, c))
                .unwrap_or(&HashSet::new())
                .contains(&(0, 0))
            {
                v.push(1);
            } else {
                v.push(0);
            }
            if initial_set0
                .get(&(r, c))
                .unwrap_or(&HashSet::new())
                .contains(&(0, 0))
            {
                w.push(1);
            } else {
                w.push(0);
            }
        }
        cell_map_even.push(v);
        cell_map_odd.push(w);
    }
    let sr = sr as isize;
    let sc = sc as isize;
    let steps = steps as isize;
    let mut sum = 0;
    for dr in -steps..steps + 1 {
        // if dr & 0xffff == 0 {
        //     println!("r = {}, s = {}", dr, sum);
        // }
        let left = steps - dr.abs();
        let r = sr + dr;
        let c = sc - left;
        let s = sum_row(
            &cell_map_even[r.rem_euclid(MAXR as isize) as usize],
            &cell_map_odd[r.rem_euclid(MAXR as isize) as usize],
            r,
            c,
            2 * left as usize + 1,
        );
        sum += s;
    }
    sum
}
