use memoize::lazy_static::lazy_static;
use std::collections::HashSet;

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
    //let count = reachable_wrap(&lines, 6);
    println!("count = {}", count);
}

fn reachable_wrap(map: &Vec<Vec<char>>, steps: usize) -> usize {
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
    let _map = newmap;
    let _set = vec![(start, vec![(0, 0)])];
    //run_steps_wrap(set, steps).iter().map(|(_, x)| x.len()).sum::<usize>()
    //solve_fast(&map, start, steps)
    //hash_solve(&map, start, steps)
    run_steps_parity(start, steps)
}

// type Coord = (i32, i32);
//
// struct Cache {
//     entries: Vec<MacroCell>,
//     map: HashMap<MacroCell, usize>,
//     results: HashMap<usize, usize>,
// }
//
// #[derive(Clone, Eq, PartialEq, Hash)]
// struct MacroCell {
//     size: usize,
//     ul: usize,
//     ur: usize,
//     ll: usize,
//     lr: usize,
//     alive: bool,
// }

// impl Cache {
//
//     fn result(&mut self, mc: usize) -> usize {
//         if self.results.contains_key(&mc) {
//             self.results.get(&mc).unwrap()
//         } else {
//             self.calc_result(mc)
//         }
//     }
//     fn calc_result(&mut self, mci: usize) -> usize {
//         let mc = &self.entries[mci];
//         if mc.size == 4
//     }
//     fn make_macro_cell(&mut self, size: usize, (x, y): Coord) -> usize {
//         println!("make {}, {}, {}", size, x, y);
//         if size == 1 {
//             assert_eq!(x, 0);
//             assert_eq!(y, 0);
//             return 1;
//         }
//         let hsize = size as i32 / 4;
//         let mc = if x >= 0 && y >= 0 {
//             MacroCell {
//                 size,
//                 ul: self.make_empty_macro_cell(size/2),
//                 ur: self.make_macro_cell(size/2, (x - hsize, y-hsize)),
//                 ll: self.make_empty_macro_cell(size/2),
//                 lr: self.make_empty_macro_cell(size/2),
//                 alive: false,
//             }
//         } else if x < 0 && y >= 0 {
//             MacroCell {
//                 size,
//                 ul: self.make_macro_cell(size/2, (x + hsize, y-hsize)),
//                 ur: self.make_empty_macro_cell(size/2),
//                 ll: self.make_empty_macro_cell(size/2),
//                 lr: self.make_empty_macro_cell(size/2),
//                 alive: false,
//             }
//         } else if x >= 0 && y < 0{
//             MacroCell {
//                 size,
//                 ul: self.make_empty_macro_cell(size/2),
//                 ur: self.make_empty_macro_cell(size/2),
//                 ll: self.make_empty_macro_cell(size/2),
//                 lr: self.make_macro_cell(size/2, (x - hsize, y+hsize)),
//                 alive: false,
//             }
//         } else {
//             MacroCell {
//                 size,
//                 ul: self.make_empty_macro_cell(size/2),
//                 ur: self.make_empty_macro_cell(size/2),
//                 ll: self.make_macro_cell(size/2, (x + hsize, y+hsize)),
//                 lr: self.make_empty_macro_cell(size/2),
//                 alive: false,
//             }
//         };
//         self.make(mc)
//     }
//
//     fn make(&mut self, mc: MacroCell) -> usize {
//         if self.map.contains_key(&mc) {
//             *self.map.get(&mc).unwrap()
//         } else {
//             let x = self.entries.len();
//             self.map.insert(mc.clone(), x);
//             self.entries.push(mc);
//             x
//         }
//     }
//
//     fn make_empty_macro_cell(&mut self, size: usize) -> usize {
//         if size == 1 {
//             return 0;
//         }
//         let h = self.make_empty_macro_cell(size/2);
//         let mc = MacroCell {
//             size: size,
//             ul: h,
//             ur: h,
//             ll: h,
//             lr: h,
//             alive: false,
//         };
//         self.make(mc)
//     }
// }
//
//
// fn hash_solve(map: &Vec<Vec<char>>, (sr, sc): (usize, usize), steps: usize) -> usize {
//     let mut cache = Cache { entries: vec![], map: HashMap::new(), results: HashMap::new() };
//     cache.make(MacroCell {
//         size: 1,
//         ul: 0,
//         ur: 0,
//         ll: 0,
//         lr: 0,
//         alive: false,
//     });
//     cache.make(MacroCell {
//         size: 1,
//         ul: 0,
//         ur: 0,
//         ll: 0,
//         lr: 0,
//         alive: true,
//     });
//     // cache.entries.push(Vec::new());
//     // cache.entries.push(vec![(sr as i32, sc as i32)]);
//     // cache.set.insert(Vec::new());
//     // cache.set.insert(vec![(sr as i32, sc as i32)]);
//     // let zero = Rc::new(QuadTree::Entry(0));
//     // let one = Rc::new(QuadTree::Entry(1));
//     // let mut tree = QuadTree::Node(zero.clone(), zero.clone(), zero.clone(), one.clone());
//
//     let mut state = cache.make_macro_cell(steps.next_power_of_two() * 2 * 2, (sr as i32, sc as i32));
//     cache.calculate_result(state);
//     //let mut state = cache.make_macro_cell(64, (1, 1));
//     println!("{:?}", state);
//     0
// }

// fn solve_fast(map: &Vec<Vec<char>>, (sr, sc): (usize, usize), steps: usize) -> usize {
//     let mut ffmap = HashMap::new();
//     println!("start = {}, {}", sr, sc);
//     println!("precomputing caches...");
//     let mut stepmap = HashMap::new();
//     for r in 0..map.len() {
//         for c in 0..map[0].len() {
//             stepmap.insert((r, c), step1(map, (r, c)));
//             let ff = fast_forward(map, (r, c));
//             ffmap.insert((r, c), ff);
//         }
//     }
//     println!("precomputing caches done");
//
//     let x = ((sr, sc), vec![(0, 0)]);
//     solve_fast_recurse(&stepmap, &ffmap, vec![x], steps)
// }

// fn solve_fast_recurse(stepmap: &HashMap<(usize, usize), Vec<((usize, usize), (i32, i32))>>,
//                       ffmap: &HashMap<(usize, usize), (usize, Vec<(usize, usize)>)>,
//                       coords: Vec<((usize, usize), Vec<(i32, i32)>)>,
//                       steps: usize) -> usize {
//
// }
//
// fn fast_forward(map: &Vec<Vec<char>>, (sr, sc): (usize, usize)) -> (usize, Vec<(usize, usize)>) {
//     let mut ret = vec![(sr, sc)];
//     let mut steps = 0;
//     loop {
//         let mut newset: HashSet<(usize, usize)> = HashSet::new();
//         for &(r, c) in ret.iter() {
//             let s = step1(map, (r, c));
//             for &((r, c), (x, y)) in s.iter() {
//                 if x != 0 || y != 0 {
//                     return (steps, ret);
//                 }
//                 newset.insert((r, c));
//             }
//         }
//         if newset.is_empty() {
//             return (0, vec![]);
//         }
//         steps += 1;
//         if steps > map[0].len() * map.len() {
//             return (0, vec![]);
//         }
//         ret = newset.iter().copied().collect();
//     }
// }

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
//
// fn run_steps_wrap(map: &Vec<Vec<char>>, start: (usize, usize), steps: usize) -> usize {
//     let maxc = map[0].len() - 1;
//     let maxr = map.len() - 1;
//     let mut set = HashMap::new();
//     set.insert((start.0, start.1), HashSet::from([(0i32, 0i32)]));
//     for s in 0..steps {
//         let mut newset = HashMap::new();
//         for (&(r, c), v) in set.iter() {
//             let rm1 = if r > 0 { r - 1 } else { maxr };
//             let rp1 = if r < maxr { r + 1 } else { 0 };
//             let cm1 = if c > 0 { c - 1 } else { maxc };
//             let cp1 = if c < maxc { c + 1 } else { 0 };
//             if map[rm1][c] == '.' {
//                 let s: &mut HashSet<(i32, i32)> = newset.entry((rm1,c)).or_default();
//                 if r == 0 {
//                     s.extend(v.iter().map(|&(x, y)| (x-1,y)));
//                 } else {
//                     s.extend(v);
//                 }
//             }
//             if map[rp1][c] == '.' {
//                 let s = newset.entry((rp1,c)).or_default();
//                 if rp1 == 0 {
//                     s.extend(v.iter().map(|&(x, y)| (x+1,y)));
//                 } else {
//                     s.extend(v);
//                 }
//             }
//             if map[r][cm1] == '.' {
//                 let s = newset.entry((r,cm1)).or_default();
//                 if c == 0 {
//                     s.extend(v.iter().map(|&(x, y)| (x,y-1)));
//                 } else {
//                     s.extend(v);
//                 }
//             }
//             if map[r][cp1] == '.' {
//                 let s = newset.entry((r,cp1)).or_default();
//                 if cp1 == 0 {
//                     s.extend(v.iter().map(|&(x, y)| (x,y+1)));
//                 } else {
//                     s.extend(v);
//                 }
//             }
//         }
//         set = newset;
//
//         if s & 0xff == 0 {
//             println!("step {} size {}", s, set.values().map(|v| v.len()).sum::<usize>());
//             set.values().take(1).for_each(|x| println!("{:?}", x));
//         }
//     }
//     set.values().map(|v| v.len()).sum()
//
//     // } else if steps % 2 == 0 {
//     //     run_steps_wrap(map, &run_steps_wrap(map, set, steps / 2), steps / 2)
//     // } else {
//     //     run_steps_wrap(map, &run_steps_wrap(map, set, steps - 1), 1)
//     // }
// }
//
type Coord = (usize, usize);
type Plane = (i32, i32);
type Planes = HashSet<Plane>;
//
// #[memoize]
// fn run_steps_wrap(initial_set: Vec<(Coord, Vec<Plane>)>, steps: usize) -> Vec<(Coord, Vec<Plane>)> {
//     if steps == 1 {
//         let mut newset: HashMap<Coord, Planes> = HashMap::new();
//         for (coord, planes) in initial_set.iter() {
//             for (new_coord, new_plane) in step1(*coord) {
//                 for &p in planes {
//                     let p = (p.0 + new_plane.0, p.1 + new_plane.1);
//                     let entry = newset.entry(new_coord).or_default();
//                     entry.insert(p);
//                 }
//             }
//         }
//         let mut ret = vec![];
//         for (&coord, planes) in newset.iter() {
//             ret.push((coord, planes.iter().copied().collect()))
//         }
//         ret
//     } else if steps % 2 == 0 {
//         let x = run_steps_wrap(run_steps_wrap(initial_set, steps / 2), steps / 2);
//         println!("computed steps {} => {}", steps, x.iter().map(|(_, x)| x.len()).sum::<usize>());
//         x
//     } else {
//         run_steps_wrap(run_steps_wrap(initial_set, 1), steps - 1)
//     }
// }

fn sum_times(r: usize, mut c: usize, sr: usize, sc: usize, count: usize) -> usize {
    let mut parity = (r - sr + c - sc) & 1;
    let maxc = MAP[0].len();
    if count < 8 * maxc {
        let mut sum = 0;
        for _ in 0..count {
            parity ^= 1;
            if MAP[r][c % maxc] != '#' {
                sum += parity;
            }
            c += 1;
            if c == maxc {
                c = 0;
            }
        }
        sum
    } else {
        let next = c.next_multiple_of(2 * maxc);
        let t = next - c;
        let mut sum = sum_times(r, c, sr, sc, t);
        c += t;
        assert_eq!(0, c % (2 * maxc));
        let count = count - t;
        let range = sum_times(r, c, sr, sc, 2 * maxc);
        let times = count / (2 * maxc);
        sum += range * times;
        let count = count - times * 2 * maxc;
        sum += sum_times(r, c, sr, sc, count);
        sum
    }
}

fn run_steps_parity((sr, sc): (usize, usize), steps: usize) -> usize {
    let maxr = MAP.len() as isize;
    let maxc = MAP[0].len();
    let sr = sr as isize;
    let sc = sc as isize;
    let steps = steps as isize;
    let mut sum = 0;
    for dr in -steps..steps + 1 {
        if dr & 0xffff == 0 {
            println!("r = {}, s = {}", dr, sum);
        }
        let mc = steps - dr.abs();
        let _parity = (dr - mc - sr - sc) & 1;
        let r = (dr - sr).rem_euclid(maxr) as usize;
        let c = (-mc - sc).rem_euclid(maxc as isize) as usize;
        let s = sum_times(r, c, sr as usize, sc as usize, 2 * mc as usize);
        //println!("r = {}, c = {} ..= {}, += {}", r, c, c + 2 * mc as usize, s);
        sum += s;
    }
    sum
    // let mut initial_set = HashMap::from([((sr, sc), HashSet::from([(0, 0)]))]);
    // for _ in 0..200 {
    //     let mut newset: HashMap<Coord, Planes> = HashMap::new();
    //     newset.clear();
    //     for (coord, planes) in initial_set.iter() {
    //         for (new_coord, new_plane) in step1(*coord) {
    //             for &p in planes {
    //                 let p = (p.0 + new_plane.0, p.1 + new_plane.1);
    //                 let entry = newset.entry(new_coord).or_default();
    //                 entry.insert(p);
    //             }
    //         }
    //     }
    //     initial_set = newset;
    // }
    // let mut screen = MAP.clone();
    // for (&coord, planes) in initial_set.iter() {
    //     if planes.contains(&(0,0)) {
    //         screen[coord.0][coord.1] = 'O';
    //     }
    // }
    // let mut os = 0;
    // for (i, x) in screen.iter().enumerate() {
    //     println!("{}", x.iter().collect::<String>());
    //     for (j, y) in x.iter().enumerate() {
    //         if *y == 'O' {
    //             assert_eq!(0, (i - sr + j - sc) & 1);
    //         }
    //     }
    //     os += x.iter().filter(|&&y| y == 'O').count()
    // }
    // println!("{}", os);
}
