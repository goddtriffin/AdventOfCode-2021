use std::fmt;

use crate::bingo_board::BingoBoard;

pub struct BingoGame {
    pub drawn_numbers: Vec<usize>,
    pub boards: Vec<BingoBoard>,
}

impl BingoGame {
    pub fn new(input: &Vec<String>) -> Self {
        let mut input_iter = input.iter();
        let drawn_numbers = input_iter
            .next()
            .unwrap()
            .split(",")
            .map(|s| s.parse::<usize>().unwrap())
            .collect::<Vec<usize>>();

        let mut boards = vec![];
        while input_iter.next() == Some(&"".to_string()) {
            boards.push(BingoBoard::new(&mut input_iter));
        }

        BingoGame {
            drawn_numbers,
            boards,
        }
    }

    pub fn play(&mut self) -> Option<usize> {
        for drawn_number in self.drawn_numbers.iter() {
            for board in self.boards.iter_mut() {
                board.mark(*drawn_number);

                match board.check_win() {
                    Some(score) => {
                        return Some(drawn_number * score);
                    }
                    None => (),
                }
            }
        }

        None
    }
}

impl fmt::Display for BingoGame {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut builder = String::new();

        // boards
        for board in &self.boards {
            builder.push_str(format!("{}\n", board).as_str());
        }

        write!(f, "{}", builder)
    }
}
