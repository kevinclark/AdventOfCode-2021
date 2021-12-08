use std::collections::{HashMap, HashSet};

use itertools::Itertools;

pub fn parse(input: &str) -> impl Iterator<Item = &str> {
    input
        .trim()
        .split(&[' ', '|', '\n'][..])
        .filter(|s| !s.is_empty())
}

pub fn part_1<'a>(tokens: impl Iterator<Item = &'a str>) -> usize {
    tokens
        .chunks(14)
        .into_iter()
        .map(|c| c.skip(10).filter(|s| signal_unambiguous(s)).count())
        .sum()
}

pub fn part_2<'a>(tokens: impl Iterator<Item = &'a str>) -> usize {
    tokens
        .chunks(14)
        .into_iter()
        .map(|chunk| {
            let ts: Vec<&str> = chunk.collect();
            let input_map = decode_input_signals(&ts[..10]);
            ts[10..].iter().fold(0, |acc, s| {
                let sorted: String = s.chars().sorted().collect();
                //println!("Mapping {:?}", sorted);
                let value = *input_map.get(&sorted).unwrap();
                acc * 10 + value
            })
        })
        .sum()
}

fn signal_unambiguous(s: &str) -> bool {
    [2, 4, 3, 7].contains(&s.len())
}

fn decode_input_signals(signals: &[&str]) -> HashMap<String, usize> {
    let (known, unknown): (Vec<&str>, Vec<&str>) =
        signals.iter().partition(|s| signal_unambiguous(s));

    let known: HashMap<usize, String> = known
        .iter()
        .map(|s| {
            let value = match s.len() {
                2 => 1,
                3 => 7,
                4 => 4,
                7 => 8,
                _ => panic!("Unknown length in unambiguous part: {}", s.len()),
            };
            let sorted = s.chars().sorted().collect();

            //println!("Decoded {:?} as {}", sorted, value);

            (value, sorted)
        })
        .collect();

    let four: HashSet<char> = known.get(&4).unwrap().chars().collect();
    let one: HashSet<char> = known.get(&1).unwrap().chars().collect();

    let derived = unknown.iter().map(|s| {
        let segs: HashSet<char> = s.chars().collect();

        let value: usize = match segs.len() {
            6 => {
                if segs.intersection(&four).count() == four.len() {
                    9
                } else if segs.intersection(&one).count() == one.len() {
                    0
                } else {
                    6
                }
            }
            5 => {
                if segs.intersection(&one).count() == one.len() {
                    3
                } else if segs.intersection(&four).count() == 3 {
                    5
                } else {
                    2
                }
            }
            _ => panic!("Unknown segment length: {}", s.len()),
        };

        let sorted = s.chars().sorted().collect();
        //println!("Decoded {:?} as {}", sorted, value);
        (sorted, value)
    });

    known
        .into_iter()
        .map(|(k, v)| (v, k))
        .chain(derived.into_iter())
        .collect()
}

#[cfg(test)]
mod tests {
    use super::{parse, part_2};

    #[test]
    fn examples() {
        let parsed = parse(
            "acedgfb cdfbe gcdfa fbcad dab cefabd cdfgeb eafb cagedb ab | \
                            cdfeb fcadb cdfeb cdbaf",
        );
        assert_eq!(5353, part_2(parsed));

        let parsed = parse(
            "be cfbegad cbdgef fgaecd cgeb fdcge agebfd fecdb fabcd edb | \
                            fdgacbe cefdb cefbgd gcbe",
        );
        assert_eq!(8394, part_2(parsed));
    }
}
