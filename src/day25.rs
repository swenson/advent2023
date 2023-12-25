use rand::prelude::*;
use std::collections::{HashMap, HashSet};

pub fn day25() {
    let inp = std::fs::read_to_string("day25.input.txt").unwrap();
    let score = part1(&inp);
    println!("part 1 score = {}", score);
}

pub fn part1(inp: &str) -> usize {
    let graph = &mut parse_input(inp);
    let mut edges: HashSet<(usize, usize)> = HashSet::new();
    for (&k, vs) in graph.iter() {
        for &v in vs.iter() {
            if k < v {
                edges.insert((k, v));
            } else {
                edges.insert((v, k));
            }
        }
    }
    let edges: Vec<_> = edges.iter().copied().collect();
    karger_contract(&edges)
}

fn karger_contract(edges: &[(usize, usize)]) -> usize {
    let original = edges;
    let mut edges: Vec<_> = edges
        .iter()
        .map(|(a, b)| (Vec::from([*a]), Vec::from([*b])))
        .collect();

    while edges.iter().collect::<HashSet<_>>().len() > 1 {
        edges = contract(&edges);
    }
    println!("Contracted to {} edges", edges.len());
    if edges.len() == 3 {
        edges[0].0.len() * edges[0].1.len()
    } else {
        println!("Trying again");
        karger_contract(original)
    }
}

fn contract(edges: &[(Vec<usize>, Vec<usize>)]) -> Vec<(Vec<usize>, Vec<usize>)> {
    let i = rand::thread_rng().gen_range(0..edges.len());
    let a = &(edges[i].0.clone());
    let b = &(edges[i].1.clone());
    let mut n: Vec<_> = a.clone();
    n.extend(b);
    n.sort();
    let mut new_edges = vec![];
    for (e0, e1) in edges {
        if (e0 == a && e1 == b) || (e0 == b && e1 == a) {
            continue;
        } else if e0 == a || e0 == b {
            new_edges.push((n.clone(), e1.clone()));
        } else if e1 == a || e1 == b {
            new_edges.push((n.clone(), e0.clone()));
        } else {
            new_edges.push((e0.clone(), e1.clone()));
        }
    }
    new_edges
}

fn parse_input(inp: &str) -> HashMap<usize, HashSet<usize>> {
    let inp = inp.trim();
    let lines: Vec<_> = inp.lines().collect();
    let mut map = HashMap::new();
    let mut nodes = HashMap::new();
    for line in lines {
        let sides: Vec<_> = line.split(':').collect();
        let a = String::from(sides[0]);
        let a_id = if nodes.contains_key(&a) {
            *nodes.get(&a).unwrap()
        } else {
            let a_id = nodes.len();
            nodes.insert(a, a_id);
            a_id
        };
        let others: Vec<_> = sides[1].trim().split(' ').collect();
        let amap: &mut HashSet<usize> = map.entry(a_id).or_default();
        let bs: Vec<_> = others
            .iter()
            .map(|other| {
                let other = String::from(*other);
                let b_id = if nodes.contains_key(&other) {
                    *nodes.get(&other).unwrap()
                } else {
                    let b_id = nodes.len();
                    nodes.insert(other, b_id);
                    b_id
                };
                amap.insert(b_id);
                b_id
            })
            .collect();
        bs.iter().for_each(|b_id| {
            map.entry(*b_id).or_default().insert(a_id);
        });
    }
    map
}

pub fn part2(inp: &str) -> usize {
    let _inp = inp.trim();
    let _lines: Vec<_> = inp.lines().collect();
    0
}

pub fn day25_2() {
    let inp = std::fs::read_to_string("day25.input.txt").unwrap();
    let score = part2(&inp);
    println!("part 2 score = {}", score);
}

#[cfg(test)]
mod test {
    use crate::day25::{part1, part2};

    const TEST_INPUT: &str = "jqt: rhn xhk nvd
rsh: frs pzl lsr
xhk: hfx
cmg: qnr nvd lhk bvb
rhn: xhk bvb hfx
bvb: xhk hfx
pzl: lsr hfx nvd
qnr: nvd
ntq: jqt hfx bvb xhk
nvd: lhk
lsr: lhk
rzs: qnr cmg lsr rsh
frs: qnr lhk lsr";

    #[test]
    fn test_25_1() {
        assert_eq!(54, part1(TEST_INPUT));
    }
    #[test]
    fn test_25_2() {
        let inp = "";
        assert_eq!(0, part2(inp));
    }
}
