pub fn day_one(input: &Vec<String>) {
    let input: Vec<i32> = input.iter().map(|s| s.parse::<i32>().unwrap()).collect();

    println!("Day One");
    println!("\tPart 1: {}", part_one(&input));
    println!("\tPart 2: {}", part_two(&input));
}

pub fn part_one(input: &Vec<i32>) -> i32 {
    let mut count = 0;
    for w in input.windows(2) {
        let [a, b]: [i32; 2] = w.try_into().unwrap();
        if b > a {
            count += 1;
        }
    }
    count
}

pub fn part_two(input: &Vec<i32>) -> i32 {
    let mut count = 0;
    let mut last_depth_sum = -1;
    for w in input.windows(3) {
        let [a, b, c]: [i32; 3] = w.try_into().unwrap();
        let depth_sum = a + b + c;

        // don't track the very first window
        if last_depth_sum == -1 {
            last_depth_sum = depth_sum;
            continue;
        }

        if depth_sum > last_depth_sum {
            count += 1;
        }
        last_depth_sum = depth_sum;
    }
    count
}
