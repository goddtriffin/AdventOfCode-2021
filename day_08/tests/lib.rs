use day_08::part_one;
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
