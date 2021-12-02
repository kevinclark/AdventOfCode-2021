use itertools::Itertools;

#[derive(Default)]
struct Position {
    depth: u32,
    distance: u32,
    aim: u32,
}

pub fn part_1(input: &str) -> u32 {
    let instructions = input.split(&['\n', ' '][..]).tuples();
    let Position {
        depth, distance, ..
    } = instructions.fold(Position::default(), |current, (direction, distance)| {
        let distance = distance.parse::<u32>().unwrap();
        match direction {
            "forward" => Position {
                distance: current.distance + distance,
                ..current
            },
            "down" => Position {
                depth: current.depth + distance,
                ..current
            },
            "up" => Position {
                depth: current.depth - distance,
                ..current
            },
            _ => panic!("Unknown direction: {}", direction),
        }
    });

    depth * distance
}

pub fn part_2(input: &str) -> u32 {
    let instructions = input.split(&['\n', ' '][..]).tuples();
    let Position {
        depth, distance, ..
    } = instructions.fold(Position::default(), |current, (direction, distance)| {
        let distance = distance.parse::<u32>().unwrap();
        match direction {
            "forward" => Position {
                distance: current.distance + distance,
                depth: current.depth + (current.aim * distance),
                ..current
            },
            "down" => Position {
                aim: current.aim + distance,
                ..current
            },
            "up" => Position {
                aim: current.aim - distance,
                ..current
            },
            _ => panic!("Unknown direction: {}", direction),
        }
    });

    depth * distance
}
