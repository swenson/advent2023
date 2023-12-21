use num_integer::Integer;
use std::collections::{HashMap, HashSet, VecDeque};

pub fn day20() {
    let _inp = "broadcaster -> a
%a -> inv, con
&inv -> b
%b -> con
&con -> output";
    let inp = std::fs::read_to_string("day20.input.txt").unwrap();
    let inp = inp.trim();

    let lines: Vec<_> = inp.lines().collect();

    let modules = parse_modules(&lines);

    let (lows, highs) = push_button_times_begin(&modules, 1000);
    let score = lows * highs;
    println!("score = {} * {} = {}", lows, highs, score);
}

struct Module {
    kind: char,
    outputs: Vec<String>,
}

#[derive(Clone, Debug)]
struct ModuleInt {
    kind: char,
    outputs: Vec<usize>,
}

fn push_button_times_begin(modules: &HashMap<String, Module>, times: usize) -> (usize, usize) {
    let mut init_ff_state = HashMap::new();
    let mut init_conj_state = HashMap::new();
    let conj_states: HashSet<_> = modules
        .iter()
        .filter(|(_, m)| m.kind == '&')
        .map(|(n, _)| n.clone())
        .collect();
    modules
        .iter()
        .filter(|(_, m)| m.kind == '%')
        .for_each(|(n, _)| {
            init_ff_state.insert(n.clone(), 0u8);
        });
    for x in &conj_states {
        init_conj_state.insert(x.clone(), HashMap::new());
    }
    for (name, module) in modules {
        for out in &module.outputs {
            if conj_states.contains(out) {
                init_conj_state
                    .get_mut(out)
                    .unwrap()
                    .insert(name.clone(), 0u8);
            }
        }
    }
    let (lows, highs, _, _) = push_button_times(modules, times, init_ff_state, init_conj_state);
    (lows, highs)
}

#[allow(clippy::type_complexity)]
fn push_button_times(
    modules: &HashMap<String, Module>,
    times: usize,
    ff_state: HashMap<String, u8>,
    conj_state: HashMap<String, HashMap<String, u8>>,
) -> (
    usize,
    usize,
    HashMap<String, u8>,
    HashMap<String, HashMap<String, u8>>,
) {
    let mut lows = 0;
    let mut highs = 0;
    let mut new_ff_state = ff_state;
    let mut new_conj_state = conj_state;
    for _ in 0..times {
        let (nl, nh, nffs, ncs) = push_button(modules, &new_ff_state, &new_conj_state);
        new_ff_state = nffs;
        new_conj_state = ncs;
        lows += nl;
        highs += nh;
    }

    (lows, highs, new_ff_state.clone(), new_conj_state.clone())
}

#[allow(clippy::type_complexity)]
fn push_button(
    modules: &HashMap<String, Module>,
    ff_state: &HashMap<String, u8>,
    conj_state: &HashMap<String, HashMap<String, u8>>,
) -> (
    usize,
    usize,
    HashMap<String, u8>,
    HashMap<String, HashMap<String, u8>>,
) {
    let mut lows = 1;
    let mut highs = 0;
    let mut new_ff_state = ff_state.clone();
    let mut new_conj_state: HashMap<String, HashMap<String, u8>> = conj_state.clone();
    let mut incoming: VecDeque<(String, String, u8)> = VecDeque::new();

    for output in &modules.get("broadcaster").unwrap().outputs {
        incoming.push_front((String::from("broadcaster"), output.clone(), 0));
        lows += 1;
    }

    while !incoming.is_empty() {
        let (src, name, s) = incoming.pop_front().unwrap();
        let module = modules.get(&name).unwrap();
        let send;
        if module.kind == ' ' {
            continue;
        } else if module.kind == '%' {
            // flip-flop
            // ignore high
            if s == 1 {
                continue;
            }
            let curr = *new_ff_state.get(&name).unwrap();
            new_ff_state.insert(name.clone(), 1 - curr);
            send = 1 - curr;
        } else {
            let curr = new_conj_state.get_mut(&name).unwrap();
            curr.insert(src, s);
            if curr.values().all(|&x| x == 1) {
                send = 0;
            } else {
                send = 1;
            }
        }
        for out in &modules.get(&name).unwrap().outputs {
            incoming.push_back((String::from(&name), out.clone(), send));
            if send == 0 {
                lows += 1;
            } else {
                highs += 1;
            }
        }
    }

    (lows, highs, new_ff_state, new_conj_state)
}

fn parse_modules(lines: &[&str]) -> HashMap<String, Module> {
    let mut modules = HashMap::new();
    let mut output = HashSet::new();
    for line in lines {
        let sides: Vec<_> = line.split("->").collect();
        let name = sides[0].trim();
        let outputs: Vec<String> = sides[1]
            .split(',')
            .map(|x| String::from(x.trim()))
            .collect();
        let (c, n) = if name.starts_with('&') || name.starts_with('%') {
            (name.as_bytes()[0] as char, String::from(&name[1..]))
        } else {
            assert_eq!(name, "broadcaster");
            ('B', String::from(name))
        };
        modules.insert(
            n,
            Module {
                kind: c,
                outputs: outputs.clone(),
            },
        );
        for out in &outputs {
            output.insert(out.clone());
        }
    }
    for o in &output {
        modules.entry(o.clone()).or_insert_with(|| Module {
            kind: ' ',
            outputs: vec![],
        });
    }
    modules
}

pub fn day20_2() {
    let _inp = "broadcaster -> a
%a -> inv, con
&inv -> b
%b -> con
&con -> output";
    let inp = std::fs::read_to_string("day20.input.txt").unwrap();
    let inp = inp.trim();

    let lines: Vec<_> = inp.lines().collect();

    let (modules, _) = parse_modules_int(&lines);
    let rx = solve(&modules);
    println!("rx = {}", rx);
}

fn solve(modules: &[ModuleInt]) -> usize {
    // we know something about the structure
    // the broadcast should operate on independent subgraphs, for the most part, so we should be able to solve them individually

    // get the node that points to rx
    let mut rxptr = 0;
    for (i, m) in modules.iter().enumerate() {
        if m.outputs.contains(&1) {
            rxptr = i;
            break;
        }
    }
    let mut prod = 1;
    for x in &modules[0].outputs {
        let cycle_len = solve_subgraph_output_len(modules, *x, rxptr);
        prod = prod.lcm(&cycle_len);
    }
    prod
}

// time for the given graph to output a 1 on the output
fn solve_subgraph_output_len(modules: &[ModuleInt], start: usize, target: usize) -> usize {
    let mut init_ff_state = vec![0u8; modules.len()];
    let mut init_conj_state: HashMap<usize, HashMap<usize, u8>> = HashMap::new();
    let conj_states: HashSet<_> = modules
        .iter()
        .enumerate()
        .filter(|(_, m)| m.kind == '&')
        .map(|(n, _)| n)
        .collect();
    modules
        .iter()
        .enumerate()
        .filter(|(_, m)| m.kind == '%')
        .for_each(|(n, _)| {
            init_ff_state.insert(n, 0u8);
        });
    for &x in &conj_states {
        init_conj_state.insert(x, HashMap::new());
    }
    for (name, module) in modules.iter().enumerate() {
        for out in &module.outputs {
            if conj_states.contains(out) {
                init_conj_state.get_mut(out).unwrap().insert(name, 0u8);
            }
        }
    }

    let ff_state = init_ff_state;
    let conj_state = init_conj_state;
    let mut new_ff_state = ff_state;
    let mut new_conj_state = conj_state;
    let mut cnt = 0;
    loop {
        let triggered = send0(
            modules,
            &mut new_ff_state,
            &mut new_conj_state,
            start,
            target,
        );
        cnt += 1;
        if triggered {
            return cnt;
        }
    }
}

fn send0(
    modules: &[ModuleInt],
    ff_state: &mut [u8],
    conj_state: &mut HashMap<usize, HashMap<usize, u8>>,
    start: usize,
    target: usize,
) -> bool {
    let mut incoming: VecDeque<(usize, usize, u8)> = VecDeque::new();
    incoming.push_front((0, start, 0));
    let mut triggered = false;

    while !incoming.is_empty() {
        let (src, name, s) = incoming.pop_front().unwrap();
        let module = &modules[name];
        let send;
        if module.kind == ' ' {
            continue;
        } else if module.kind == '%' {
            // flip-flop
            // ignore high
            if s == 1 {
                continue;
            }
            let curr = ff_state[name];
            ff_state[name] = 1 - curr;
            send = 1 - curr;
        } else {
            let curr = conj_state.get_mut(&name).unwrap();
            curr.insert(src, s);
            if curr.values().all(|&x| x == 1) {
                send = 0;
            } else {
                send = 1;
            }
        }
        for out in &modules[name].outputs {
            incoming.push_back((name, *out, send));
            if *out == target && send == 1 {
                triggered = true;
            }
        }
    }
    triggered
}

fn parse_modules_int(lines: &[&str]) -> (Vec<ModuleInt>, usize) {
    let mut names = HashMap::new();
    names.insert(String::from("broadcaster"), 0);
    names.insert(String::from("rx"), 1);
    // collect names
    for line in lines {
        let sides: Vec<_> = line.split("->").collect();
        let name = sides[0].trim();
        let name = if name.starts_with('%') || name.starts_with('&') {
            &name[1..]
        } else {
            name
        };
        let len = names.len();
        names.entry(String::from(name)).or_insert_with(|| len);
    }
    for line in lines {
        let sides: Vec<_> = line.split("->").collect();
        let outputs: Vec<String> = sides[1]
            .split(',')
            .map(|x| String::from(x.trim()))
            .collect();
        for out in &outputs {
            let len = names.len();
            names.entry(out.clone()).or_insert_with(|| len);
        }
    }

    let mut modules = vec![
        ModuleInt {
            kind: ' ',
            outputs: vec![]
        };
        names.len()
    ];
    for line in lines {
        let sides: Vec<_> = line.split("->").collect();
        let name = sides[0].trim();
        let outputs: Vec<String> = sides[1]
            .split(',')
            .map(|x| String::from(x.trim()))
            .collect();
        let outputs = outputs
            .iter()
            .map(|x| names.get(x).unwrap())
            .copied()
            .collect();
        let (c, n) = if name.starts_with('&') || name.starts_with('%') {
            (name.as_bytes()[0] as char, String::from(&name[1..]))
        } else {
            assert_eq!(name, "broadcaster");
            ('B', String::from(name))
        };
        let idx = *names.get(&n).unwrap();
        modules[idx].kind = c;
        modules[idx].outputs = outputs;
    }
    (modules, *names.get("rx").unwrap())
}

fn _to_c(lines: &[&str]) -> String {
    let mut out = String::new();
    let mut statics = String::new();
    let mut decls = String::new();
    let mut inputs = vec![];
    let mut process_src = String::new();
    let mut process_dst = String::new();
    let (modules, _) = parse_modules_int(lines);

    for _ in 0..modules.len() {
        inputs.push(vec![]);
    }
    for (i, module) in modules.iter().enumerate() {
        for o in &module.outputs {
            inputs[*o].push(i);
        }
    }
    println!("// inputs = {:?}", inputs);
    for (i, module) in modules.iter().enumerate() {
        if module.kind == '%' {
            decls += &format!("static inline void process_{}(int signal);\n", i);
            statics += &format!("static int s_{} = 0;\n", i);
            out += &format!(
                "static inline void process_{}(int signal) {{
  int curr = s_{};
  s_{} = 1 - curr;
",
                i, i, i
            );
            process_src += &format!("    case {}:\n", i);
            process_dst += &format!("    case {}:\n", i);
            for output in &module.outputs {
                if modules[*output].kind == '%' {
                    out += &format!("  if (curr==1) enqueue({}, {}, 1-curr);\n", i, output);
                } else {
                    out += &format!("  enqueue({}, {}, 1-curr);\n", i, output);
                }
                if modules[*output].kind == '&' {
                    process_src += &format!(
                        "      if (dst == {}) input_{}_{} = signal;\n",
                        output, output, i
                    );
                }
            }
            process_dst += &format!("      process_{}(signal);\n", i);
            process_src += "      break;\n";
            process_dst += "      break;\n";
            out += "}\n";
        } else if module.kind == '&' {
            decls += &format!("static inline void process_{}(int signal);\n", i);
            statics += &format!("static int s_{} = 0;\n", i);
            out += &format!("static inline void process_{}(int signal) {{\n", i);
            let inps = &inputs[i];
            for x in inps {
                statics += &format!("static int input_{}_{} = 0;\n", i, x);
            }
            out += "  int _s = 1;\n";
            out += "  if (";
            for (a, j) in inps.iter().enumerate() {
                if a != 0 {
                    out += " && ";
                }
                out += &format!("input_{}_{} == 1", i, j);
            }
            out += ") _s = 0;\n";

            process_src += &format!("    case {}:\n", i);
            process_dst += &format!("    case {}:\n", i);
            for output in &module.outputs {
                if modules[*output].kind == '%' {
                    out += &format!("  if (_s==0) enqueue({}, {}, 0);\n", i, output);
                } else {
                    out += &format!("  enqueue({}, {}, _s);\n", i, output);
                }
                if modules[*output].kind == '&' {
                    process_src += &format!(
                        "      if (dst == {}) input_{}_{} = signal;\n",
                        output, output, i
                    );
                }
            }
            process_dst += &format!("      process_{}(signal);\n", i);
            process_src += "      break;\n";
            process_dst += "      break;\n";
            out += "}\n";
        } else {
            process_src += &format!("    case {}:\n", i);
            process_dst += &format!("    case {}:\n", i);
            if i != 1 {
                decls += &format!("static inline void process_{}(int signal);\n", i);
                statics += &format!("static int s_{} = 0;\n", i);
                out += &format!("static inline void process_{}(int signal) {{\n", i);
                for output in &module.outputs {
                    if modules[*output].kind == '%' {
                        out += &format!("  if (signal==0) enqueue({}, {}, 0);\n", i, output);
                    } else {
                        out += &format!("  enqueue({}, {}, signal);\n", i, output);
                    }
                    if modules[*output].kind == '&' {
                        process_src += &format!(
                            "      if (dst == {}) input_{}_{} = signal;\n",
                            output, output, i
                        );
                    }
                }
                out += "}\n";
            }
            process_dst += &format!("      process_{}(signal);\n", i);
            process_src += "      break;\n";
            process_dst += "      break;\n";
        }
    }

    let header = format!(
        "
#include <stdio.h>
#include <stdlib.h>

static int queue[65536];
static int queue_l = 0;
static int queue_r = 0;

static inline void enqueue(int src, int dst, int signal) {{
  queue[queue_r++] = (src << 10) | (dst << 1) | signal;
}}

#define s_rx s_1
static int s_1 = 0;
{}
{}
static int step();
static void process(int src, int dst, int signal);
static inline void rx(int signal);
int main() {{
  for (int i = 0; i < 65536; i++) {{
    queue[i] = 0;
  }}

  long long i = 0;
  while (1) {{
    i++;
    if (step() == 1) {{
      printf(\"%lld\\n\", i);
      return 0;
    }}
    if ((i & 0xfffff) == 0) {{
      printf(\"progress %lld\\n\", i);
    }}
  }}
}}
static int step() {{
  queue_l = 0;
  queue_r = 0;
  enqueue(0, 0, 0);
  while (queue_l != queue_r) {{
    int x = queue[queue_l++];
    process(x >> 10, (x >> 1) & 0x1ff, x & 1);
  }}
  int r = s_rx;
  s_rx = 0;
  return r;
}}
static inline void process_1(int signal) {{
  if (signal == 0) {{
    s_rx++;
  }}
}}
static void process(int src, int dst, int signal) {{
  switch (src) {{
{}
    default:
      break;
  }}
  switch (dst) {{
{}
    default:
      break;
  }}
}}
",
        statics, decls, process_src, process_dst
    );
    header + &out
}
