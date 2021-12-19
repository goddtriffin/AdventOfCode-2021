use std::{error, fmt, str::FromStr};

use crate::{invalid_input_string_length::InvalidInputStringLength, point::Point};

pub struct HydrothermalVent {
    pub start: Point,
    pub end: Point,
}

impl HydrothermalVent {
    pub fn new(start: Point, end: Point) -> Self {
        HydrothermalVent { start, end }
    }

    pub fn is_horizontal_or_vertical(&self) -> bool {
        self.start.x == self.end.x || self.start.y == self.end.y
    }

    pub fn get_points(&self) -> Vec<Point> {
        let mut points = vec![];

        let mut x_offset: isize = 0;
        let mut y_offset: isize = 0;

        points.push(Point::new(self.start.x, self.start.y));

        loop {
            if (self.start.x as isize) + x_offset != (self.end.x as isize) {
                if self.start.x < self.end.x {
                    x_offset += 1;
                }
                if self.end.x < self.start.x {
                    x_offset -= 1;
                }
            }

            if (self.start.y as isize) + y_offset != (self.end.y as isize) {
                if self.start.y < self.end.y {
                    y_offset += 1;
                }
                if self.end.y < self.start.y {
                    y_offset -= 1;
                }
            }

            points.push(Point::new(
                ((self.start.x as isize) + x_offset) as usize,
                ((self.start.y as isize) + y_offset) as usize,
            ));

            if (self.start.x as isize) + x_offset == (self.end.x as isize)
                && (self.start.y as isize) + y_offset == (self.end.y as isize)
            {
                break;
            }
        }

        points
    }
}

impl FromStr for HydrothermalVent {
    type Err = Box<dyn error::Error>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let split = s.split(' ').collect::<Vec<&str>>();
        if split.len() != 3 {
            return Err(InvalidInputStringLength(split.len()).into());
        }

        Ok(HydrothermalVent {
            start: Point::from_str(split[0])?,
            end: Point::from_str(split[2])?,
        })
    }
}

impl fmt::Display for HydrothermalVent {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} -> {}", self.start, self.end)
    }
}
