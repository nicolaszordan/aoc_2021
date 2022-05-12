use crate::aoc::AOCPart;

#[derive(Debug, PartialEq)]
struct BingoGame {
    numbers: Vec<u32>,
    boards: Vec<BingoBoard>,
}

#[derive(Debug, PartialEq, PartialOrd)]
enum BingoNumber {
    Marked(u32),
    Unmarked(u32),
}

#[derive(Debug, PartialEq, PartialOrd)]
struct BingoBoard {
    bingo_status: BingoStatus,
    bingo_lines: Vec<Vec<BingoNumber>>,
}

#[derive(Debug, PartialEq, PartialOrd, Clone, Copy)]
enum BingoStatus {
    Bingo(u32), // Bingo! with all Nonmarked values added to one another
    Bango,      // not Bingo
}

fn retrieve_bingo_game(input: &str) -> BingoGame {
    let numbers: Vec<u32> = input
        .lines()
        .next()
        .unwrap()
        .split(',')
        .map(|number| number.parse().unwrap())
        .collect();

    let boards: Vec<BingoBoard> = input
        .split("\n\n")
        .skip(1)
        .map(|board| BingoBoard {
            bingo_status: BingoStatus::Bango,
            bingo_lines: board
                .lines()
                .map(|board_line| {
                    board_line
                        .split_whitespace()
                        .map(|board_cell| BingoNumber::Unmarked(board_cell.parse().unwrap()))
                        .collect()
                })
                .collect(),
        })
        .collect();

    BingoGame { numbers, boards }
}

pub struct Part1 {}

impl AOCPart for Part1 {
    fn new() -> Self {
        Self {}
    }

    fn solve(&mut self, input: &str) -> String {
        let bingo_game = retrieve_bingo_game(input);
        Part1::do_bingo_game(bingo_game).to_string()
    }
}

impl Part1 {
    fn do_bingo_game(mut bingo_game: BingoGame) -> u32 {
        bingo_game
            .numbers
            .iter()
            .find_map(|bingo_number| {
                bingo_game
                    .boards
                    .iter_mut()
                    .find_map(
                        |bingo_board| match bingo_board.incoming_number(*bingo_number) {
                            BingoStatus::Bingo(bingo_result) => Some(bingo_result),
                            _ => None,
                        },
                    )
                    .map(|bingo_result| bingo_number * bingo_result)
            })
            .unwrap()
    }
}
pub struct Part2 {}

impl AOCPart for Part2 {
    fn new() -> Self {
        Self {}
    }

    fn solve(&mut self, input: &str) -> String {
        let bingo_game = retrieve_bingo_game(input);
        Part2::do_bingo_game(bingo_game).to_string()
    }
}

impl Part2 {
    // beurk
    fn do_bingo_game(mut bingo_game: BingoGame) -> u32 {
        bingo_game
            .numbers
            .iter()
            .find_map(|bingo_number| {
                bingo_game.boards.iter_mut().for_each(|game_board| {
                    game_board.incoming_number(*bingo_number);
                });

                match bingo_game.boards.len() {
                    0 => panic!("missed bingo"),
                    1 => {
                        if let BingoStatus::Bingo(bingo_result) = bingo_game.boards[0].bingo_status
                        {
                            return Some(bingo_result * bingo_number);
                        }
                    }
                    _ => (),
                };

                // remove all bingo games while waiting to find the last one
                bingo_game
                    .boards
                    .retain(|bingo_board| match bingo_board.bingo_status {
                        BingoStatus::Bingo(_) => false,
                        BingoStatus::Bango => true,
                    });

                None
            })
            .unwrap()
    }
}

impl BingoBoard {
    fn incoming_number(&mut self, incoming_number: u32) -> BingoStatus {
        for (y, line) in self.bingo_lines.iter_mut().enumerate() {
            for (x, bingo_number) in line.iter_mut().enumerate() {
                if *bingo_number == BingoNumber::Unmarked(incoming_number) {
                    *bingo_number = BingoNumber::Marked(incoming_number);
                    self.bingo_status = self.bingo_status(y, x);
                    return self.bingo_status;
                }
            }
        }
        BingoStatus::Bango
    }

    fn bingo_status(&self, line: usize, column: usize) -> BingoStatus {
        if self.is_bingo_column(column) || self.is_bingo_line(line) {
            BingoStatus::Bingo(self.calc_bingo_value())
        } else {
            BingoStatus::Bango
        }
    }

    fn calc_bingo_value(&self) -> u32 {
        self.bingo_lines.iter().fold(0, |acc, line| {
            acc + line.iter().fold(0, |acc, number| match number {
                BingoNumber::Unmarked(number) => acc + number,
                BingoNumber::Marked(_) => acc,
            })
        })
    }

    fn is_bingo_line(&self, line: usize) -> bool {
        self.bingo_lines[line].iter().all(|number| match number {
            BingoNumber::Marked(_) => true,
            BingoNumber::Unmarked(_) => false,
        })
    }

    fn is_bingo_column(&self, column: usize) -> bool {
        self.bingo_lines.iter().all(|line| match line[column] {
            BingoNumber::Marked(_) => true,
            BingoNumber::Unmarked(_) => false,
        })
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use BingoNumber::{Marked, Unmarked};

    #[test]
    fn retrieve_bingo_example() {
        assert_eq!(
            retrieve_bingo_game(
                r#"7,4,9,5,11,17,23,2,0,14,21,24,10,16,13,6,15,25,12,22,18,20,8,19,3,26,1

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
             2  0 12  3  7
"#
            ),
            BingoGame {
                numbers: vec![
                    7, 4, 9, 5, 11, 17, 23, 2, 0, 14, 21, 24, 10, 16, 13, 6, 15, 25, 12, 22, 18,
                    20, 8, 19, 3, 26, 1
                ],
                boards: vec![
                    BingoBoard {
                        bingo_status: BingoStatus::Bango,
                        bingo_lines: vec![
                            vec![
                                Unmarked(22),
                                Unmarked(13),
                                Unmarked(17),
                                Unmarked(11),
                                Unmarked(0)
                            ],
                            vec![
                                Unmarked(8),
                                Unmarked(2),
                                Unmarked(23),
                                Unmarked(4),
                                Unmarked(24)
                            ],
                            vec![
                                Unmarked(21),
                                Unmarked(9),
                                Unmarked(14),
                                Unmarked(16),
                                Unmarked(7)
                            ],
                            vec![
                                Unmarked(6),
                                Unmarked(10),
                                Unmarked(3),
                                Unmarked(18),
                                Unmarked(5)
                            ],
                            vec![
                                Unmarked(1),
                                Unmarked(12),
                                Unmarked(20),
                                Unmarked(15),
                                Unmarked(19)
                            ],
                        ],
                    },
                    BingoBoard {
                        bingo_status: BingoStatus::Bango,
                        bingo_lines: vec![
                            vec![
                                Unmarked(3),
                                Unmarked(15),
                                Unmarked(0),
                                Unmarked(2),
                                Unmarked(22)
                            ],
                            vec![
                                Unmarked(9),
                                Unmarked(18),
                                Unmarked(13),
                                Unmarked(17),
                                Unmarked(5)
                            ],
                            vec![
                                Unmarked(19),
                                Unmarked(8),
                                Unmarked(7),
                                Unmarked(25),
                                Unmarked(23)
                            ],
                            vec![
                                Unmarked(20),
                                Unmarked(11),
                                Unmarked(10),
                                Unmarked(24),
                                Unmarked(4)
                            ],
                            vec![
                                Unmarked(14),
                                Unmarked(21),
                                Unmarked(16),
                                Unmarked(12),
                                Unmarked(6)
                            ],
                        ],
                    },
                    BingoBoard {
                        bingo_status: BingoStatus::Bango,
                        bingo_lines: vec![
                            vec![
                                Unmarked(14),
                                Unmarked(21),
                                Unmarked(17),
                                Unmarked(24),
                                Unmarked(4)
                            ],
                            vec![
                                Unmarked(10),
                                Unmarked(16),
                                Unmarked(15),
                                Unmarked(9),
                                Unmarked(19)
                            ],
                            vec![
                                Unmarked(18),
                                Unmarked(8),
                                Unmarked(23),
                                Unmarked(26),
                                Unmarked(20)
                            ],
                            vec![
                                Unmarked(22),
                                Unmarked(11),
                                Unmarked(13),
                                Unmarked(6),
                                Unmarked(5)
                            ],
                            vec![
                                Unmarked(2),
                                Unmarked(0),
                                Unmarked(12),
                                Unmarked(3),
                                Unmarked(7)
                            ],
                        ],
                    },
                ],
            }
        );
    }

    #[test]
    fn test_incoming_number() {
        let mut bingo_board = BingoBoard {
            bingo_status: BingoStatus::Bango,
            bingo_lines: vec![
                vec![Unmarked(14), Unmarked(21)],
                vec![Unmarked(10), Unmarked(16)],
            ],
        };

        // non present number
        assert_eq!(bingo_board.incoming_number(1234), BingoStatus::Bango);
        assert_eq!(
            bingo_board.bingo_lines,
            vec![
                vec![Unmarked(14), Unmarked(21),],
                vec![Unmarked(10), Unmarked(16),],
            ]
        );

        // present number
        assert_eq!(bingo_board.incoming_number(14), BingoStatus::Bango);
        assert_eq!(
            bingo_board.bingo_lines,
            vec![
                vec![Marked(14), Unmarked(21),],
                vec![Unmarked(10), Unmarked(16),],
            ]
        );

        // present number - vertical bingo
        let bingo = bingo_board.incoming_number(10);
        assert_eq!(
            bingo_board.bingo_lines,
            vec![
                vec![Marked(14), Unmarked(21),],
                vec![Marked(10), Unmarked(16),],
            ]
        );
        assert_eq!(bingo, BingoStatus::Bingo(37));

        // reset
        let mut bingo_board = BingoBoard {
            bingo_status: BingoStatus::Bango,
            bingo_lines: vec![
                vec![Unmarked(14), Unmarked(21)],
                vec![Unmarked(10), Unmarked(16)],
            ],
        };

        // present numbers - horizontal bingo
        assert_eq!(bingo_board.incoming_number(14), BingoStatus::Bango);
        let bingo = bingo_board.incoming_number(21);
        assert_eq!(
            bingo_board.bingo_lines,
            vec![
                vec![Marked(14), Marked(21),],
                vec![Unmarked(10), Unmarked(16),],
            ]
        );
        assert_eq!(bingo, BingoStatus::Bingo(26));
    }

    #[test]
    fn solve_part1_example() {
        let bingo_game = retrieve_bingo_game(
            r#"7,4,9,5,11,17,23,2,0,14,21,24,10,16,13,6,15,25,12,22,18,20,8,19,3,26,1

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
             2  0 12  3  7
"#,
        );

        assert_eq!(Part1::do_bingo_game(bingo_game), 4512);
    }

    #[test]
    fn solve_part2_example() {
        let bingo_game = retrieve_bingo_game(
            r#"7,4,9,5,11,17,23,2,0,14,21,24,10,16,13,6,15,25,12,22,18,20,8,19,3,26,1

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
             2  0 12  3  7
"#,
        );

        assert_eq!(Part2::do_bingo_game(bingo_game), 1924);
    }
}
