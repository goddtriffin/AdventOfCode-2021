use day_02::{command::Command, part_one, part_two, submarine::Submarine};
use utils::io::lines_from_file;

#[test]
fn d2_p1_t1() {
    let input = &lines_from_file("d2t1.txt")
        .unwrap()
        .iter()
        .map(|s| s.parse::<Command>().unwrap())
        .collect();
    let mut submarine = Submarine::new();
    submarine.travel(input);

    assert_eq!(150, part_one(&submarine));
}

#[test]
fn d2_p1_real() {
    let input = &lines_from_file("d2.txt")
        .unwrap()
        .iter()
        .map(|s| s.parse::<Command>().unwrap())
        .collect();
    let mut submarine = Submarine::new();
    submarine.travel(input);

    assert_eq!(1636725, part_one(&submarine));
}

#[test]
fn d2_p2_t1() {
    let input = &lines_from_file("d2t1.txt")
        .unwrap()
        .iter()
        .map(|s| s.parse::<Command>().unwrap())
        .collect();
    let mut submarine = Submarine::new();
    submarine.travel(input);

    assert_eq!(900, part_two(&submarine));
}

#[test]
fn d2_p2_real() {
    let input = &lines_from_file("d2.txt")
        .unwrap()
        .iter()
        .map(|s| s.parse::<Command>().unwrap())
        .collect();
    let mut submarine = Submarine::new();
    submarine.travel(input);

    assert_eq!(1872757425, part_two(&submarine));
}
