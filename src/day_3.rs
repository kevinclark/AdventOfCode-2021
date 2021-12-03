use arrayvec::ArrayVec;

pub fn parse<const T: usize>(input: &str) -> impl Iterator<Item = ArrayVec<bool, T>> + '_ {
    input
        .split('\n')
        .map(|line| line.bytes().map(|b| b == b'1').collect())
}

#[derive(Debug, PartialEq, Eq)]
struct Diagnostic {
    gamma: u32,
    epsilon: u32,
}

pub fn part_1<const T: usize>(measurements: impl Iterator<Item = ArrayVec<bool, T>>) -> u32 {
    let Diagnostic {
        gamma: most_common,
        epsilon: least_common,
    } = solve_1(measurements);

    most_common * least_common
}

fn solve_1<const T: usize>(measurements: impl Iterator<Item = ArrayVec<bool, T>>) -> Diagnostic {
    let mut num_entries = 0;
    let mut counts = [0; T];

    for measurement in measurements {
        num_entries += 1;
        measurement
            .iter()
            .enumerate()
            .for_each(|(i, v)| counts[i] += if *v { 1 } else { 0 })
    }

    let most_common = counts
        .iter()
        .map(|c| *c > num_entries / 2)
        .fold(0, |acc, b| (acc << 1) | b as u32);

    let least_common = (2u32.pow(T as u32) - 1) - most_common;

    Diagnostic {
        gamma: most_common,
        epsilon: least_common,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_one_line() {
        assert_eq!(
            [true, false].repeat(3),
            &parse::<6>("101010").next().unwrap()[..]
        );
    }

    #[test]
    fn parse_two_linel() {
        let mut parsed = parse::<6>("101010\n010101");
        assert_eq!([true, false].repeat(3), &parsed.next().unwrap()[..]);

        assert_eq!([false, true].repeat(3), &parsed.next().unwrap()[..]);
    }

    #[test]
    fn solve_part_1_example() {
        let parsed = parse::<5>(
            "00100\n\
             11110\n\
             10110\n\
             10111\n\
             10101\n\
             01111\n\
             00111\n\
             11100\n\
             10000\n\
             11001\n\
             00010\n\
             01010",
        );

        assert_eq!(
            Diagnostic {
                gamma: 22,
                epsilon: 9
            },
            solve_1(parsed)
        );
    }
}
