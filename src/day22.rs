use std::collections::HashSet;
use std::sync::Arc;

const THREADS: usize = 32;

pub fn day22() {
    let inp = "1,0,1~1,2,1
0,0,2~2,0,2
0,2,3~2,2,3
0,0,4~0,2,4
2,0,5~2,2,5
0,1,6~2,1,6
1,1,8~1,1,9";
    let _inp = std::fs::read_to_string("day22.input.txt").unwrap();
    let inp = inp.trim();
    let lines: Vec<_> = inp.lines().collect();

    let s = score(&lines);
    println!("score = {}", s);
}

fn score(lines: &[&str]) -> usize {
    let bricks = parse_bricks(lines);
    let bricks = collapse_all(&bricks);

    let mut score = 0;
    for i in 0..bricks.len() {
        if can_disintegrate(&bricks, i) {
            score += 1;
        }
    }
    score
}

fn can_disintegrate(bricks: &[Brick], b: usize) -> bool {
    let new_bricks: Vec<_> = bricks
        .iter()
        .enumerate()
        .filter(|&(i, _)| i != b)
        .map(|(_, brick)| brick.clone())
        .collect();
    collapse1(&new_bricks) == new_bricks
}

fn collapse_all(bricks: &[Brick]) -> Vec<Brick> {
    let mut last_bricks = vec![];
    let mut bricks = Vec::from(bricks);
    while bricks != last_bricks {
        last_bricks = bricks.clone();
        bricks = collapse1(&bricks);
    }
    bricks
}

fn collapse1(bricks: &[Brick]) -> Vec<Brick> {
    let mut new_bricks = vec![];
    for i in 0..bricks.len() {
        let b = &bricks[i];
        if b.on_ground() {
            new_bricks.push(b.clone());
            continue;
        }
        let mut supported = false;
        for (j, c) in bricks.iter().enumerate() {
            if j == i {
                continue;
            }
            if b.on_top_of(c) {
                supported = true;
                break;
            }
        }
        if supported {
            new_bricks.push(b.clone());
        } else {
            new_bricks.push(b.drop1());
        }
    }
    new_bricks
}

#[derive(Clone, Debug, Eq, PartialEq)]
struct Brick {
    a: Point,
    b: Point,
}

#[derive(Clone, Debug, Eq, PartialEq, Hash)]
struct Point {
    x: i32,
    y: i32,
    z: i32,
}

impl Point {
    fn on_top_of(&self, q: &Point) -> bool {
        self.x == q.x && self.y == q.y && self.z == q.z + 1
    }
}

impl Brick {
    fn on_ground(&self) -> bool {
        self.a.z == 1 || self.b.z == 1
    }
    fn drop1(&self) -> Brick {
        Brick {
            a: Point {
                x: self.a.x,
                y: self.a.y,
                z: self.a.z - 1,
            },
            b: Point {
                x: self.b.x,
                y: self.b.y,
                z: self.b.z - 1,
            },
        }
    }
    fn points(&self) -> Vec<Point> {
        if self.a == self.b {
            vec![self.a.clone()]
        } else {
            let mut points = vec![];
            let x0 = self.a.x.min(self.b.x);
            let x1 = self.a.x.max(self.b.x);
            let y0 = self.a.y.min(self.b.y);
            let y1 = self.a.y.max(self.b.y);
            let z0 = self.a.z.min(self.b.z);
            let z1 = self.a.z.max(self.b.z);
            for x in x0..x1 + 1 {
                for y in y0..y1 + 1 {
                    for z in z0..z1 + 1 {
                        points.push(Point { x, y, z });
                    }
                }
            }
            points
        }
    }
    fn on_top_of(&self, b: &Brick) -> bool {
        let ap = self.points();
        let bp = b.points();
        for x in ap {
            for y in &bp {
                if x.on_top_of(y) {
                    return true;
                }
            }
        }
        false
    }
}

fn parse_bricks(lines: &[&str]) -> Vec<Brick> {
    let mut bricks = vec![];
    for &l in lines.iter() {
        let parts: Vec<_> = l.split('~').collect();
        let xyz: Vec<_> = parts[0]
            .split(',')
            .map(|x| x.parse::<i32>().unwrap())
            .collect();
        let a = Point {
            x: xyz[0],
            y: xyz[1],
            z: xyz[2],
        };
        let xyz: Vec<_> = parts[1]
            .split(',')
            .map(|x| x.parse::<i32>().unwrap())
            .collect();
        let b = Point {
            x: xyz[0],
            y: xyz[1],
            z: xyz[2],
        };
        bricks.push(Brick { a, b });
    }
    bricks
}

pub fn day22_2() {
    let _inp = "1,0,1~1,2,1
0,0,2~2,0,2
0,2,3~2,2,3
0,0,4~0,2,4
2,0,5~2,2,5
0,1,6~2,1,6
1,1,8~1,1,9";
    let inp = std::fs::read_to_string("day22.input.txt").unwrap();
    let inp = inp.trim();
    let lines: Vec<_> = inp.lines().collect();

    let s = score_chain(&lines);
    println!("score = {}", s);
}

fn score_chain(lines: &[&str]) -> usize {
    let bricks = parse_bricks(lines);
    let bricks = collapse_all(&bricks);
    let bcopy = Arc::new(bricks.clone());

    crossbeam::scope(|s| {
        let mut sums = vec![];
        for t in 0..THREADS {
            let bcopy = bcopy.clone();
            let s = s.spawn(move |_| {
                let mut sum = 0;
                for i in (t..bcopy.len()).step_by(THREADS) {
                    sum += chain_score(&bcopy, i);
                }
                sum
            });
            sums.push(s);
        }
        let mut sum = 0;
        for s in sums {
            sum += s.join().unwrap();
        }
        sum
    })
    .unwrap()
}

fn chain_score(bricks: &[Brick], b: usize) -> usize {
    let new_bricks: Vec<_> = bricks
        .iter()
        .enumerate()
        .filter(|&(i, _)| i != b)
        .map(|(_, brick)| brick.clone())
        .collect();
    count_collapse(&new_bricks)
}

fn count_collapse(bricks: &[Brick]) -> usize {
    let mut last_bricks = vec![];
    let mut bricks = Vec::from(bricks);
    let mut all_dropped = HashSet::new();
    while bricks != last_bricks {
        last_bricks = bricks.clone();
        let (nb, dropped) = collapse1_d(&bricks);
        all_dropped.extend(dropped);
        bricks = nb;
    }
    all_dropped.len()
}

fn collapse1_d(bricks: &[Brick]) -> (Vec<Brick>, HashSet<usize>) {
    let mut new_bricks = vec![];
    let mut d = HashSet::new();
    for i in 0..bricks.len() {
        let b = &bricks[i];
        if b.on_ground() {
            new_bricks.push(b.clone());
            continue;
        }
        let mut supported = false;
        for (j, c) in bricks.iter().enumerate() {
            if j == i {
                continue;
            }
            if b.on_top_of(c) {
                supported = true;
                break;
            }
        }
        if supported {
            new_bricks.push(b.clone());
        } else {
            new_bricks.push(b.drop1());
            d.insert(i);
        }
    }
    (new_bricks, d)
}
