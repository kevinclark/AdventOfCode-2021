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
        .map(|c| {
            c.skip(10)
                .filter(|s| [2, 4, 3, 7].contains(&s.len()))
                .count()
        })
        .sum()
}
