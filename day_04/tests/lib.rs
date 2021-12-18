use day_04::part_one;
use utils::io::lines_from_file;

#[test]
fn d4_p1_t1() {
    let input = &lines_from_file("d4t1.txt")
        .unwrap()
        .iter()
        .map(|s| s.parse::<String>().unwrap())
        .collect();

    assert_eq!(4512, part_one(&input));
}
