use std::collections::{HashMap, HashSet};

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
enum Direction {
    Right,
    Left,
    Up,
    Down,
}

pub fn day17() {
    let _inp = "2413432311323
3215453535623
3255245654254
3446585845452
4546657867536
1438598798454
4457876987766
3637877979653
4654967986887
4564679986453
1224686865563
2546548887735
4322674655533";
    let inp = std::fs::read_to_string("day17.input.txt").unwrap();
    let inp = inp.trim();

    let map: Vec<Vec<usize>> = inp
        .lines()
        .map(|x| {
            x.chars()
                .map(|y| y.to_digit(10).unwrap() as usize)
                .collect()
        })
        .collect();

    let score = find_crucible_path(&map);
    println!("score = {}", score);
}

fn find_crucible_path(map: &Vec<Vec<usize>>) -> usize {
    let endx = map[0].len() - 1;
    let endy = map.len() - 1;

    let maxx = map[0].len();
    let maxy = map.len();
    let dirs = vec![
        Direction::Down,
        Direction::Up,
        Direction::Right,
        Direction::Left,
    ];

    let mut nodes_to_ids = HashMap::new();
    let mut edges = vec![];
    let mut final_nodes = HashSet::new();

    // build a new graph with 3 history entries
    for y in 0..maxy {
        for x in 0..maxx {
            for &d1 in &dirs {
                for &d2 in &dirs {
                    if opposite(d1, d2) {
                        continue;
                    }
                    for &d3 in &dirs {
                        // trim impossible directions
                        if opposite(d2, d3) {
                            continue;
                        }
                        if x == 0 && d3 == Direction::Right && y != 0 {
                            continue;
                        }
                        if y == 0 && d3 == Direction::Down && x != 0 {
                            continue;
                        }
                        if x == maxx - 1 && d3 == Direction::Left {
                            continue;
                        }
                        if y == maxy - 1 && d3 == Direction::Up {
                            continue;
                        }
                        let src = (d1, d2, d3, x, y);
                        let len = nodes_to_ids.len();
                        let src_id = *nodes_to_ids.entry(src).or_insert_with(|| len);
                        if x == endx && y == endy {
                            final_nodes.insert(src_id);
                        }
                        let mut dests = vec![];
                        if !(d1 == Direction::Left
                            && d2 == Direction::Left
                            && d3 == Direction::Left)
                            && d3 != Direction::Right
                            && x > 0
                        {
                            let dst = (d2, d3, Direction::Left, x - 1, y);
                            dests.push(dst);
                        }
                        if !(d1 == Direction::Right
                            && d2 == Direction::Right
                            && d3 == Direction::Right)
                            && d3 != Direction::Left
                            && x < maxx - 1
                        {
                            let dst = (d2, d3, Direction::Right, x + 1, y);
                            dests.push(dst);
                        }
                        if !(d1 == Direction::Up && d2 == Direction::Up && d3 == Direction::Up)
                            && d3 != Direction::Down
                            && y > 0
                        {
                            let dst = (d2, d3, Direction::Up, x, y - 1);
                            dests.push(dst);
                        }
                        if !(d1 == Direction::Down
                            && d2 == Direction::Down
                            && d3 == Direction::Down)
                            && d3 != Direction::Up
                            && y < maxy - 1
                        {
                            let dst = (d2, d3, Direction::Down, x, y + 1);
                            dests.push(dst);
                        }
                        for dst in dests {
                            let len = nodes_to_ids.len();
                            let dst_id = *nodes_to_ids.entry(dst).or_insert_with(|| len);
                            if dst.3 == endx && dst.4 == endy {
                                final_nodes.insert(dst_id);
                            }
                            let weight = map[dst.4][dst.3];
                            edges.push((src_id, dst_id, weight));
                        }
                    }
                }
            }
        }
    }
    println!(
        "graph: {} nodes, {} edges, {} final nodes",
        nodes_to_ids.len(),
        edges.len(),
        final_nodes.len()
    );
    let mut graph: HashMap<usize, Vec<(usize, usize)>> = HashMap::new();
    for (a, b, weight) in edges {
        let e = graph.entry(a).or_default();
        if !e.contains(&(b, weight)) {
            e.push((b, weight));
        }
    }
    // Run Dijkstra twice to account for the fact that we can start going right or down.
    let s1 = *nodes_to_ids
        .get(&(Direction::Right, Direction::Right, Direction::Right, 0, 0))
        .unwrap();
    let s2 = *nodes_to_ids
        .get(&(Direction::Down, Direction::Down, Direction::Down, 0, 0))
        .unwrap();
    dijkstra(s1, &graph, &final_nodes)
        .1
        .min(dijkstra(s2, &graph, &final_nodes).1)
}

fn opposite(a: Direction, b: Direction) -> bool {
    (a == Direction::Up && b == Direction::Down)
        || (a == Direction::Down && b == Direction::Up)
        || (a == Direction::Left && b == Direction::Right)
        || (a == Direction::Right && b == Direction::Left)
}

fn dijkstra(
    start: usize,
    graph: &HashMap<usize, Vec<(usize, usize)>>,
    final_nodes: &HashSet<usize>,
) -> (Vec<usize>, usize) {
    let mut visited = HashSet::new();
    let mut dists = HashMap::new();
    let mut queue = HashSet::new();
    let mut previous: Vec<isize> = vec![-1; graph.len()];
    dists.insert(start, 0);
    queue.insert(start);

    loop {
        let next = *queue.iter().min_by_key(|k| dists.get(k)).unwrap();
        let dist = *dists.get(&next).unwrap();
        queue.remove(&next);
        if final_nodes.contains(&next) {
            let mut path = vec![];
            let mut x = next;
            path.push(x);
            loop {
                path.push(previous[x] as usize);
                x = previous[x] as usize;
                if previous[x] == -1 {
                    break;
                }
            }
            path.reverse();
            return (path, dist);
        }
        visited.insert(next);
        for (dst, weight) in graph.get(&next).unwrap().iter() {
            if visited.contains(dst) {
                continue;
            }
            queue.insert(*dst);
            let nd = dist + weight;
            if let Some(old) = dists.get(dst) {
                if nd < *old {
                    previous[*dst] = next as isize;
                    dists.insert(*dst, nd);
                }
            } else {
                previous[*dst] = next as isize;
                dists.insert(*dst, nd);
            }
        }
    }
}

pub fn day17_2() {
    let _inp = "2413432311323
3215453535623
3255245654254
3446585845452
4546657867536
1438598798454
4457876987766
3637877979653
4654967986887
4564679986453
1224686865563
2546548887735
4322674655533";
    let inp = std::fs::read_to_string("day17.input.txt").unwrap();
    let inp = inp.trim();

    let map: Vec<Vec<usize>> = inp
        .lines()
        .map(|x| {
            x.chars()
                .map(|y| y.to_digit(10).unwrap() as usize)
                .collect()
        })
        .collect();

    let score = find_ultra_path(&map);
    println!("score = {}", score);
}

fn find_ultra_path(map: &Vec<Vec<usize>>) -> usize {
    let endx = map[0].len() - 1;
    let endy = map.len() - 1;

    let maxx = map[0].len();
    let maxy = map.len();
    let dirs = vec![
        Direction::Down,
        Direction::Up,
        Direction::Right,
        Direction::Left,
    ];

    let mut nodes_to_ids = HashMap::new();
    let mut edges = vec![];
    let mut final_nodes = HashSet::new();

    // build a new graph with most recent direction and length
    for y in 0..maxy {
        for x in 0..maxx {
            for &d1 in &dirs {
                for cnt in 4..11 {
                    let src = (d1, cnt, x, y);
                    let len = nodes_to_ids.len();
                    let src_id = *nodes_to_ids.entry(src).or_insert_with(|| len);
                    if x == endx && y == endy {
                        final_nodes.insert(src_id);
                    }
                    let mut dests = vec![];
                    for &d2 in &dirs {
                        // must turn
                        if opposite(d1, d2) || d1 == d2 {
                            continue;
                        }
                        for dcnt in 4..11 {
                            match d2 {
                                Direction::Left => {
                                    if dcnt > x {
                                        continue;
                                    }
                                    let dst = (d2, dcnt, x - dcnt, y);
                                    dests.push(dst);
                                }
                                Direction::Right => {
                                    if x + dcnt >= maxx {
                                        continue;
                                    }
                                    let dst = (d2, dcnt, x + dcnt, y);
                                    dests.push(dst);
                                }
                                Direction::Up => {
                                    if dcnt > y {
                                        continue;
                                    }
                                    let dst = (d2, dcnt, x, y - dcnt);
                                    dests.push(dst);
                                }
                                Direction::Down => {
                                    if y + dcnt >= maxy {
                                        continue;
                                    }
                                    let dst = (d2, dcnt, x, y + dcnt);
                                    dests.push(dst);
                                }
                            }
                        }
                    }

                    for dst in dests {
                        let len = nodes_to_ids.len();
                        let dst_id = *nodes_to_ids.entry(dst).or_insert_with(|| len);
                        if dst.2 == endx && dst.3 == endy {
                            final_nodes.insert(dst_id);
                        }
                        let mut weight = 0;
                        for i in 1..dst.1 + 1 {
                            match dst.0 {
                                Direction::Right => {
                                    weight += map[src.3][src.2 + i];
                                }
                                Direction::Left => {
                                    weight += map[src.3][src.2 - i];
                                }
                                Direction::Up => {
                                    weight += map[src.3 - i][src.2];
                                }
                                Direction::Down => {
                                    weight += map[src.3 + i][src.2];
                                }
                            }
                        }
                        edges.push((src_id, dst_id, weight));
                    }
                }
            }
        }
    }
    println!(
        "graph: {} nodes, {} edges, {} final nodes",
        nodes_to_ids.len(),
        edges.len(),
        final_nodes.len()
    );
    let mut graph: HashMap<usize, Vec<(usize, usize)>> = HashMap::new();
    for (a, b, weight) in edges {
        let e = graph.entry(a).or_default();
        if !e.contains(&(b, weight)) {
            e.push((b, weight));
        }
    }
    let s1 = *nodes_to_ids.get(&(Direction::Right, 10, 0, 0)).unwrap();
    let s2 = *nodes_to_ids.get(&(Direction::Down, 10, 0, 0)).unwrap();

    dijkstra(s1, &graph, &final_nodes)
        .1
        .min(dijkstra(s2, &graph, &final_nodes).1)
}
