pub fn day24() {
    let inp = "";
    let _inp = std::fs::read_to_string("day24.input.txt").unwrap();
    let inp = inp.trim();
    let lines: Vec<_> = inp.lines().map(|x| x.chars().collect::<Vec<_>>()).collect();

    let s = score(&lines);
    println!("score = {}", s);
}

fn score(lines: &[Vec<char>]) -> usize {
    todo!()
}

pub fn day24_2() {
    let inp = "";
    let _inp = std::fs::read_to_string("day24.input.txt").unwrap();
    let inp = inp.trim();
    let lines: Vec<_> = inp.lines().map(|x| x.chars().collect::<Vec<_>>()).collect();

    let s = score(&lines);
    println!("score = {}", s);
}
