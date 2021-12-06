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

pub fn part_2(fish: Vec<Fish>, days: u32) -> usize {
    let mut buckets = [0usize].repeat(9); // 9 Buckets of possible timers
    for f in fish {
        buckets[f.timer as usize] += 1;
    }

    for _ in 0..days {
        // Age by creating new fish from bucket 0 and shifting everything left
        let new_fish = buckets[0]; // Going to 8
        let old_fish = buckets[0]; // Going back to 6
        buckets.remove(0); // Less memory to shift the index around, but for now
        buckets[6] += old_fish;
        buckets.push(new_fish);
    }

    buckets.iter().sum()
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

    #[test]
    fn test_main_example_part_2() {
        let fish = parse("3,4,3,1,2");
        assert_eq!(26, part_2(fish.clone(), 18));
        assert_eq!(5934, part_2(fish, 80));
    }
}
