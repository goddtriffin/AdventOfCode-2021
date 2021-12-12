pub fn day_one(input: Vec<String>) {
    println!("Day One");
    println!("\tPart 1: {}", part_one(input));
}

pub fn part_one(input: Vec<String>) -> i32 {
    let input: Vec<i32> = input.iter().map(|s| s.parse::<i32>().unwrap()).collect();

    let mut count = 0;

    for w in input.windows(2) {
        let [a, b]: [i32; 2] = w.try_into().unwrap();
        if b > a {
            count += 1;
        }
    }

    count
}
