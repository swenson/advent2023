use std::collections::HashSet;

pub fn day23() {
    let _inp = "#.#####################
#.......#########...###
#######.#########.#.###
###.....#.>.>.###.#.###
###v#####.#v#.###.#.###
###.>...#.#.#.....#...#
###v###.#.#.#########.#
###...#.#.#.......#...#
#####.#.#.#######.#.###
#.....#.#.#.......#...#
#.#####.#.#.#########v#
#.#...#...#...###...>.#
#.#.#v#######v###.###v#
#...#.>.#...>.>.#.###.#
#####v#.#.###v#.#.###.#
#.....#...#...#.#.#...#
#.#########.###.#.#.###
#...###...#...#...#.###
###.###.#.###v#####v###
#...#...#.#.>.>.#.>.###
#.###.###.#.###.#.#v###
#.....###...###...#...#
#####################.#";
    let inp = std::fs::read_to_string("day23.input.txt").unwrap();
    let inp = inp.trim();
    let lines: Vec<_> = inp.lines().map(|x| x.chars().collect::<Vec<_>>()).collect();

    let s = score(&lines);
    println!("score = {}", s);
}

fn score(map: &[Vec<char>]) -> usize {
    let start_c = map[0]
        .iter()
        .enumerate()
        .filter(|x| *x.1 == '.')
        .map(|x| x.0)
        .take(1)
        .collect::<Vec<_>>()[0];

    dfs(map, HashSet::from([(0, start_c)]), (0, start_c))
}

fn dfs(map: &[Vec<char>], mut visited: HashSet<(usize, usize)>, p: (usize, usize)) -> usize {
    let r = p.0;
    let c = p.1;
    let mut moves = vec![];
    if r > 0 && map[r - 1][c] != '#' {
        moves.push((r - 1, c));
    }
    if r < map.len() - 1 && map[r + 1][c] != '#' {
        moves.push((r + 1, c));
    }
    if c > 0 && map[r][c - 1] != '#' {
        moves.push((r, c - 1));
    }
    if c < map[0].len() - 1 && map[r][c + 1] != '#' {
        moves.push((r, c + 1));
    }
    moves.retain(|x| !visited.contains(x));
    // one choice, so just loop
    if moves.len() == 1 && map[moves[0].0][moves[0].1] == '.' {
        visited.insert(moves[0]);
        return 1 + dfs(map, visited, moves[0]);
    }

    let mut max = 0;
    for m in moves {
        let mut v = visited.clone();
        match map[m.0][m.1] {
            '.' => {
                v.insert(m);
                max = max.max(1 + dfs(map, v, m));
            }
            _ => {
                if let Some(steps) = can_step(map, &v, m) {
                    let next = *steps.last().unwrap();
                    let l = steps.len();
                    for m in steps {
                        v.insert(m);
                    }
                    max = max.max(1 + l - 1 + dfs(map, v, next));
                }
            }
        }
    }
    max
}

fn can_step(
    map: &[Vec<char>],
    visited: &HashSet<(usize, usize)>,
    m: (usize, usize),
) -> Option<Vec<(usize, usize)>> {
    if !((0..map.len()).contains(&m.0) && (0..map[0].len()).contains(&m.1)) || visited.contains(&m)
    {
        None
    } else if map[m.0][m.1] == '.' {
        Some(vec![m])
    } else {
        let mut v2 = visited.clone();
        v2.insert(m);
        let m2 = match map[m.0][m.1] {
            '>' => (m.0, m.1 + 1),
            '<' => (m.0, m.1 - 1),
            '^' => (m.0 - 1, m.1),
            'v' => (m.0 + 1, m.1),
            _ => panic!("unknown direction {}", map[m.0][m.1]),
        };
        can_step(map, &v2, m2).map(|ms| {
            let mut nms = vec![];
            nms.push(m);
            nms.extend(&ms);
            nms
        })
    }
}

pub fn day23_2() {
    let _inp = "#.#####################
#.......#########...###
#######.#########.#.###
###.....#.>.>.###.#.###
###v#####.#v#.###.#.###
###.>...#.#.#.....#...#
###v###.#.#.#########.#
###...#.#.#.......#...#
#####.#.#.#######.#.###
#.....#.#.#.......#...#
#.#####.#.#.#########v#
#.#...#...#...###...>.#
#.#.#v#######v###.###v#
#...#.>.#...>.>.#.###.#
#####v#.#.###v#.#.###.#
#.....#...#...#.#.#...#
#.#########.###.#.#.###
#...###...#...#...#.###
###.###.#.###v#####v###
#...#...#.#.>.>.#.>.###
#.###.###.#.###.#.#v###
#.....###...###...#...#
#####################.#";
    let inp = std::fs::read_to_string("day23.input.txt").unwrap();
    let inp = inp.trim().replace(['>', '<', '^', 'v'], ".");
    let lines: Vec<_> = inp.lines().map(|x| x.chars().collect::<Vec<_>>()).collect();

    let s = score_ignore(&lines);
    println!("score = {}", s);
}

fn find_first(v: &[char], needle: char) -> usize {
    v.iter().collect::<String>().find(needle).unwrap()
}

fn score_ignore(map: &[Vec<char>]) -> usize {
    let start_c = find_first(&map[0], '.');
    let start = (0, start_c);
    let target_c = find_first(&map[map.len() - 1], '.');
    let target = (map.len() - 1, target_c);
    dfs_ignore(map, target, HashSet::from([start]), start, 0)
}

fn dfs_ignore(
    map: &[Vec<char>],
    target: (usize, usize),
    mut visited: HashSet<(usize, usize)>,
    p: (usize, usize),
    s: usize,
) -> usize {
    if p == target {
        return s;
    }
    let r = p.0;
    let c = p.1;
    let mut moves = vec![];
    if r > 0 && map[r - 1][c] != '#' {
        moves.push((r - 1, c));
    }
    if r < map.len() - 1 && map[r + 1][c] != '#' {
        moves.push((r + 1, c));
    }
    if c > 0 && map[r][c - 1] != '#' {
        moves.push((r, c - 1));
    }
    if c < map[0].len() - 1 && map[r][c + 1] != '#' {
        moves.push((r, c + 1));
    }
    moves.retain(|x| !visited.contains(x));
    if moves.is_empty() {
        return 0;
    } else if moves.len() == 1 {
        visited.insert(moves[0]);
        return dfs_ignore(map, target, visited, moves[0], s + 1);
    }

    let mut max = 0;
    for m in moves {
        let mut v = visited.clone();
        v.insert(m);
        max = max.max(dfs_ignore(map, target, v, m, s + 1));
    }
    max
}
