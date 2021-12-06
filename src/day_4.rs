use ndarray::Array2;

#[derive(Debug, PartialEq, Eq, Default)]
pub struct BingoBoard {
    spaces: Array2<u32>,
    matches: Vec<u32>,
}

impl BingoBoard {
    fn new(spaces: Array2<u32>) -> BingoBoard {
        BingoBoard {
            spaces,
            matches: vec![],
        }
    }

    fn score_after(&mut self, num: u32) -> Option<u32> {
        if self.spaces.iter().any(|s| *s == num) {
            self.matches.push(num);
        }

        if self.matches.len() >= 5
            && (self
                .spaces
                .rows()
                .into_iter()
                .any(|row| row.iter().all(|space| self.matches.contains(space)))
                || self
                    .spaces
                    .columns()
                    .into_iter()
                    .any(|col| col.iter().all(|space| self.matches.contains(space))))
        {
            let sum_of_non_matches: u32 = self
                .spaces
                .iter()
                .filter(|s| !self.matches.contains(s))
                .sum();

            return Some(sum_of_non_matches * num);
        }

        None
    }
}

#[derive(Debug, PartialEq, Eq, Default)]
pub struct BingoGame {
    boards: Vec<BingoBoard>,
    numbers: Vec<u32>,
}

pub fn parse(input: &str) -> BingoGame {
    let mut parts = input.split("\n\n");

    let numbers = parts
        .next()
        .unwrap()
        .split(',')
        .map(|n| n.parse::<u32>().unwrap())
        .collect();

    let boards = parts
        .map(|b| {
            BingoBoard::new(
                Array2::from_shape_vec(
                    (5, 5),
                    b.split(&[',', '\n', ' '][..])
                        .filter(|s| !s.is_empty())
                        .map(|n| n.parse::<u32>().unwrap())
                        .collect(),
                )
                .unwrap(),
            )
        })
        .collect();

    BingoGame { numbers, boards }
}
pub fn part_1(game: BingoGame) -> u32 {
    let mut boards = game.boards;
    for number in game.numbers {
        for board in boards.iter_mut() {
            if let Some(score) = board.score_after(number) {
                return score;
            }
        }
    }

    0
}

#[cfg(test)]
mod tests {
    use ndarray::arr2;

    use super::*;

    #[test]
    fn parse_single_board() {
        let input = "1,2,3,4,5\n\n\
                     36 11 70 77 80\n\
                     63  3 56 75 28\n\
                     89 91 27 33 82\n\
                     53 79 52 96 32\n\
                     58 14 78 65 38";

        let game = parse(input);
        assert_eq!(vec![1, 2, 3, 4, 5], game.numbers);
        assert_eq!(
            vec![BingoBoard::new(arr2(&[
                [36, 11, 70, 77, 80],
                [63, 3, 56, 75, 28],
                [89, 91, 27, 33, 82],
                [53, 79, 52, 96, 32],
                [58, 14, 78, 65, 38]
            ]))],
            game.boards
        );
    }

    #[test]
    fn score_after_no_win() {
        let mut board = BingoBoard::new(Array2::from_shape_vec((5, 5), (0..25).collect()).unwrap());

        assert_eq!(None, board.score_after(25));
        assert_eq!(None, board.score_after(26));
        assert_eq!(None, board.score_after(27));
        assert_eq!(None, board.score_after(28));
        assert_eq!(None, board.score_after(29));
    }

    #[test]
    fn score_after_row_win() {
        let mut board = BingoBoard::new(Array2::from_shape_vec((5, 5), (0..25).collect()).unwrap());

        assert_eq!(None, board.score_after(24));
        assert_eq!(None, board.score_after(23));
        assert_eq!(None, board.score_after(22));
        assert_eq!(None, board.score_after(21));
        assert_eq!(Some(3800), board.score_after(20));
    }

    #[test]
    fn score_after_column_win() {
        let mut board = BingoBoard::new(Array2::from_shape_vec((5, 5), (0..25).collect()).unwrap());

        assert_eq!(None, board.score_after(0));
        assert_eq!(None, board.score_after(5));
        assert_eq!(None, board.score_after(10));
        assert_eq!(None, board.score_after(15));
        assert_eq!(Some(5000), board.score_after(20));
    }
}
