use std::{fs::File, io::{BufRead, BufReader}};

use fancy_regex::Regex;

fn main() {
    // Part 1
    // let vowel_regex = Regex::new("([aeiou])(.)*([aeiou])(.)*([aeiou])").unwrap();
    // let double_regex = Regex::new("(.)\\1").unwrap();

    // let result = BufReader::new(File::open("input").unwrap())
    //     .lines()
    //     .flatten()
    //     .filter(|line| vowel_regex.is_match(line).unwrap() )
    //     .filter(|line| double_regex.is_match(line).unwrap() )
    //     .filter(|line| {
    //         !(line.contains("ab") || line.contains("cd") || line.contains("pq") || line.contains("xy"))
    //     })
    //     .map(|line| {println!("{}", line); line})
    //     .count();


    // Part 2
    let pairs_regex = Regex::new("(.)(.)(.)*\\1\\2").unwrap();
    let seperated_regex = Regex::new("(.)(.)\\1").unwrap();

    let result = BufReader::new(File::open("input").unwrap())
        .lines()
        .flatten()
        .filter(|line| pairs_regex.is_match(line).unwrap() )
        .filter(|line| seperated_regex.is_match(line).unwrap() )
        .count();

    println!("There are {} nice strings.", result);
}
