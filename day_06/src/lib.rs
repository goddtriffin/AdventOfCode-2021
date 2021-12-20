use std::str::FromStr;

use crate::lanternfish_school::LanternfishSchool;

pub mod lanternfish_school;

pub fn day_06(input: &Vec<String>) {
    let mut school = LanternfishSchool::from_str(input[0].as_str()).unwrap();

    println!("Day 06");
    println!("\tPart 1: {}", part_one(&mut school));
    println!("\tPart 2: {}", part_two(&mut school));
}

pub fn part_one(school: &mut LanternfishSchool) -> usize {
    school.simulate(80);
    school.size()
}

pub fn part_two(school: &mut LanternfishSchool) -> usize {
    school.simulate(176);
    school.size()
}
