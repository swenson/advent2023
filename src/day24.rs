use num_bigint::BigInt;
use num_integer::Integer;
use num_traits::identities::Zero;
use num_traits::ToPrimitive;
use std::ops::RangeInclusive;

pub fn day24() {
    let _inp = "19, 13, 30 @ -2,  1, -2
18, 19, 22 @ -1, -1, -2
20, 25, 34 @ -2, -2, -4
12, 31, 28 @ -1, -2, -1
20, 19, 15 @  1, -5, -3";
    let inp = std::fs::read_to_string("day24.input.txt").unwrap();
    let inp = inp.trim();
    let lines: Vec<_> = inp.lines().collect();

    let particles = parse_particles(&lines);

    //let s = intersects(&particles, 7.0..=27.0, 7.0..=27.0);
    let s = intersects_xy(
        &particles,
        200000000000000.0..=400000000000000.0,
        200000000000000.0..=400000000000000.0,
    );
    println!("score = {}", s);
}

fn intersects_xy(
    particles: &[Particle],
    xrange: RangeInclusive<f64>,
    yrange: RangeInclusive<f64>,
) -> usize {
    let mut s = 0;
    for i in 0..particles.len() {
        let p = &particles[i];
        for q in particles.iter().skip(i+1) {
            if p.intersects_xy(q, &xrange, &yrange) {
                s += 1;
            }
        }
    }
    s
}

fn parse_particles(lines: &[&str]) -> Vec<Particle> {
    lines.iter().map(parse_particle).collect()
}

fn parse_particle(line: &&str) -> Particle {
    let sides: Vec<_> = line.split('@').collect();
    let p: Vec<_> = sides[0]
        .split(',')
        .map(|x| x.trim().parse::<f64>().unwrap())
        .collect();
    let v: Vec<_> = sides[1]
        .split(',')
        .map(|x| x.trim().parse::<f64>().unwrap())
        .collect();
    Particle {
        p: (p[0], p[1], p[2]),
        v: (v[0], v[1], v[2]),
    }
}

type Point = (f64, f64, f64);

#[derive(Debug)]
struct Particle {
    p: Point,
    v: Point,
}

impl Particle {
    fn intersects_xy(
        &self,
        other: &Particle,
        xrange: &RangeInclusive<f64>,
        yrange: &RangeInclusive<f64>,
    ) -> bool {
        let a = self.p.0;
        let b = self.p.1;
        let c = self.v.0;
        let d = self.v.1;

        let e = other.p.0;
        let f = other.p.1;
        let g = other.v.0;
        let h = other.v.1;

        let a1 = c;
        let b1 = -g;
        let c1 = e - a;
        let a2 = d;
        let b2 = -h;
        let c2 = f - b;
        let det = a1 * b2 - b1 * a2;
        let u = (c1 * b2 - b1 * c2) / det;
        let v = (a1 * c2 - c1 * a2) / det;
        if u < 0.0 || v < 0.0 {
            return false;
        }
        let x = a + c * u;
        let y = b + d * u;
        xrange.contains(&x) && yrange.contains(&y)
    }
}

pub fn day24_2() {
    let _inp = "19, 13, 30 @ -2,  1, -2
18, 19, 22 @ -1, -1, -2
20, 25, 34 @ -2, -2, -4
12, 31, 28 @ -1, -2, -1
20, 19, 15 @  1, -5, -3";
    let inp = std::fs::read_to_string("day24.input.txt").unwrap();
    let inp = inp.trim();
    let lines: Vec<_> = inp.lines().collect();

    let particles = parse_particles(&lines);

    let s = solve_throw(&particles);

    println!("score = {}", s);
}

fn solve_throw(particles: &[Particle]) -> isize {
    let mut x = vec![];
    let mut b0 = vec![];
    let mut b1 = vec![];
    let mut b2 = vec![];
    let mut a0b1 = vec![];
    let mut a0b2 = vec![];
    let mut a1b0 = vec![];
    let mut a2b0 = vec![];
    let mut a0 = vec![];
    let mut a1 = vec![];
    let mut a2 = vec![];

    for p in particles {
        let c0 = p.p.0 as i64;
        let d0 = p.v.0 as i64;
        let c1 = p.p.1 as i64;
        let d1 = p.v.1 as i64;
        let c2 = p.p.2 as i64;
        let d2 = p.v.2 as i64;
        // solve for a0, a1, a2, b0, b1, b2 for each particle, eliminating t
        b2.push(c0);
        a0b2.push(-1);
        a0.push(d2);
        b0.push(-c2);
        a2b0.push(1);
        a2.push(-d0);
        x.push(c0 * d2 - c2 * d0);
        a1.push(0);
        b1.push(0);
        a0b1.push(0);
        a1b0.push(0);

        b2.push(0);
        a2.push(0);
        a0b2.push(0);
        a2b0.push(0);
        b1.push(c0);
        a0b1.push(-1);
        a0.push(d1);
        b0.push(-c1);
        a1b0.push(1);
        a1.push(-d0);
        x.push(c0 * d1 - c1 * d0);
    }

    let mut v = vec![];
    for i in 0..x.len() {
        v.push(vec![
            a2b0[i], a1b0[i], a0b1[i], a0b2[i], a0[i], a1[i], a2[i], b0[i], b1[i], b2[i], x[i],
        ]);
    }

    // each pair of particles eliminates the non-linear parts if we subtract them
    let mut eqs = vec![];
    for i in (0..x.len()).step_by(4) {
        if i + 2 >= v.len() {
            break;
        }
        let x: Vec<_> = v[i]
            .iter()
            .zip(v[i + 2].iter())
            .map(|(x, y)| *x - *y)
            .skip(4)
            .collect();
        let y: Vec<_> = v[i + 1]
            .iter()
            .zip(v[i + 3].iter())
            .map(|(x, y)| *x - *y)
            .skip(4)
            .collect();
        eqs.push(x);
        eqs.push(y);
    }
    // we now have a system of linear equations in 6 variables.
    // we don't need all of the equations. Take the last 6.
    eqs.reverse();
    eqs = eqs.iter().take(6).cloned().collect();
    let b: Vec<_> = eqs.iter().map(|v| *v.last().unwrap()).collect();
    let m: Vec<_> = eqs
        .iter()
        .map(|v| v.iter().take(6).copied().collect::<Vec<_>>())
        .collect();
    // solve using Cramer's rule.
    let det = determinant(&m);
    // checking that we didn't pick rows with problems.
    assert!(!det.is_zero());
    let a0d = determinant(&swap(&m, 0, &b));
    let a1d = determinant(&swap(&m, 1, &b));
    let a2d = determinant(&swap(&m, 2, &b));
    assert!(&a0d.is_multiple_of(&det));
    assert!(&a1d.is_multiple_of(&det));
    assert!(&a2d.is_multiple_of(&det));
    // we need bigint for the intermediate computations, but this should all cancel out to be isize.
    let a0 = a0d / &det;
    let a1 = a1d / &det;
    let a2 = a2d / &det;
    (a0 + a1 + a2).to_isize().unwrap()
}

fn swap(m: &[Vec<i64>], c: usize, b: &[i64]) -> Vec<Vec<i64>> {
    let mut n = vec![];
    for i in 0..m.len() {
        let mut v = vec![];
        for j in 0..m[0].len() {
            if j == c {
                v.push(b[i]);
            } else {
                v.push(m[i][j]);
            }
        }
        n.push(v);
    }
    n
}

fn determinant(m: &[Vec<i64>]) -> BigInt {
    assert_eq!(m.len(), m[0].len());
    if m.len() == 2 {
        let m00 = BigInt::from(m[0][0]);
        let m11 = BigInt::from(m[1][1]);
        let m01 = BigInt::from(m[0][1]);
        let m10 = BigInt::from(m[1][0]);
        m00 * m11 - m01 * m10
    } else {
        let mut d = BigInt::zero();
        let mut mult = 1;
        for (i, x) in m[0].iter().enumerate() {
            d += mult * x * determinant(&remove_row_column(m, 0, i));
            mult *= -1;
        }
        d
    }
}

fn remove_row_column(m: &[Vec<i64>], r: usize, c: usize) -> Vec<Vec<i64>> {
    let mut n = vec![];
    for i in 0..m.len() {
        if i == r {
            continue;
        }
        let mut v = vec![];
        for j in 0..m[0].len() {
            if j == c {
                continue;
            }
            v.push(m[i][j]);
        }
        n.push(v);
    }
    n
}
