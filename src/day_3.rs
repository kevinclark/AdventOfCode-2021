use arrayvec::ArrayVec;

pub fn parse<const N: usize>(input: &str) -> impl Iterator<Item = ArrayVec<bool, N>> + Clone + '_ {
    input
        .split('\n')
        .filter(|&x| !x.is_empty())
        .map(|line| line.bytes().map(|b| b == b'1').collect())
}

pub fn part_1<const N: usize>(measurements: impl Iterator<Item = ArrayVec<bool, N>>) -> u32 {
    let (most_common, least_common) = solve_1(measurements);

    most_common * least_common
}

fn solve_1<const N: usize>(measurements: impl Iterator<Item = ArrayVec<bool, N>>) -> (u32, u32) {
    let (counts, num_entries) = frequency(measurements);

    let most_common = counts
        .iter()
        .map(|c| *c > num_entries / 2)
        .fold(0, |acc, b| (acc << 1) | b as u32);

    let least_common = (2u32.pow(N as u32) - 1) - most_common;

    (most_common, least_common)
}

pub fn part_2<const N: usize>(
    measurements: impl Iterator<Item = ArrayVec<bool, N>> + Clone,
) -> u32 {
    let (most_common, least_common) = solve_2(measurements);

    most_common * least_common
}

fn solve_2<const N: usize>(
    measurements: impl Iterator<Item = ArrayVec<bool, N>> + Clone,
) -> (u32, u32) {
    let mut candidates: Vec<ArrayVec<bool, N>> = measurements.clone().collect();

    let mut o2_rating = None;

    let mut co2_rating = None;

    for i in 0..N {
        let ones_count = candidates.iter().filter(|c| c[i]).count();
        let most_common = ones_count as f32 >= candidates.len() as f32 / 2.0;

        let filtered: Vec<ArrayVec<bool, N>> = candidates
            .into_iter()
            .filter(|m| m[i] == most_common)
            .collect();

        if filtered.len() == 1 {
            o2_rating = Some(filtered[0].clone());
            break;
        }
        candidates = filtered;
    }

    let mut candidates: Vec<ArrayVec<bool, N>> = measurements.collect();

    for i in 0..N {
        let ones_count = candidates.iter().filter(|c| c[i]).count();
        let least_common = (ones_count as f32) < candidates.len() as f32 / 2.0;

        let filtered: Vec<ArrayVec<bool, N>> = candidates
            .into_iter()
            .filter(|m| m[i] == least_common)
            .collect();
        if filtered.len() == 1 {
            co2_rating = Some(filtered[0].clone());
            break;
        }
        candidates = filtered;
    }

    (
        o2_rating
            .unwrap()
            .iter()
            .fold(0, |acc, b| (acc << 1) | *b as u32),
        co2_rating
            .unwrap()
            .iter()
            .fold(0, |acc, b| (acc << 1) | *b as u32),
    )
}

fn frequency<const N: usize>(items: impl Iterator<Item = ArrayVec<bool, N>>) -> ([u32; N], u32) {
    let mut num_items: u32 = 0;
    let mut counts = [0; N];

    for item in items {
        num_items += 1;
        item.iter()
            .enumerate()
            .for_each(|(i, v)| counts[i] += if *v { 1 } else { 0 })
    }

    (counts, num_items)
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

        assert_eq!((22, 9), solve_1(parsed));
    }

    #[test]
    fn solve_part_2_example() {
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

        assert_eq!((23, 10), solve_2(parsed));
    }
}
