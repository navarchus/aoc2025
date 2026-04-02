use std::fmt::Debug;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;

pub fn read_lines_from_file(filepath: impl AsRef<Path> + Debug) -> Vec<String> {
    let file = match File::open(&filepath) {
        Ok(file) => file,
        Err(e) => panic!("Problem opening file {e:?}"),
    };

    let buf = BufReader::new(file);

    buf.lines()
        .map(|l| l.unwrap_or(format!("Error opening file at path {:?}", &filepath)))
        .collect()
}
