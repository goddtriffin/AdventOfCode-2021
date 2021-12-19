use std::{error, fmt, str::FromStr};

use crate::invalid_input_string_length::InvalidInputStringLength;

pub struct Point {
    pub x: usize,
    pub y: usize,
}

impl Point {
    pub fn new(x: usize, y: usize) -> Self {
        Point { x, y }
    }
}

impl FromStr for Point {
    type Err = Box<dyn error::Error>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let split = s.split(',').collect::<Vec<&str>>();
        if split.len() != 2 {
            return Err(InvalidInputStringLength(split.len()).into());
        }

        Ok(Point {
            x: split[0].parse::<usize>()?,
            y: split[1].parse::<usize>()?,
        })
    }
}

impl fmt::Display for Point {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{},{}", self.x, self.y)
    }
}
