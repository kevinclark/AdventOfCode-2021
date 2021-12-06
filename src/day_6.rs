pub fn parse(input: &str) -> Vec<Fish> {
    input
        .trim()
        .split(',')
        .filter(|s| !s.is_empty())
        .map(|n| n.parse::<u8>().unwrap())
        .map(|timer| Fish { timer })
        .collect()
}

pub fn part_1(mut fish: Vec<Fish>, days: u8) -> usize {
    for _ in 0..days {
        fish = fish.into_iter().flat_map(|f| f.age()).collect();
    }

    fish.len()
}

#[derive(Copy, Clone)]
pub struct Fish {
    timer: u8,
}

impl Fish {
    fn age(mut self) -> Vec<Fish> {
        if self.timer == 0 {
            self.timer = 6;
            vec![self, Fish::default()]
        } else {
            self.timer -= 1;
            vec![self]
        }
    }
}

impl Default for Fish {
    fn default() -> Self {
        Fish { timer: 8 }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_main_example() {
        let fish = parse("3,4,3,1,2");
        assert_eq!(26, part_1(fish.clone(), 18));
        assert_eq!(5934, part_1(fish, 80));
    }
}
