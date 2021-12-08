pub fn parse(input: &str) -> Vec<i32> {
    input
        .trim()
        .split(',')
        .map(|n| n.parse::<i32>().unwrap())
        .collect()
}

pub fn part_1(mut nums: Vec<i32>) -> usize {
    nums.sort_unstable();

    let smallest = nums.first().unwrap();
    let largest = nums.last().unwrap();

    (*smallest..=*largest)
        .map(|s| nums.iter().map(|n| (n - s).abs() as usize).sum())
        .min()
        .unwrap()
}

pub fn part_2(mut nums: Vec<i32>) -> usize {
    nums.sort_unstable();

    let smallest = nums.first().unwrap();
    let largest = nums.last().unwrap();

    (*smallest..=*largest)
        .map(|s| nums.iter().map(|n| triangle((n - s).abs() as usize)).sum())
        .min()
        .unwrap()
}

fn triangle(n: usize) -> usize {
    (n * (n + 1)) / 2
}
