use std::{
    fs::File,
    io::{self, BufRead},
};

pub fn load_lines(file_path: &str) -> impl Iterator<Item = String> {
    let file = File::open(file_path).unwrap();
    io::BufReader::new(file)
        .lines()
        .filter_map(|line| line.ok())
}
