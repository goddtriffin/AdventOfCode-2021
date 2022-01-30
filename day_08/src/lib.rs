use std::str::FromStr;

use seven_segment_display::SevenSegmentDisplay;

use crate::entry::Entry;

pub mod entry;
pub mod seven_segment_display;

pub fn day_08(input: &Vec<String>) {
    let entries: &Vec<Entry> = &input
        .into_iter()
        .map(|line| Entry::from_str(line).unwrap())
        .collect();

    println!("Day 08");
    println!("\tPart 1: {}", part_one(entries));
    println!("\tPart 2: {}", part_two(entries));
}

pub fn part_one(entries: &Vec<Entry>) -> usize {
    let mut count = 0;
    for entry in entries.iter() {
        count += entry.count_unique_out_numbers();
    }

    count
}

pub fn part_two(entries: &Vec<Entry>) -> usize {
    let mut sum = 0;

    for entry in entries.iter() {
        println!("{}", entry);
        let display = SevenSegmentDisplay::new(entry);
        println!("{}", display);
    }

    sum
}
