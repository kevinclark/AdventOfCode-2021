use std::fs;
use std::path::PathBuf;

use aoc21::{day_1, day_2, day_3, day_4, day_5, day_6, day_7};
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
    #[clap(subcommand)]
    Day3(Part),
    #[clap(subcommand)]
    Day4(Part),
    #[clap(subcommand)]
    Day5(Part),
    #[clap(subcommand)]
    Day6(Part),
    #[clap(subcommand)]
    Day7(Part),
}

fn main() {
    let opts = Options::parse();
    let input = &fs::read_to_string(opts.input).expect("Unable to read input file");

    let result = match opts.subcmd {
        SubCommand::Day1(part) => match part {
            Part::One => day_1::part_1(input) as usize,
            Part::Two => day_1::part_2(input) as usize,
        },

        SubCommand::Day2(part) => {
            let parsed = day_2::parse(input);
            match part {
                Part::One => day_2::part_1(parsed) as usize,
                Part::Two => day_2::part_2(parsed) as usize,
            }
        }

        SubCommand::Day3(part) => {
            let parsed = day_3::parse::<12>(input);

            match part {
                Part::One => day_3::part_1(parsed) as usize,
                Part::Two => day_3::part_2(parsed) as usize,
            }
        }

        SubCommand::Day4(part) => {
            let parsed = day_4::parse(input);

            match part {
                Part::One => day_4::part_1(parsed) as usize,
                Part::Two => day_4::part_2(parsed) as usize,
            }
        }

        SubCommand::Day5(part) => {
            let parsed = day_5::parse(input);

            match part {
                Part::One => day_5::part_1(parsed),
                Part::Two => day_5::part_2(parsed),
            }
        }

        SubCommand::Day6(part) => {
            let parsed = day_6::parse(input);

            match part {
                Part::One => day_6::part_1(parsed, 80),
                Part::Two => day_6::part_2(parsed, 256),
            }
        }

        SubCommand::Day7(part) => {
            let parsed = day_7::parse(input);

            match part {
                Part::One => day_7::part_1(parsed),
                Part::Two => day_7::part_2(parsed),
            }
        }
    };

    println!("{}", result);
}
