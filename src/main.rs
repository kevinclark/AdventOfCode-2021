use std::fs;
use std::path::PathBuf;

use aoc21::day_1;
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
}

fn main() {
    let opts = Options::parse();
    let input = &fs::read_to_string(opts.input).expect("Unable to read input file");

    match opts.subcmd {
        SubCommand::Day1(part) => match part {
            Part::One => println!("{}", day_1::part_1(input)),
            Part::Two => println!("{}", day_1::part_2(input)),
        },
    };
}
