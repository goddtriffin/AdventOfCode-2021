use day_03::{
    filter_by_least_common, filter_by_most_common, get_co2_scrubber_rating, get_epsilon_rate,
    get_gamma_rate, get_most_occurring_bit, get_oxygen_generator_rating, part_one, part_two,
};
use utils::io::lines_from_file;

#[test]
fn d3_p1_t1() {
    let input: &Vec<String> = &lines_from_file("d3t1.txt")
        .unwrap()
        .iter()
        .map(|s| s.parse::<String>().unwrap())
        .collect();
    let binary_size = input[0].len();
    let input: Vec<u16> = input
        .iter()
        .map(|s| u16::from_str_radix(s, 2).unwrap())
        .collect();

    assert_eq!(198, part_one(&input, &binary_size));
}

#[test]
fn d3_p1_real() {
    let input: &Vec<String> = &lines_from_file("d3.txt")
        .unwrap()
        .iter()
        .map(|s| s.parse::<String>().unwrap())
        .collect();
    let binary_size = input[0].len();
    let input: Vec<u16> = input
        .iter()
        .map(|s| u16::from_str_radix(s, 2).unwrap())
        .collect();

    assert_eq!(775304, part_one(&input, &binary_size));
}

#[test]
fn d3_p2_t1() {
    let input: &Vec<String> = &lines_from_file("d3t1.txt")
        .unwrap()
        .iter()
        .map(|s| s.parse::<String>().unwrap())
        .collect();
    let binary_size = input[0].len();
    let input: Vec<u16> = input
        .iter()
        .map(|s| u16::from_str_radix(s, 2).unwrap())
        .collect();

    assert_eq!(230, part_two(&input, &binary_size));
}

#[test]
fn d3_p2_real() {
    let input: &Vec<String> = &lines_from_file("d3.txt")
        .unwrap()
        .iter()
        .map(|s| s.parse::<String>().unwrap())
        .collect();
    let binary_size = input[0].len();
    let input: Vec<u16> = input
        .iter()
        .map(|s| u16::from_str_radix(s, 2).unwrap())
        .collect();

    assert_eq!(1370737, part_two(&input, &binary_size));
}

#[test]
fn get_most_occuring_bit_success() {
    let input: &Vec<u16> = &lines_from_file("d3t1.txt")
        .unwrap()
        .iter()
        .map(|s| u16::from_str_radix(s, 2).unwrap())
        .collect();

    assert_eq!(0, get_most_occurring_bit(&input, 1));
    assert_eq!(1, get_most_occurring_bit(&input, 2));
    assert_eq!(1, get_most_occurring_bit(&input, 3));
    assert_eq!(0, get_most_occurring_bit(&input, 4));
    assert_eq!(1, get_most_occurring_bit(&input, 5));

    // test that the fraction is being set properly
    assert_eq!(
        0,
        get_most_occurring_bit(
            &vec![0b11110, 0b10110, 0b10111, 0b10101, 0b11100, 0b10000, 0b11001],
            4
        )
    );

    // test that defaults to 1 in a tie
    assert_eq!(1, get_most_occurring_bit(&vec![0b10110, 0b10111], 1));
}

#[test]
fn get_gamma_rate_success() {
    let input: &Vec<u16> = &lines_from_file("d3t1.txt")
        .unwrap()
        .iter()
        .map(|s| u16::from_str_radix(s, 2).unwrap())
        .collect();

    assert_eq!(22, get_gamma_rate(&input));
}

#[test]
fn get_epsilon_rate_success() {
    let input: &Vec<String> = &lines_from_file("d3t1.txt")
        .unwrap()
        .iter()
        .map(|s| s.parse::<String>().unwrap())
        .collect();
    let binary_size = input[0].len();
    let input: Vec<u16> = input
        .iter()
        .map(|s| u16::from_str_radix(s, 2).unwrap())
        .collect();

    assert_eq!(9, get_epsilon_rate(get_gamma_rate(&input), &binary_size));
}

#[test]
fn filter_by_most_common_success() {
    let input: &Vec<u16> = &lines_from_file("d3t1.txt")
        .unwrap()
        .iter()
        .map(|s| u16::from_str_radix(s, 2).unwrap())
        .collect();

    let input = &filter_by_most_common(&input, 5);
    assert_eq!(
        vec![0b11110, 0b10110, 0b10111, 0b10101, 0b11100, 0b10000, 0b11001],
        *input
    );

    let input = &filter_by_most_common(&input, 4);
    assert_eq!(vec![0b10110, 0b10111, 0b10101, 0b10000], *input);

    let input = &filter_by_most_common(&input, 3);
    assert_eq!(vec![0b10110, 0b10111, 0b10101], *input);

    let input = &filter_by_most_common(&input, 2);
    assert_eq!(vec![0b10110, 0b10111], *input);

    let input = &filter_by_most_common(&input, 1);
    assert_eq!(vec![0b10111], *input);
}

#[test]
fn filter_by_least_common_success() {
    let input: &Vec<u16> = &lines_from_file("d3t1.txt")
        .unwrap()
        .iter()
        .map(|s| u16::from_str_radix(s, 2).unwrap())
        .collect();

    let input = &filter_by_least_common(&input, 5);
    assert_eq!(vec![0b00100, 0b01111, 0b00111, 0b00010, 0b01010], *input);

    let input = &filter_by_least_common(&input, 4);
    assert_eq!(vec![0b01111, 0b01010], *input);

    let input = &filter_by_least_common(&input, 3);
    assert_eq!(vec![0b01010], *input);
}

#[test]
fn get_oxygen_generator_rating_success() {
    let input: &Vec<u16> = &lines_from_file("d3t1.txt")
        .unwrap()
        .iter()
        .map(|s| u16::from_str_radix(s, 2).unwrap())
        .collect();

    assert_eq!(23, get_oxygen_generator_rating(&input));
}

#[test]
fn get_c02_scrubber_rating_success() {
    let input: &Vec<String> = &lines_from_file("d3t1.txt")
        .unwrap()
        .iter()
        .map(|s| s.parse::<String>().unwrap())
        .collect();
    let binary_size = input[0].len();
    let input: Vec<u16> = input
        .iter()
        .map(|s| u16::from_str_radix(s, 2).unwrap())
        .collect();

    assert_eq!(10, get_co2_scrubber_rating(&input, &binary_size));
}
