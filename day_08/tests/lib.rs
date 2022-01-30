use day_08::{entry::Entry, part_one, part_two};
use utils::io::lines_from_file;

#[test]
fn d8_p1_t1() {
    let entries: Vec<Entry> = lines_from_file("d8t1.txt")
        .unwrap()
        .iter()
        .map(|s| s.parse::<Entry>().unwrap())
        .collect();

    assert_eq!(26, part_one(&entries));
}

#[test]
fn d8_p1_real() {
    let entries: Vec<Entry> = lines_from_file("d8.txt")
        .unwrap()
        .iter()
        .map(|s| s.parse::<Entry>().unwrap())
        .collect();

    assert_eq!(534, part_one(&entries));
}

#[test]
fn d8_p2_t1() {
    let entries: Vec<Entry> = lines_from_file("d8t1.txt")
        .unwrap()
        .iter()
        .map(|s| s.parse::<Entry>().unwrap())
        .collect();

    assert_eq!(61229, part_two(&entries));
}
