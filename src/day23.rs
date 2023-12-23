pub fn day23() {
    let inp = "";
    let _inp = std::fs::read_to_string("day23.input.txt").unwrap();
    let inp = inp.trim();
    let lines: Vec<_> = inp.lines().collect();

    let s = score(&lines);
    println!("score = {}", s);
}

fn score(lines: &[&str]) -> usize {
    0
}

pub fn day23_2() {
    let inp = "";
    let _inp = std::fs::read_to_string("day23.input.txt").unwrap();
    let inp = inp.trim();
    let lines: Vec<_> = inp.lines().collect();

    let s = score(&lines);
    println!("score = {}", s);
}
