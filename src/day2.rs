use std::fs;

pub fn day2() {
    let inp = fs::read_to_string("day2.input.txt").unwrap();
    let red = 12;
    let green = 13;
    let blue = 14;

    let mut game = 0u64;
    let mut sum = 0u64;
    for line in inp.lines() {
        game += 1;
        let parts: Vec<_> = line.split(':').collect();
        //println!("{} {}", parts[0], parts[1]);
        let plays: Vec<_> = parts[1].split(';').collect();
        let mut impossible = false;
        for play in plays {
            for x in play.split(',') {
                let y: Vec<_> = x.trim().split(' ').collect();
                let num: u64 = y[0].parse().unwrap();
                if x.contains("red") && num > red {
                    impossible = true;
                }
                if x.contains("green") && num > green {
                    impossible = true;
                }
                if x.contains("blue") && num > blue {
                    impossible = true;
                }
            }
        }
        if !impossible {
            sum += game;
        }
    }
    println!("sum = {}", sum);
    day2_2();
}

fn day2_2() {
    let inp = fs::read_to_string("day2.input.txt").unwrap();
    let mut power = 0u64;
    for line in inp.lines() {
        let parts: Vec<_> = line.split(':').collect();
        let plays: Vec<_> = parts[1].split(';').collect();
        let mut red = 0;
        let mut green = 0;
        let mut blue = 0;
        for play in plays {
            for x in play.split(',') {
                let y: Vec<_> = x.trim().split(' ').collect();
                let num: u64 = y[0].parse().unwrap();
                if x.contains("red") && num > red {
                    red = num;
                }
                if x.contains("green") && num > green {
                    green = num;
                }
                if x.contains("blue") && num > blue {
                    blue = num;
                }
            }
        }
        power += red * green * blue;
    }
    println!("power = {}", power);
}
