use crate::ocean_floor::OceanFloor;

pub mod hydrothermal_vent;
pub mod invalid_input_string_length;
pub mod ocean_floor;
pub mod point;

pub fn day_05(input: &Vec<String>) {
    let mut ocean_floor = OceanFloor::from(input);

    println!("Day 05");
    println!("\tPart 1: {}", part_one(&mut ocean_floor));
    println!("\tPart 2: {}", part_two(&mut ocean_floor));
}

pub fn part_one(ocean_floor: &mut OceanFloor) -> usize {
    ocean_floor.chart_horizontal_and_vertical_vents();
    ocean_floor.count_danger_points()
}

pub fn part_two(ocean_floor: &mut OceanFloor) -> usize {
    ocean_floor.chart_diagonal_vents();
    ocean_floor.count_danger_points()
}
