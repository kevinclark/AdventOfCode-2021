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
enum SubCommand {
    Day1,
}

fn main() {
    let opts = Options::parse();
    let input = &fs::read_to_string(opts.input).expect("Unable to read input file");

    match opts.subcmd {
        SubCommand::Day1 => println!("{}", day_1::solve(input)),
    };
}
