use std::str::FromStr;

use day_06::{lanternfish_school::LanternfishSchool, part_one, part_two};
use utils::io::lines_from_file;

#[test]
fn d6_p1_t1() {
    let input: &Vec<String> = &lines_from_file("d6t1.txt")
        .unwrap()
        .iter()
        .map(|s| s.parse::<String>().unwrap())
        .collect();
    let mut school = LanternfishSchool::from_str(input[0].as_str()).unwrap();

    assert_eq!(5934, part_one(&mut school));
}

#[test]
fn d6_p1_real() {
    let input: &Vec<String> = &lines_from_file("d6.txt")
        .unwrap()
        .iter()
        .map(|s| s.parse::<String>().unwrap())
        .collect();
    let mut school = LanternfishSchool::from_str(input[0].as_str()).unwrap();

    assert_eq!(350605, part_one(&mut school));
}

#[test]
fn d6_p2_t1() {
    let input: &Vec<String> = &lines_from_file("d6t1.txt")
        .unwrap()
        .iter()
        .map(|s| s.parse::<String>().unwrap())
        .collect();
    let mut school = LanternfishSchool::from_str(input[0].as_str()).unwrap();

    part_one(&mut school);
    assert_eq!(26984457539, part_two(&mut school));
}

#[test]
fn d6_p2_real() {
    let input: &Vec<String> = &lines_from_file("d6.txt")
        .unwrap()
        .iter()
        .map(|s| s.parse::<String>().unwrap())
        .collect();
    let mut school = LanternfishSchool::from_str(input[0].as_str()).unwrap();

    part_one(&mut school);
    assert_eq!(1592778185024, part_two(&mut school));
}
