extern crate core;

mod day1;
mod day10;
mod day11;
mod day12;
mod day13;
mod day14;
mod day15;
mod day16;
mod day17;
mod day18;
mod day19;
mod day2;
mod day3;
mod day4;
mod day5;
mod day6;
mod day7;
mod day8;
mod day9;

use clap::{Parser, Subcommand};
use std::fmt;
use std::time::Instant;

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
    D3,
    D4,
    D5,
    D6,
    D7,
    D8,
    D9,
    D10,
    D11,
    D12,
    D13,
    D14,
    D15,
    D16,
    D17,
    D18,
    D19,
}

impl fmt::Display for Command {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let x = match self {
            Command::D1 => "d1",
            Command::D2 => "d2",
            Command::D3 => "d3",
            Command::D4 => "d4",
            Command::D5 => "d5",
            Command::D6 => "d6",
            Command::D7 => "d7",
            Command::D8 => "d8",
            Command::D9 => "d9",
            Command::D10 => "d10",
            Command::D11 => "d11",
            Command::D12 => "d12",
            Command::D13 => "d13",
            Command::D14 => "d14",
            Command::D15 => "d15",
            Command::D16 => "d16",
            Command::D17 => "d17",
            Command::D18 => "d18",
            Command::D19 => "d19",
        };
        write!(f, "{}", x)
    }
}

fn main() {
    let args = Cli::parse();

    let cmd = args.command.unwrap_or(Command::D18);
    let (pt1_fun, pt2_fun): (fn(), fn()) = match cmd {
        Command::D1 => (day1::day1, day1::day1_2),
        Command::D2 => (day2::day2, day2::day2_2),
        Command::D3 => (day3::day3, day3::day3_2),
        Command::D4 => (day4::day4, day4::day4_2),
        Command::D5 => (day5::day5, day5::day5_2),
        Command::D6 => (day6::day6, day6::day6_2),
        Command::D7 => (day7::day7, day7::day7_2),
        Command::D8 => (day8::day8, day8::day8_2),
        Command::D9 => (day9::day9, day9::day9_2),
        Command::D10 => (day10::day10, day10::day10_2),
        Command::D11 => (day11::day11, day11::day11_2),
        Command::D12 => (day12::day12, day12::day12_2),
        Command::D13 => (day13::day13, day13::day13_2),
        Command::D14 => (day14::day14, day14::day14_2),
        Command::D15 => (day15::day15, day15::day15_2),
        Command::D16 => (day16::day16, day16::day16_2),
        Command::D17 => (day17::day17, day17::day17_2),
        Command::D18 => (day18::day18, day18::day18_2),
        Command::D19 => (day19::day19, day19::day19_2),
    };
    let p1_start = Instant::now();
    pt1_fun();
    let p1 = Instant::now() - p1_start;
    println!("Part 1 took: {:.6} seconds\n", p1.as_secs_f64());
    let p2_start = Instant::now();
    pt2_fun();
    let p2 = Instant::now() - p2_start;
    println!("Part 2 took: {:.6} seconds", p2.as_secs_f64());
}
