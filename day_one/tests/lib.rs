use day_one::{part_one, part_two};
use utils::lines_from_file;

#[test]
fn d1_p1_t1() {
    let input = &lines_from_file("d1t1.txt")
        .unwrap()
        .iter()
        .map(|s| s.parse::<i32>().unwrap())
        .collect();

    assert_eq!(7, part_one(&input));
}

#[test]
fn d1_p1_real() {
    let input = &lines_from_file("d1.txt")
        .unwrap()
        .iter()
        .map(|s| s.parse::<i32>().unwrap())
        .collect();

    assert_eq!(1692, part_one(&input));
}

#[test]
fn d1_p2_t1() {
    let input = &lines_from_file("d1t1.txt")
        .unwrap()
        .iter()
        .map(|s| s.parse::<i32>().unwrap())
        .collect();

    assert_eq!(5, part_two(&input));
}

#[test]
fn d1_p2_real() {
    let input = &lines_from_file("d1.txt")
        .unwrap()
        .iter()
        .map(|s| s.parse::<i32>().unwrap())
        .collect();

    assert_eq!(1724, part_two(&input));
}
