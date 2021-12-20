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

    calculate_total_fuel(input, least_fuel_position, &linear_fuel_cost)
}

pub fn part_two(input: &Vec<usize>) -> usize {
    let least_fuel_position =
        (input.iter().sum::<usize>() as f32 / input.len() as f32).floor() as usize;

    let floor_total_fuel = calculate_total_fuel(input, least_fuel_position, &quadratic_fuel_cost);
    let ceil_total_fuel =
        calculate_total_fuel(input, least_fuel_position + 1, &quadratic_fuel_cost);

    if floor_total_fuel < ceil_total_fuel {
        floor_total_fuel
    } else {
        ceil_total_fuel
    }
}

pub fn calculate_total_fuel(
    input: &Vec<usize>,
    least_fuel_position: usize,
    cost_function: &dyn Fn(usize, usize) -> usize,
) -> usize {
    let mut total_fuel = 0;
    for position in input.iter() {
        total_fuel += cost_function(least_fuel_position, *position);
    }

    total_fuel as usize
}

pub fn linear_fuel_cost(least_fuel_position: usize, horizontal_position: usize) -> usize {
    position_difference(least_fuel_position, horizontal_position)
}

pub fn quadratic_fuel_cost(least_fuel_position: usize, horizontal_position: usize) -> usize {
    let difference = position_difference(least_fuel_position, horizontal_position);
    (difference * (difference + 1)) / 2
}

pub fn position_difference(least_fuel_position: usize, horizontal_position: usize) -> usize {
    ((horizontal_position as isize) - (least_fuel_position as isize)).abs() as usize
}
