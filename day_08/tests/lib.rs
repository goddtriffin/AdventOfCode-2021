use day_08::{part_one, part_two};
use utils::io::lines_from_file;

#[test]
fn d8_p1_t1() {
    let input: Vec<String> = lines_from_file("d8t1.txt")
        .unwrap()
        .iter()
        .map(|s| s.parse::<String>().unwrap())
        .collect();

    assert_eq!(26, part_one(&input));
}

#[test]
fn d8_p1_real() {
    let input: Vec<String> = lines_from_file("d8.txt")
        .unwrap()
        .iter()
        .map(|s| s.parse::<String>().unwrap())
        .collect();

    assert_eq!(534, part_one(&input));
}

#[test]
fn d8_p2_t1() {
    let input: Vec<String> = lines_from_file("d8t1.txt")
        .unwrap()
        .iter()
        .map(|s| s.parse::<String>().unwrap())
        .collect();

    assert_eq!(61229, part_two(&input));
}
