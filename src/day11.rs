pub fn day11() {
    let _inp = "...#......
.......#..
#.........
..........
......#...
.#........
.........#
..........
.......#..
#...#.....";
    let inp = std::fs::read_to_string("day11.input.txt").unwrap();
    let inp = inp.trim();

    let mut rows = vec![];

    for line in inp.lines() {
        // expand rows
        rows.push(line.chars().collect::<Vec<char>>());
        if line.chars().all(|x| x == '.') {
            rows.push(line.chars().collect::<Vec<char>>());
        }
    }
    for c in (0..rows[0].len()).rev() {
        let mut all = true;
        for r in &rows {
            all = all && r[c] == '.';
        }
        if all {
            for r in &mut rows {
                r.insert(c, '.');
            }
        }
    }

    let mut galaxies = vec![];

    for r in 0..rows.len() {
        for c in 0..rows[0].len() {
            if rows[r][c] == '#' {
                galaxies.push((r, c));
            }
        }
    }
    let mut total = 0isize;
    for i in 0..galaxies.len() {
        let (a, b) = galaxies.get(i).unwrap();
        let a = *a as isize;
        let b = *b as isize;
        for j in (i + 1)..galaxies.len() {
            let (c, d) = galaxies.get(j).unwrap();
            let c = *c as isize;
            let d = *d as isize;
            total += (a - c).abs() + (b - d).abs()
        }
    }
    println!("total = {}", total);

    day11_2();
}

pub fn day11_2() {
    let _inp = "...#......
.......#..
#.........
..........
......#...
.#........
.........#
..........
.......#..
#...#.....";
    let inp = std::fs::read_to_string("day11.input.txt").unwrap();
    let inp = inp.trim();

    let mut rows = vec![];
    let mult = 1000000;
    let mut expanded_rows = vec![];
    let mut expanded_cols = vec![];
    for (r, line) in inp.lines().enumerate() {
        // expand rows
        rows.push(line.chars().collect::<Vec<char>>());
        if line.chars().all(|x| x == '.') {
            expanded_rows.push(r);
        }
    }
    for c in (0..rows[0].len()).rev() {
        let mut all = true;
        for r in &rows {
            all = all && r[c] == '.';
        }
        if all {
            expanded_cols.push(c);
        }
    }

    let mut galaxies = vec![];

    for r in 0..rows.len() {
        for c in 0..rows[0].len() {
            if rows[r][c] == '#' {
                galaxies.push((r, c));
            }
        }
    }
    let mut total = 0isize;
    for i in 0..galaxies.len() {
        let (a, b) = galaxies.get(i).unwrap();
        let a = *a;
        let b = *b;
        for j in (i + 1)..galaxies.len() {
            let (c, d) = galaxies.get(j).unwrap();
            let c = *c;
            let d = *d;
            let mut dist = 0;
            for x in a.min(c)..a.max(c) {
                if expanded_rows.contains(&x) {
                    dist += mult;
                } else {
                    dist += 1;
                }
            }
            for y in b.min(d)..b.max(d) {
                if expanded_cols.contains(&y) {
                    dist += mult;
                } else {
                    dist += 1;
                }
            }
            total += dist;
        }
    }
    println!("total = {}", total);
}
