mod day1;
mod day2;
mod day3;
mod day4;
mod day5;
mod day6;
mod day7;

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
        };
        write!(f, "{}", x)
    }
}

fn main() {
    let args = Cli::parse();

    let cmd = args.command.unwrap_or(Command::D7);
    let main_fun = match cmd {
        Command::D1 => day1::day1,
        Command::D2 => day2::day2,
        Command::D3 => day3::day3,
        Command::D4 => day4::day4,
        Command::D5 => day5::day5,
        Command::D6 => day6::day6,
        Command::D7 => day7::day7,
    };
    main_fun();
}
