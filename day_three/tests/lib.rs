use day_three::{part_one, part_two};
use utils::lines_from_file;

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
    let input: Vec<u16> = input
        .iter()
        .map(|s| u16::from_str_radix(s, 2).unwrap())
        .collect();

    assert_eq!(230, part_two(&input));
}
