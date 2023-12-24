pub fn day25() {
    let inp = std::fs::read_to_string("day25.input.txt").unwrap();
    let score = part1(&inp);
    println!("part 1 score = {}", score);
}

pub fn part1(inp: &str) -> usize {
    let _inp = inp.trim();
    let _lines: Vec<_> = inp.lines().collect();
    0
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

    #[test]
    fn test_25_1() {
        let inp = "";
        assert_eq!(0, part1(inp));
    }
    #[test]
    fn test_25_2() {
        let inp = "";
        assert_eq!(0, part2(inp));
    }
}