use itertools::Itertools;

struct Position {
    depth: u32,
    distance: u32,
}

pub fn part_1(input: &str) -> u32 {
    let instructions = input.split(&['\n', ' '][..]).tuples();
    let Position { depth, distance } = instructions.fold(
        Position {
            depth: 0,
            distance: 0,
        },
        |current, (direction, distance)| {
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
        },
    );

    depth * distance
}
