use std::{
    fs::File,
    io::{self, BufRead, BufReader},
    path::{Path, PathBuf},
};

pub fn lines_from_file(filename: impl AsRef<Path>) -> io::Result<Vec<String>> {
    let mut d = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    d.push("../input");
    d.push(filename);

    BufReader::new(File::open(d)?).lines().collect()
}
