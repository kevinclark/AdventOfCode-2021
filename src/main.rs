use std::fs;
use std::path::PathBuf;

use aoc21::{day_1, day_2};
use clap::Parser;

#[derive(Parser, Debug)]
struct Options {
    input: PathBuf,
    #[clap(subcommand)]
    subcmd: SubCommand,
}

#[derive(Parser, Debug)]
enum Part {
    One,
    Two,
}

#[derive(Parser, Debug)]
enum SubCommand {
    #[clap(subcommand)]
    Day1(Part),
    #[clap(subcommand)]
    Day2(Part),
}

fn main() {
    let opts = Options::parse();
    let input = &fs::read_to_string(opts.input).expect("Unable to read input file");

    let result = match opts.subcmd {
        SubCommand::Day1(part) => match part {
            Part::One => day_1::part_1(input),
            Part::Two => day_1::part_2(input),
        },

        SubCommand::Day2(part) => {
            let parsed = day_2::parse(input);
            match part {
                Part::One => day_2::part_1(parsed),
                Part::Two => day_2::part_2(parsed),
            }
        }
    };

    println!("{}", result);
}
