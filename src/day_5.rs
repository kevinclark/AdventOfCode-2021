use itertools::Itertools;
use num_rational::Ratio;
use std::cmp::{max, min};

#[derive(PartialEq, Eq, Debug)]
pub struct Line {
    start: (i32, i32),
    end: (i32, i32),
}

impl Line {
    fn contains_point(&self, x: i32, y: i32) -> bool {
        let dx = self.end.0 - self.start.0;
        let dy = self.end.1 - self.start.1;

        let domain = min(self.start.0, self.end.0)..=max(self.start.0, self.end.0);
        let range = min(self.start.1, self.end.1)..=max(self.start.1, self.end.1);

        if !range.contains(&y) || !domain.contains(&x) {
            return false;
        }

        if dx == 0 {
            x == self.start.0
        } else {
            Ratio::new(y - self.start.1, 1) == Ratio::new(dy, dx) * (x - self.start.0)
        }
    }

    fn is_vertical(&self) -> bool {
        self.end.0 - self.start.0 == 0
    }

    fn is_horizontal(&self) -> bool {
        self.end.1 - self.start.1 == 0
    }
}

pub fn part_1(lines: Vec<Line>) -> usize {
    let lines = lines
        .iter()
        .filter(|l| l.is_horizontal() || l.is_vertical());

    let max_x = lines
        .clone()
        .flat_map(|l| [l.start.0, l.end.0])
        .max()
        .unwrap();
    let max_y = lines
        .clone()
        .flat_map(|l| [l.start.1, l.end.1])
        .max()
        .unwrap();

    let mut count = 0;

    for x in 0..=max_x {
        for y in 0..=max_y {
            let mut containing = lines.clone().filter(|l| l.contains_point(x, y));

            if let Some(_first) = containing.next() {
                if let Some(_second) = containing.next() {
                    count += 1;

                    //println!("({}, {}) matches: {:?} and {:?}", x, y, first, second);
                }
            }
        }
    }

    count
}

pub fn parse(input: &str) -> Vec<Line> {
    input
        .split(&['\n', ' ', '-', '>', ','][..])
        .filter(|s| !s.is_empty())
        .map(|n| n.parse::<i32>().unwrap())
        .tuples()
        .map(|(x1, y1, x2, y2)| Line {
            start: (x1, y1),
            end: (x2, y2),
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_input() {
        assert_eq!(
            vec![
                Line {
                    start: (0, 0),
                    end: (10, 10)
                },
                Line {
                    start: (5, 6),
                    end: (7, 8)
                }
            ],
            parse("0,0 -> 10,10\n5,6 -> 7,8")
        );
    }

    #[test]
    fn line_contains_with_vertical_line() {
        let line = Line {
            start: (1, 1),
            end: (1, 3),
        };

        assert!(line.contains_point(1, 1));
        assert!(line.contains_point(1, 2));
        assert!(line.contains_point(1, 3));
        assert!(!line.contains_point(1, 4));
    }

    #[test]
    fn line_contains_with_horizontal_line() {
        let line = Line {
            start: (3, 1),
            end: (5, 1),
        };

        assert!(line.contains_point(3, 1));
        assert!(line.contains_point(4, 1));
        assert!(line.contains_point(5, 1));
        assert!(!line.contains_point(6, 1));
    }

    #[test]
    fn example() {
        let lines = parse(
            "0,9 -> 5,9\n\
             8,0 -> 0,8\n\
             9,4 -> 3,4\n\
             2,2 -> 2,1\n\
             7,0 -> 7,4\n\
             6,4 -> 2,0\n\
             0,9 -> 2,9\n\
             3,4 -> 1,4\n\
             0,0 -> 8,8\n\
             5,5 -> 8,2",
        );

        assert_eq!(5, part_1(lines));
    }
}
