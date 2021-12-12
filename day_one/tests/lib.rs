use day_one::part_one;

#[test]
fn it_works() {
    assert_eq!(
        7,
        part_one(
            vec!["199", "200", "208", "210", "200", "207", "240", "269", "260", "263"]
                .iter_mut()
                .map(|s| s.to_string())
                .collect()
        )
    );
}
