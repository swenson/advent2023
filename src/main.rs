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
    D2,
}

impl fmt::Display for Command {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let x = match self {
            Command::D1 => "d1",
            Command::D2 => "d2",
        };
        write!(f, "{}", x)
    }
}


fn main() {
    let args = Cli::parse();

    let cmd = args.command.unwrap_or(Command::D2);
    let main_fun = match cmd {
        Command::D1 => day1,
        Command::D2 => day2,
    };
    main_fun();
}

fn day2() {
    let inp = fs::read_to_string("day2.input.txt").unwrap();
    let red = 12;
    let green = 13;
    let blue = 14;

    let mut game = 0u64;
    let mut sum = 0u64;
    for line in inp.lines() {
        game += 1;
        let parts: Vec<_> = line.split(":").collect();
        //println!("{} {}", parts[0], parts[1]);
        let plays: Vec<_> = parts[1].split(";").collect();
        let mut impossible = false;
        for play in plays {
            for x in play.split(",") {
                let y: Vec<_> = x.trim().split(" ").collect();
                let num: u64 = y[0].parse().unwrap();
                if x.contains("red") {
                    if num > red {
                        impossible = true;
                    }
                }
                if x.contains("green") {
                    if num > green {
                        impossible = true;
                    }
                }
                if x.contains("blue") {
                    if num > blue {
                        impossible = true;
                    }
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
        let parts: Vec<_> = line.split(":").collect();
        let plays: Vec<_> = parts[1].split(";").collect();
        let mut red = 0;
        let mut green = 0;
        let mut blue = 0;
        for play in plays {
            for x in play.split(",") {
                let y: Vec<_> = x.trim().split(" ").collect();
                let num: u64 = y[0].parse().unwrap();
                if x.contains("red") {
                    if num > red {
                        red = num;
                    }
                }
                if x.contains("green") {
                    if num > green {
                        green = num;
                    }
                }
                if x.contains("blue") {
                    if num > blue {
                        blue = num;
                    }
                }
            }
        }
        power += red * green * blue;
    }
    println!("power = {}", power);
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