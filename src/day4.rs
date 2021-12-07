use std::{num::ParseIntError, str::FromStr};

#[derive(Debug, Clone)]
pub struct Board {
    rows: Vec<Vec<Option<u32>>>,
    won: bool,
}

#[derive(Debug)]
pub struct BingoGame {
    numbers: Vec<u32>,
    boards: Vec<Board>,
}

impl Board {
    fn mark(&mut self, number: u32) -> bool {
        self.rows
            .iter_mut()
            .flatten()
            .filter(|x| *x == &Some(number))
            .for_each(|x| *x = None);

        self.won = self.has_won_by_row() || self.has_won_by_column();

        self.won
    }

    fn has_won_by_row(&self) -> bool {
        self.rows.iter().any(|row| row.iter().all(|x| x.is_none()))
    }

    fn has_won_by_column(&self) -> bool {
        let mut columns =
            (0..5).map(|x| -> Vec<Option<u32>> { (0..5).map(|y| self.rows[y][x]).collect() });

        columns.any(|y| y.iter().all(|x| x.is_none()))
    }

    fn compute_score(&self) -> u32 {
        self.rows
            .iter()
            .map(|row| -> u32 { row.iter().flatten().sum() })
            .sum()
    }
}

impl FromStr for Board {
    type Err = ParseIntError;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let rows = input
            .lines()
            .map(|x| {
                x.split_whitespace()
                    .map(|x| Some(x.parse().unwrap()))
                    .collect()
            })
            .collect();

        Ok(Board { rows, won: false })
    }
}

pub fn parse_input(input: &str) -> BingoGame {
    let lines: Vec<&str> = input.split("\n\n").collect();
    let numbers = lines[0]
        .split(',')
        .map(|s| s.trim().parse().unwrap())
        .collect();

    let boards = lines.iter().skip(1).map(|x| x.parse().unwrap()).collect();

    BingoGame { numbers, boards }
}

pub fn part1(input: &str) -> u32 {
    let bingo_game = parse_input(input);
    let mut boards = bingo_game.boards;

    for number in bingo_game.numbers {
        for board in &mut boards {
            if board.mark(number) {
                return board.compute_score() * number;
            }
        }
    }

    panic!("no winner or failed to compute score")
}

pub fn part2(input: &str) -> u32 {
    let bingo_game = parse_input(input);
    let mut boards = bingo_game.boards;
    let mut score = 0;

    for number in bingo_game.numbers {
        for board in &mut boards {
            if !board.won && board.mark(number) {
                score = board.compute_score() * number;
            }
        }
    }

    score
}

#[cfg(test)]
pub mod tests {
    const INPUT: &str = "7,4,9,5,11,17,23,2,0,14,21,24,10,16,13,6,15,25,12,22,18,20,8,19,3,26,1

22 13 17 11  0
 8  2 23  4 24
21  9 14 16  7
 6 10  3 18  5
 1 12 20 15 19

 3 15  0  2 22
 9 18 13 17  5
19  8  7 25 23
20 11 10 24  4
14 21 16 12  6

14 21 17 24  4
10 16 15  9 19
18  8 23 26 20
22 11 13  6  5
 2  0 12  3  7";

    use super::*;

    #[test]
    fn solve_day_4() {
        assert_eq!(part1(INPUT), 4512);
        assert_eq!(part2(INPUT), 1924);
    }
}
