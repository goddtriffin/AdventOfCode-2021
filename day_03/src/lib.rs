use utils::binary::{get_bit, set_bit};

pub fn day_three(input: &Vec<String>) {
    let binary_size = input[0].len();
    let input: Vec<u16> = input
        .iter()
        .map(|s| u16::from_str_radix(s, 2).unwrap())
        .collect();

    println!("Day Three");
    println!("\tPart 1: {}", part_one(&input, &binary_size));
    println!("\tPart 2: {}", part_two(&input, &binary_size));
}

pub fn part_one(input: &Vec<u16>, binary_size: &usize) -> i32 {
    let gamma_rate = get_gamma_rate(input);
    i32::from(gamma_rate) * i32::from(get_epsilon_rate(gamma_rate, binary_size))
}

pub fn part_two(input: &Vec<u16>, binary_size: &usize) -> i32 {
    i32::from(get_oxygen_generator_rating(input))
        * i32::from(get_co2_scrubber_rating(input, binary_size))
}

pub fn get_most_occurring_bit(input: &Vec<u16>, bit_offset: u16) -> u16 {
    if bit_offset < 1 {
        panic!(
            "Smallest bit_offset index is 1 - bit_offset {} was attempted",
            bit_offset
        );
    }
    if bit_offset > 16 {
        panic!(
            "Maximum u16 size is 16 bits long - bit_offset {} was attempted",
            bit_offset
        );
    }

    let mut count_1s = 0;
    for num in input.iter() {
        count_1s += get_bit(*num, bit_offset);
    }

    let fraction = f64::from(u16::try_from(input.len()).unwrap()) / f64::from(2);
    if f64::from(count_1s) >= fraction {
        1
    } else {
        0
    }
}

pub fn get_gamma_rate(input: &Vec<u16>) -> u16 {
    let mut gamma_rate: u16 = 0;
    // we only care about 12 bits, not the entire u16
    for bit_offset in 1u16..13 {
        if get_most_occurring_bit(input, bit_offset) == 1u16 {
            gamma_rate = set_bit(gamma_rate, bit_offset, true);
        } else {
            gamma_rate = set_bit(gamma_rate, bit_offset, false);
        }
    }
    gamma_rate
}

pub fn get_epsilon_rate(gamma_rate: u16, binary_size: &usize) -> u16 {
    // literally just the inverse
    let epsilon_rate = !gamma_rate;
    // have to wipe out the extra bits, since we're using u16 to represent 12 bits
    let epsilon_rate = epsilon_rate << (16 - binary_size) >> (16 - binary_size);
    epsilon_rate
}

pub fn filter_by_most_common(input: &Vec<u16>, bit_offset: u16) -> Vec<u16> {
    let most_occurring_bit = get_most_occurring_bit(input, bit_offset);
    let input = input
        .iter()
        .filter_map(|num| {
            if get_bit(*num, bit_offset) == most_occurring_bit {
                Some(*num)
            } else {
                None
            }
        })
        .collect();
    input
}

pub fn filter_by_least_common(input: &Vec<u16>, bit_offset: u16) -> Vec<u16> {
    let most_occurring_bit = get_most_occurring_bit(input, bit_offset);
    let least_occurring_bit = if most_occurring_bit == 1 { 0 } else { 1 };
    let input = input
        .iter()
        .filter_map(|num| {
            if get_bit(*num, bit_offset) == least_occurring_bit {
                Some(*num)
            } else {
                None
            }
        })
        .collect();
    input
}

pub fn get_oxygen_generator_rating(input: &Vec<u16>) -> u16 {
    let mut input_remaining = input.clone();
    let mut bit_offset = 12;
    while input_remaining.len() != 1 {
        input_remaining = filter_by_most_common(&input_remaining, bit_offset);
        bit_offset -= 1;
    }
    *input_remaining.get(0).unwrap()
}

pub fn get_co2_scrubber_rating(input: &Vec<u16>, binary_size: &usize) -> u16 {
    let mut input_remaining = input.clone();
    let mut bit_offset = u16::try_from(*binary_size).unwrap();
    while input_remaining.len() != 1 {
        input_remaining = filter_by_least_common(&input_remaining, bit_offset);
        bit_offset -= 1;
    }
    *input_remaining.get(0).unwrap()
}
