use day_one::{part_one, part_two};

#[test]
fn day_one_part_one() {
    assert_eq!(
        7,
        part_one(
            &vec!["199", "200", "208", "210", "200", "207", "240", "269", "260", "263"]
                .iter_mut()
                .map(|s| s.parse::<i32>().unwrap())
                .collect()
        )
    );
}

#[test]
fn day_one_part_two() {
    assert_eq!(
        5,
        part_two(
            &vec!["199", "200", "208", "210", "200", "207", "240", "269", "260", "263"]
                .iter_mut()
                .map(|s| s.parse::<i32>().unwrap())
                .collect()
        )
    );
}
