use itertools::Itertools;

pub fn part_2(input: &str) -> u32 {
    let depths = parse(input).tuple_windows().map(|(a, b, c)| a + b + c);

    count_increases(depths)
}

pub fn part_1(input: &str) -> u32 {
    let depths = parse(input);

    count_increases(depths)
}

fn count_increases<T: Clone + Iterator<Item = u32>>(depths: T) -> u32 {
    let starting_at_0 = depths.clone();
    let starting_at_1 = depths.skip(1);

    starting_at_0
        .zip_longest(starting_at_1)
        .fold(0, |count, pair| {
            let value = match pair {
                itertools::EitherOrBoth::Both(l, r) => {
                    if r > l {
                        1
                    } else {
                        0
                    }
                }
                _ => 0,
            };

            count + value
        })
}

fn parse(input: &str) -> impl Iterator<Item = u32> + Clone + '_ {
    input.split('\n').map(|s| s.parse::<u32>().unwrap_or(0))
}
