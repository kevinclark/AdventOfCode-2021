use itertools::Itertools;

pub fn solve(input: &str) -> u32 {
    let depths = parse(input);

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
                itertools::EitherOrBoth::Left(_) => 0,
                itertools::EitherOrBoth::Right(_) => 0,
            };

            count + value
        })
}

fn parse<'a>(input: &'a str) -> impl Iterator<Item = u32> + Clone + 'a {
    input.split("\n").map(|s| s.parse::<u32>().unwrap_or(0))
}
