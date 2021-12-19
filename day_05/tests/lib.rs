use day_05::{ocean_floor::OceanFloor, part_one, part_two};
use utils::io::lines_from_file;

#[test]
fn d5_p1_t1() {
    let input = &lines_from_file("d5t1.txt")
        .unwrap()
        .iter()
        .map(|s| s.parse::<String>().unwrap())
        .collect();
    let mut ocean_floor = OceanFloor::from(input);

    assert_eq!(5, part_one(&mut ocean_floor));
}

#[test]
fn d5_p1_real() {
    let input = &lines_from_file("d5.txt")
        .unwrap()
        .iter()
        .map(|s| s.parse::<String>().unwrap())
        .collect();
    let mut ocean_floor = OceanFloor::from(input);

    assert_eq!(5442, part_one(&mut ocean_floor));
}

#[test]
fn d5_p2_t1() {
    let input = &lines_from_file("d5t1.txt")
        .unwrap()
        .iter()
        .map(|s| s.parse::<String>().unwrap())
        .collect();
    let mut ocean_floor = OceanFloor::from(input);

    part_one(&mut ocean_floor);
    assert_eq!(12, part_two(&mut ocean_floor));
}

#[test]
fn d5_p2_real() {
    let input = &lines_from_file("d5.txt")
        .unwrap()
        .iter()
        .map(|s| s.parse::<String>().unwrap())
        .collect();
    let mut ocean_floor = OceanFloor::from(input);

    part_one(&mut ocean_floor);
    assert_eq!(19571, part_two(&mut ocean_floor));
}
