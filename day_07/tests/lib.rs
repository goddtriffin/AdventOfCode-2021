use day_07::{part_one, part_two};
use utils::io::lines_from_file;

#[test]
fn d7_p1_t1() {
    let input: Vec<String> = lines_from_file("d7t1.txt")
        .unwrap()
        .iter()
        .map(|s| s.parse::<String>().unwrap())
        .collect();
    let mut input: Vec<usize> = input[0]
        .split(",")
        .map(|s| s.parse::<usize>().unwrap())
        .collect();
    input.sort();

    assert_eq!(37, part_one(&input));
}

#[test]
fn d7_p1_real() {
    let input: Vec<String> = lines_from_file("d7.txt")
        .unwrap()
        .iter()
        .map(|s| s.parse::<String>().unwrap())
        .collect();
    let mut input: Vec<usize> = input[0]
        .split(",")
        .map(|s| s.parse::<usize>().unwrap())
        .collect();
    input.sort();

    assert_eq!(364898, part_one(&input));
}

#[test]
fn d7_p2_t1() {
    let input: Vec<String> = lines_from_file("d7t1.txt")
        .unwrap()
        .iter()
        .map(|s| s.parse::<String>().unwrap())
        .collect();
    let mut input: Vec<usize> = input[0]
        .split(",")
        .map(|s| s.parse::<usize>().unwrap())
        .collect();
    input.sort();

    assert_eq!(168, part_two(&input));
}

#[test]
fn d7_p2_real() {
    let input: Vec<String> = lines_from_file("d7.txt")
        .unwrap()
        .iter()
        .map(|s| s.parse::<String>().unwrap())
        .collect();
    let mut input: Vec<usize> = input[0]
        .split(",")
        .map(|s| s.parse::<usize>().unwrap())
        .collect();
    input.sort();

    assert_eq!(104149091, part_two(&input));
}
