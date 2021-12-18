use day_04::{bingo_game::BingoGame, part_one, part_two};
use utils::io::lines_from_file;

#[test]
fn d4_p1_t1() {
    let input = &lines_from_file("d4t1.txt")
        .unwrap()
        .iter()
        .map(|s| s.parse::<String>().unwrap())
        .collect();
    let mut bingo = BingoGame::new(input);

    assert_eq!(4512, part_one(&mut bingo));
}

#[test]
fn d4_p1_real() {
    let input = &lines_from_file("d4.txt")
        .unwrap()
        .iter()
        .map(|s| s.parse::<String>().unwrap())
        .collect();
    let mut bingo = BingoGame::new(input);

    assert_eq!(28082, part_one(&mut bingo));
}

#[test]
fn d4_p2_t1() {
    let input = &lines_from_file("d4t1.txt")
        .unwrap()
        .iter()
        .map(|s| s.parse::<String>().unwrap())
        .collect();
    let mut bingo = BingoGame::new(input);

    assert_eq!(1924, part_two(&mut bingo));
}

#[test]
fn d4_p2_real() {
    let input = &lines_from_file("d4.txt")
        .unwrap()
        .iter()
        .map(|s| s.parse::<String>().unwrap())
        .collect();
    let mut bingo = BingoGame::new(input);

    assert_eq!(8224, part_two(&mut bingo));
}
