use std::collections::HashMap;
use std::ops::Range;

pub fn day19() {
    let _inp = "px{a<2006:qkq,m>2090:A,rfg}
pv{a>1716:R,A}
lnx{m>1548:A,A}
rfg{s<537:gd,x>2440:R,A}
qs{s>3448:A,lnx}
qkq{x<1416:A,crn}
crn{x>2662:A,R}
in{s<1351:px,qqz}
qqz{s>2770:qs,m<1801:hdj,R}
gd{a>3333:R,R}
hdj{m>838:A,pv}

{x=787,m=2655,a=1222,s=2876}
{x=1679,m=44,a=2067,s=496}
{x=2036,m=264,a=79,s=2244}
{x=2461,m=1339,a=466,s=291}
{x=2127,m=1623,a=2188,s=1013}";
    let inp = std::fs::read_to_string("day19.input.txt").unwrap();
    let inp = inp.trim();

    let parts: Vec<_> = inp.split("\n\n").collect();
    let prog = parse_program(parts[0]);
    let parts = parse_parts(parts[1]);

    let score = run(&prog, &parts);
    println!("score = {}", score);
}

type Workflow = Vec<Rule>;
type Rule = (char, char, usize, String);

type Part = (usize, usize, usize, usize);

fn run(prog: &HashMap<String, Workflow>, parts: &Vec<Part>) -> usize {
    let mut score = 0;
    for part in parts {
        score += run_part(prog, part);
    }
    score
}

fn run_part(prog: &HashMap<String, Workflow>, part: &Part) -> usize {
    let mut current = String::from("in");

    loop {
        if current == "A" {
            return part.0 + part.1 + part.2 + part.3;
        } else if current == "R" {
            return 0;
        }
        let workflow = prog.get(&current).unwrap();
        for rule in workflow {
            if rule_is_match(rule, part) {
                current = rule.3.clone();
                break;
            }
        }
    }
}

fn rule_is_match(rule: &Rule, part: &Part) -> bool {
    if rule.0 == 'Z' {
        return true;
    }
    let num = match rule.0 {
        'x' => part.0,
        'm' => part.1,
        'a' => part.2,
        's' => part.3,
        _ => panic!("invalid part reference {}", rule.1),
    };
    if rule.1 == '<' {
        num < rule.2
    } else {
        num > rule.2
    }
}

fn parse_parts(inp: &str) -> Vec<Part> {
    let mut parts = vec![];
    for line in inp.lines() {
        let mut x = 0;
        let mut m = 0;
        let mut a = 0;
        let mut s = 0;
        for piece in line[1..line.len() - 1].split(',') {
            let sides: Vec<_> = piece.split('=').collect();
            let num = sides[1].parse::<usize>().unwrap();
            match sides[0] {
                "x" => {
                    x = num;
                }
                "m" => {
                    m = num;
                }
                "a" => {
                    a = num;
                }
                "s" => {
                    s = num;
                }
                _ => panic!("unknown char {}", sides[0]),
            }
        }
        parts.push((x, m, a, s));
    }
    parts
}

fn parse_program(inp: &str) -> HashMap<String, Workflow> {
    let mut workflows = HashMap::new();
    for line in inp.lines() {
        let idx = line.find('{').unwrap();
        let name = String::from(&line[0..idx]);
        let rules: Vec<_> = line[(idx + 1)..(line.len() - 1)]
            .split(',')
            .map(parse_rule)
            .collect();
        workflows.insert(name, rules);
    }
    workflows
}

fn parse_rule(txt: &str) -> Rule {
    if !txt.contains(':') {
        return ('Z', 'Z', 0, String::from(txt));
    }
    let parts: Vec<_> = txt.split(':').collect();
    let label = String::from(parts[1]);
    let check = parts[0];
    if check.contains('<') {
        let check: Vec<_> = check.split('<').collect();
        let x = check[0].as_bytes()[0] as char;
        let num = check[1].parse::<usize>().unwrap();
        (x, '<', num, label)
    } else {
        let check: Vec<_> = check.split('>').collect();
        let x = check[0].as_bytes()[0] as char;
        let num = check[1].parse::<usize>().unwrap();
        (x, '>', num, label)
    }
}

pub fn day19_2() {
    let _inp = "px{a<2006:qkq,m>2090:A,rfg}
pv{a>1716:R,A}
lnx{m>1548:A,A}
rfg{s<537:gd,x>2440:R,A}
qs{s>3448:A,lnx}
qkq{x<1416:A,crn}
crn{x>2662:A,R}
in{s<1351:px,qqz}
qqz{s>2770:qs,m<1801:hdj,R}
gd{a>3333:R,R}
hdj{m>838:A,pv}

{x=787,m=2655,a=1222,s=2876}
{x=1679,m=44,a=2067,s=496}
{x=2036,m=264,a=79,s=2244}
{x=2461,m=1339,a=466,s=291}
{x=2127,m=1623,a=2188,s=1013}";
    let inp = std::fs::read_to_string("day19.input.txt").unwrap();
    let inp = inp.trim();

    let parts: Vec<_> = inp.split("\n\n").collect();
    let prog = parse_program(parts[0]);
    let score = combinations(&prog);
    println!("score = {}", score);
}

fn combinations(prog: &HashMap<String, Workflow>) -> usize {
    let combos = combinations_workflow(
        prog,
        String::from("in"),
        (1..4001, 1..4001, 1..4001, 1..4001),
    );
    size(&combos)
}

fn range_size(r: &Range<usize>) -> usize {
    r.end - r.start
}

fn range4_size(r: &Range4) -> usize {
    range_size(&r.0) * range_size(&r.1) * range_size(&r.2) * range_size(&r.3)
}

fn intersect_size(r: &Range<usize>, s: &Range<usize>) -> usize {
    if !r.contains(&s.start) && !r.contains(&s.end) {
        0
    } else {
        let start = r.start.max(s.start);
        let end = r.end.min(s.end);
        end - start
    }
}

fn intersect4_size(r: &Range4, s: &Range4) -> usize {
    intersect_size(&r.0, &s.0)
        * intersect_size(&r.1, &s.1)
        * intersect_size(&r.2, &s.2)
        * intersect_size(&r.3, &s.3)
}

fn size(ranges: &[Range4]) -> usize {
    if ranges.is_empty() {
        0
    } else if ranges.len() == 1 {
        range4_size(&ranges[0])
    } else {
        let r = &ranges[0];
        let mut s = range4_size(r);
        for t in ranges.iter().skip(1) {
            s -= intersect4_size(r, t);
        }
        s + size(&ranges[1..ranges.len()])
    }
}

type Range4 = (Range<usize>, Range<usize>, Range<usize>, Range<usize>);

fn combinations_workflow(
    prog: &HashMap<String, Workflow>,
    name: String,
    ranges: Range4,
) -> Vec<Range4> {
    let mut ranges = ranges;
    let mut results = vec![];
    for rule in prog.get(&name).unwrap() {
        let target = rule.3.clone();
        let (accepts, rejects) = accepts_rejects(rule, &ranges);
        if accepts.is_some() {
            let accepts = accepts.clone().unwrap();
            if target == "A" {
                results.push(accepts);
            } else if target != "R" {
                results.extend(combinations_workflow(prog, target, accepts).into_iter());
            }
        }
        if rejects.is_some() {
            ranges = rejects.clone().unwrap();
        }
        if accepts.is_none() && rejects.is_none() {
            break;
        }
    }
    results
}

fn accepts_rejects(rule: &Rule, ranges: &Range4) -> (Option<Range4>, Option<Range4>) {
    let ranges = ranges.clone();
    let ranges2 = ranges.clone();
    let bound = rule.2;
    if rule.0 == 'Z' {
        (Some(ranges), None)
    } else {
        match rule.0 {
            'x' => {
                let (r, s) = split_range(rule.1, bound, ranges.0);
                (
                    r.map(|r| (r, ranges.1, ranges.2, ranges.3)),
                    s.map(|s| (s, ranges2.1, ranges2.2, ranges2.3)),
                )
            }
            'm' => {
                let (r, s) = split_range(rule.1, bound, ranges.1);
                (
                    r.map(|r| (ranges.0, r, ranges.2, ranges.3)),
                    s.map(|s| (ranges2.0, s, ranges2.2, ranges2.3)),
                )
            }
            'a' => {
                let (r, s) = split_range(rule.1, bound, ranges.2);
                (
                    r.map(|r| (ranges.0, ranges.1, r, ranges.3)),
                    s.map(|s| (ranges2.0, ranges2.1, s, ranges2.3)),
                )
            }
            's' => {
                let (r, s) = split_range(rule.1, bound, ranges.3);
                (
                    r.map(|r| (ranges.0, ranges.1, ranges.2, r)),
                    s.map(|s| (ranges2.0, ranges2.1, ranges2.2, s)),
                )
            }
            _ => panic!("invalid part reference {}", rule.1),
        }
    }
}

fn split_range(
    cmp: char,
    bound: usize,
    range: Range<usize>,
) -> (Option<Range<usize>>, Option<Range<usize>>) {
    if cmp == '<' {
        if range.end <= bound {
            (Some(range), None)
        } else if bound < range.start {
            (None, Some(range))
        } else {
            // split
            (Some(range.start..bound), Some(bound..range.end))
        }
    } else {
        // >
        if range.start > bound {
            (Some(range), None)
        } else if bound > range.end {
            (None, Some(range))
        } else {
            // split
            (Some(bound + 1..range.end), Some(range.start..bound + 1))
        }
    }
}
