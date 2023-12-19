use std::collections::HashMap;

pub fn day15() {
    let _inp = "HASH";
    let _inp = "rn=1,cm-,qp=3,cm=2,qp-,pc=4,ot=9,ab=5,pc-,pc=6,ot=7";
    let inp = std::fs::read_to_string("day15.input.txt").unwrap();
    let inp = inp.trim();

    let seq = inp.split(',');
    let mut s = 0;
    for x in seq {
        s += hash(x.as_bytes());
    }

    println!("hash sum = {}", s);
}

fn hash(x: &[u8]) -> usize {
    let mut h = 0;
    for a in x {
        h += *a as usize;
        h *= 17;
        h &= 255;
    }
    h
}

pub fn day15_2() {
    let _inp = "rn=1,cm-,qp=3,cm=2,qp-,pc=4,ot=9,ab=5,pc-,pc=6,ot=7";
    let inp = std::fs::read_to_string("day15.input.txt").unwrap();
    let inp = inp.trim();

    let mut map: HashMap<usize, Vec<(&str, usize)>> = HashMap::new();
    let seq = inp.split(',');
    for x in seq {
        if x.contains('=') {
            let v: Vec<_> = x.split('=').collect();
            let a = v[0];
            let h = hash(a.as_bytes());
            let mm = map.entry(h).or_default();
            let b = v[1].parse::<usize>().unwrap();
            let mut found = false;
            for i in 0..mm.len() {
                if mm[i].0 == a {
                    found = true;
                    mm.push((a, b));
                    mm.swap_remove(i);
                    break;
                }
            }
            if !found {
                mm.push((a, b));
            }
        } else {
            let y = &x[..x.len() - 1];
            let h = hash(y.as_bytes());
            let mm = map.entry(h).or_default();
            for i in 0..mm.len() {
                if mm[i].0 == y {
                    mm.remove(i);
                    break;
                }
            }
        }
    }

    let s = map
        .iter()
        .map(|(&k, v)| {
            v.iter()
                .enumerate()
                .map(|(a, &(_, c))| (k + 1) * (a + 1) * c)
                .sum::<usize>()
        })
        .sum::<usize>();

    println!("hash sum = {}", s);
}
