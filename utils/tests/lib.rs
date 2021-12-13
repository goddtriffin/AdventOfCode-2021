use utils::lines_from_file;

#[test]
fn lines_from_file_success() {
    assert_eq!(
        vec!["199", "200", "208", "210", "200", "207", "240", "269", "260", "263"]
            .iter_mut()
            .map(|s| s.to_string())
            .collect::<Vec<String>>(),
        lines_from_file("d1t1.txt").unwrap()
    );
}
