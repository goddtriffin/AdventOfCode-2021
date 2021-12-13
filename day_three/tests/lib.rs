use day_three::part_one;
use utils::lines_from_file;

#[test]
fn d3_p1_t1() {
    let input = &lines_from_file("d3t1.txt")
        .unwrap()
        .iter()
        .map(|s| s.parse::<String>().unwrap())
        .collect();

    assert_eq!(198, part_one(&input));
}

#[test]
fn d3_p1_real() {
    let input = &lines_from_file("d3.txt")
        .unwrap()
        .iter()
        .map(|s| s.parse::<String>().unwrap())
        .collect();

    assert_eq!(775304, part_one(&input));
}
