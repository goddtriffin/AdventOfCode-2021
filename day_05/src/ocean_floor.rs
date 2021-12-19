use std::{fmt, str::FromStr};

use crate::hydrothermal_vent::HydrothermalVent;

pub struct OceanFloor {
    pub map: Vec<usize>,
    pub width: usize,
    pub height: usize,
    pub hydrothermal_vents: Vec<HydrothermalVent>,
}

impl OceanFloor {
    pub fn chart_horizontal_and_vertical_vents(&mut self) {
        for vent in self.hydrothermal_vents.iter() {
            if vent.is_horizontal_or_vertical() {
                for point in vent.get_points().iter() {
                    let index = self.generate_index(point.y, point.x);
                    self.map[index] += 1;
                }
            }
        }
    }

    pub fn chart_diagonal_vents(&mut self) {
        for vent in self.hydrothermal_vents.iter() {
            if !vent.is_horizontal_or_vertical() {
                for point in vent.get_points().iter() {
                    let index = self.generate_index(point.y, point.x);
                    self.map[index] += 1;
                }
            }
        }
    }

    pub fn count_danger_points(&self) -> usize {
        let mut count = 0;
        for danger_level in self.map.iter() {
            if *danger_level >= 2 {
                count += 1;
            }
        }
        count
    }

    pub fn generate_index(&self, y: usize, x: usize) -> usize {
        (y * self.width) + x
    }
}

impl From<&Vec<String>> for OceanFloor {
    fn from(item: &Vec<String>) -> Self {
        let mut max_width = 0;
        let mut max_height = 0;

        let lines = item
            .iter()
            .filter_map(|line| {
                if line == "" {
                    return None;
                }

                let line = HydrothermalVent::from_str(line).unwrap();

                if line.start.x > max_width {
                    max_width = line.start.x;
                }
                if line.end.x > max_width {
                    max_width = line.end.x;
                }

                if line.start.y > max_height {
                    max_height = line.start.y;
                }
                if line.end.y > max_height {
                    max_height = line.end.y;
                }

                Some(line)
            })
            .collect::<Vec<HydrothermalVent>>();

        // must add 1 since zero-indexed
        max_width += 1;
        max_height += 1;

        let mut map = vec![];
        map.resize(max_width * max_height, 0);

        OceanFloor {
            map,
            width: max_width,
            height: max_height,
            hydrothermal_vents: lines,
        }
    }
}

impl fmt::Display for OceanFloor {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut builder = String::new();
        for y in 0..self.height {
            for x in 0..self.width {
                let index = self.generate_index(y, x);
                if self.map[index] == 0 {
                    builder.push('.');
                } else {
                    builder.push_str(format!("{}", self.map[index]).as_str());
                }

                if x == self.width - 1 {
                    builder.push('\n');
                }
            }
        }
        write!(f, "{}", builder)
    }
}
