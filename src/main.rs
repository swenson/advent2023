use std::{fmt, fs};
use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Option<Command>,
}

#[derive(Subcommand, Debug)]
enum Command {
    D1,
}

impl fmt::Display for Command {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let x = match self {
            Command::D1 => "d1",
        };
        write!(f, "{}", x)
    }
}


fn main() {
    let args = Cli::parse();

    let cmd = args.command.unwrap_or(Command::D1);
    let main_fun = match cmd {
        Command::D1 => day1,
    };
    main_fun();
}

fn day1() {
    let inp = fs::read_to_string("day1.input.txt").unwrap();
    let mut s: u64 = 0;
    for line in inp.split_whitespace() {
        let digits: Vec<u8> = line.as_bytes().iter().filter(|y| y.is_ascii_digit()).map(|y| *y).collect();
        let num = (digits.first().unwrap() - 48) * 10 + digits.last().unwrap() - 48;
        s += num as u64;
    }
    println!("sum = {}", s);

    day1_2();
}

fn day1_2() {
    let inp = fs::read_to_string("day1.input.txt").unwrap();
    let mut s: u64 = 0;
    for line in inp.split_whitespace() {
        let line = line.replace("one", "one1one");
        let line = line.replace("two", "two2two");
        let line = line.replace("three", "three3three");
        let line = line.replace("four", "four4four");
        let line = line.replace("five", "five5five");
        let line = line.replace("six", "six6six");
        let line = line.replace("seven", "seven7seven");
        let line = line.replace("eight", "eight8eight");
        let line = line.replace("nine", "nine9nine");
        let digits: Vec<u8> = line.as_bytes().iter().filter(|y| y.is_ascii_digit()).map(|y| *y).collect();
        let num = (digits.first().unwrap() - 48) * 10 + digits.last().unwrap() - 48;
        s += num as u64;
    }
    println!("part 2 sum = {}", s);
}