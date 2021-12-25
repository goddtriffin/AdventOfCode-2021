use std::str::FromStr;

use crate::entry::Entry;

pub mod entry;

pub fn day_08(input: &Vec<String>) {
    println!("Day 08");
    println!("\tPart 1: {}", part_one(input));
    println!("\tPart 2: {}", part_two(input));
}

pub fn part_one(input: &Vec<String>) -> usize {
    let mut count = 0;
    for line in input.iter() {
        let entry = Entry::from_str(line).unwrap();

        count += entry.count_unique_out_numbers();
    }

    count
}

pub fn part_two(_input: &Vec<String>) -> usize {
    0
}
