pub fn day_three(input: &Vec<String>) {
    let binary_size = input[0].len();
    let input: Vec<u16> = input
        .iter()
        .map(|s| u16::from_str_radix(s, 2).unwrap())
        .collect();

    println!("Day Three");
    println!("\tPart 1: {}", part_one(&input, &binary_size));
    println!("\tPart 2: {}", part_two(&input));
}

pub fn part_one(input: &Vec<u16>, binary_size: &usize) -> i32 {
    let mut counter: [usize; 12] = [0; 12];
    for num in input.iter() {
        for i in 0..12 {
            counter[11 - i] += usize::from(num >> i & 1);
        }
    }

    let mut gamma_rate: u16 = 0;
    for (index, count) in counter.iter().enumerate() {
        let mask = 1 << (11 - index);

        if *count >= input.len() / 2 {
            gamma_rate |= mask;
        }
    }

    let epsilon_rate = !gamma_rate << (16 - binary_size) >> (16 - binary_size);

    i32::from(gamma_rate) * i32::from(epsilon_rate)
}

pub fn part_two(input: &Vec<u16>) -> i32 {
    println!("{}", get_most_occuring_bit(input, 1));
    1
}

fn get_most_occuring_bit(input: &Vec<u16>, bit_offset: u16) -> u16 {
    let mut count_1s = 0;
    for num in input.iter() {
        print_binary(*num);
        println!("New count: {}", count_1s);
        count_1s += num >> bit_offset & 1;
    }
    count_1s
}

fn print_binary(num: u16) {
    print!("{}:\t", num);
    for i in 0..16 {
        print!("{}", num >> (15 - i) & 1);
    }
    println!();
}
