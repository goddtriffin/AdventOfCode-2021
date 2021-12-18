use std::{fmt, slice::Iter};

pub struct BingoBoard {
    pub board: Vec<usize>,
    pub marked: Vec<bool>,
    pub width: usize,
    pub height: usize,
}

impl BingoBoard {
    pub fn new(input_iter: &mut Iter<String>) -> Self {
        let width: usize = 5;
        let height: usize = 5;

        // generate board
        let mut board = vec![];
        for _ in 0..height {
            board.extend(
                input_iter
                    .next()
                    .unwrap()
                    .split_ascii_whitespace()
                    .map(|s| s.parse::<usize>().unwrap())
                    .collect::<Vec<usize>>(),
            );
        }

        // initialize marks
        let mut marked = vec![];
        marked.resize(width * height, false);

        BingoBoard {
            board,
            marked,
            width,
            height,
        }
    }

    pub fn mark(&mut self, drawn_number: usize) {
        for y in 0..self.height {
            for x in 0..self.width {
                let index = self.generate_index(y, x);
                if self.board.get(index) == Some(&drawn_number) {
                    self.marked[index] = true;
                }
            }
        }
    }

    /// if win is found, the returned usize is the sum of all unmarked numbers on the board
    pub fn check_win(&self) -> Option<usize> {
        match self.check_horizontal_win() {
            Some(score) => {
                return Some(score);
            }
            None => (),
        }

        match self.check_vertical_win() {
            Some(score) => {
                return Some(score);
            }
            None => (),
        }

        None
    }

    /// if win is found, the returned usize is the sum of all unmarked numbers on the board
    fn check_horizontal_win(&self) -> Option<usize> {
        for y in 0..self.height {
            let mut found = true;
            for x in 0..self.width {
                let index = self.generate_index(y, x);
                if !self.marked[index] {
                    found = false;
                    break;
                }
            }

            if found {
                return Some(self.unmarked_sum());
            }
        }

        None
    }

    /// if win is found, the returned usize is the sum of all unmarked numbers on the board
    fn check_vertical_win(&self) -> Option<usize> {
        for x in 0..self.width {
            let mut found = true;
            for y in 0..self.height {
                let index = self.generate_index(y, x);
                if !self.marked[index] {
                    found = false;
                    break;
                }
            }

            if found {
                return Some(self.unmarked_sum());
            }
        }

        None
    }

    fn unmarked_sum(&self) -> usize {
        let mut sum = 0;
        for y in 0..self.height {
            for x in 0..self.width {
                let index = self.generate_index(y, x);
                if !self.marked[index] {
                    sum += self.board[index];
                }
            }
        }

        sum
    }

    fn generate_index(&self, y: usize, x: usize) -> usize {
        (y * self.width) + x
    }
}

impl fmt::Display for BingoBoard {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut builder = String::new();
        for y in 0..self.height {
            for x in 0..self.width {
                let index = self.generate_index(y, x);
                if self.marked[index] {
                    builder.push_str(format!("[{}]", self.board[index]).as_str());
                } else {
                    builder.push_str(format!("{}", self.board[index]).as_str());
                }

                if x != self.width - 1 {
                    builder.push('\t');
                } else {
                    builder.push('\n');
                }
            }
        }
        write!(f, "{}", builder)
    }
}
