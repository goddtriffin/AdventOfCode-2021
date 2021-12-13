pub fn day_three(input: &Vec<String>) {
    println!("Day Three");
    println!("\tPart 1: {}", part_one(&input));
}

pub fn part_one(input: &Vec<String>) -> i32 {
    let binary_size = input[0].len();
    let input: Vec<u16> = input
        .iter()
        .map(|s| u16::from_str_radix(s, 2).unwrap())
        .collect();
    let input_length = input.len();

    let mut counter: [usize; 12] = [0; 12];
    for num in input {
        for i in 0..12 {
            counter[11 - i] += usize::from(num >> i & 1);
        }
    }

    let mut gamma_rate: u16 = 0;
    for (index, count) in counter.iter().enumerate() {
        let mask = 1 << (11 - index);

        if *count > input_length / 2 {
            gamma_rate |= mask;
        }
    }

    let epsilon_rate = !gamma_rate << (16 - binary_size) >> (16 - binary_size);

    i32::from(gamma_rate) * i32::from(epsilon_rate)
}
