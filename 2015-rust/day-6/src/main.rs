use std::{fs::File, io::{BufRead, BufReader}};

fn main() {
    let result = BufReader::new(File::open("input").unwrap())
        .lines()
        .flatten();
}
