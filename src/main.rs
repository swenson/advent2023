extern crate core;

mod day1;
mod day10;
mod day11;
mod day12;
mod day13;
mod day14;
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
        };
        write!(f, "{}", x)
    }
}

fn main() {
    let args = Cli::parse();

    let cmd = args.command.unwrap_or(Command::D14);
    let main_fun = match cmd {
        Command::D1 => day1::day1,
        Command::D2 => day2::day2,
        Command::D3 => day3::day3,
        Command::D4 => day4::day4,
        Command::D5 => day5::day5,
        Command::D6 => day6::day6,
        Command::D7 => day7::day7,
        Command::D8 => day8::day8,
        Command::D9 => day9::day9,
        Command::D10 => day10::day10,
        Command::D11 => day11::day11,
        Command::D12 => day12::day12,
        Command::D13 => day13::day13,
        Command::D14 => day14::day14,
    };
    main_fun();
}
