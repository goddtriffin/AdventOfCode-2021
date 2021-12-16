use submarine::Submarine;

use crate::command::Command;

pub mod command;
pub mod submarine;

pub fn day_two(input: &Vec<String>) {
    // setup
    let input: &Vec<Command> = &input
        .iter()
        .map(|s| s.parse::<Command>().unwrap())
        .collect();
    let mut submarine = Submarine::new();
    submarine.travel(input);

    // solutions
    println!("Day Two");
    println!("\tPart 1: {}", part_one(&submarine));
    println!("\tPart 2: {}", part_two(&submarine));
}

pub fn part_one(submarine: &Submarine) -> i32 {
    submarine.multiply()
}

pub fn part_two(submarine: &Submarine) -> i32 {
    submarine.aim_multiply()
}
