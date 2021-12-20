pub fn day_07(input: &Vec<String>) {
    let mut input: Vec<usize> = input[0]
        .split(",")
        .map(|s| s.parse::<usize>().unwrap())
        .collect();
    input.sort();

    println!("Day 07");
    println!("\tPart 1: {}", part_one(&input));
    println!("\tPart 2: {}", part_two(&input));
}

pub fn part_one(input: &Vec<usize>) -> usize {
    let mid = input.len() / 2;
    let least_fuel_position = input[mid];

    let mut total_fuel = 0;
    for position in input.iter() {
        total_fuel += ((*position as isize) - (least_fuel_position as isize)).abs();
    }

    total_fuel as usize
}

pub fn part_two(_input: &Vec<usize>) -> usize {
    2
}
