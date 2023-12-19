pub fn day18() {
    let _inp = "R 6 (#70c710)
D 5 (#0dc571)
L 2 (#5713f0)
D 2 (#d2c081)
R 2 (#59c680)
D 2 (#411b91)
L 5 (#8ceee2)
U 2 (#caa173)
L 1 (#1b58a2)
U 2 (#caa171)
R 2 (#7807d2)
U 3 (#a77fa3)
L 2 (#015232)
U 2 (#7a21e3)";
    let inp = std::fs::read_to_string("day18.input.txt").unwrap();
    let inp = inp.trim();

    let lines: Vec<_> = inp.lines().collect();
    let points = parse_polygon(&lines);
    let area = calculate_area_efficient(&points);

    println!("area = {}", area);
}

pub fn day18_2() {
    let _inp = "R 6 (#70c710)
D 5 (#0dc571)
L 2 (#5713f0)
D 2 (#d2c081)
R 2 (#59c680)
D 2 (#411b91)
L 5 (#8ceee2)
U 2 (#caa173)
L 1 (#1b58a2)
U 2 (#caa171)
R 2 (#7807d2)
U 3 (#a77fa3)
L 2 (#015232)
U 2 (#7a21e3)";
    let inp = std::fs::read_to_string("day18.input.txt").unwrap();
    let inp = inp.trim();

    let lines: Vec<_> = inp.lines().collect();

    let points = parse_polygon_swapped(&lines);
    let area = calculate_area_efficient(&points);

    println!("area = {}", area);
}

fn calculate_area_efficient(points: &[(isize, isize)]) -> usize {
    // thanks https://stackoverflow.com/a/451482/10524
    let mut sum = 0;
    let mut perimeter = 0;
    for ab in points.windows(2) {
        let (x0, y0) = ab[0];
        let (x1, y1) = ab[1];
        sum += x0 * y1 - x1 * y0;
        perimeter += (x1 - x0).unsigned_abs() + (y1 - y0).unsigned_abs();
    }
    sum.unsigned_abs() / 2 + perimeter / 2 + 1
}

fn parse_polygon(lines: &[&str]) -> Vec<(isize, isize)> {
    let mut p = vec![(0, 0)];
    for &line in lines.iter() {
        let parts: Vec<&str> = line.split_whitespace().collect();
        let count = parts[1].parse::<isize>().unwrap();
        let dir = parts[0];
        let (x, y) = *p.last().unwrap();
        let next = match dir {
            "R" => (x + count, y),
            "D" => (x, y + count),
            "L" => (x - count, y),
            "U" => (x, y - count),
            _ => panic!("Unknown direction {}", dir),
        };
        p.push(next);
    }
    p
}

fn parse_polygon_swapped(lines: &[&str]) -> Vec<(isize, isize)> {
    let mut p = vec![(0, 0)];
    for &line in lines.iter() {
        let parts: Vec<&str> = line.split_whitespace().collect();
        let count = isize::from_str_radix(&parts[2][2..7], 16).unwrap();
        let dir = parts[2].as_bytes()[7];
        let (x, y) = *p.last().unwrap();

        let next = match dir {
            49 => (x, y + count),
            48 => (x + count, y),
            50 => (x - count, y),
            51 => (x, y - count),
            _ => panic!("Unknown direction {}", dir),
        };
        p.push(next);
    }
    p
}
