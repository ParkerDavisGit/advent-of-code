use std::{fs::File, io::{BufReader, Read}};

fn main() {
    let floor: i32 = BufReader::new(File::open("input").expect("Can't open file"))
        .bytes()
        .flatten()
        .map(|byte| {
            match byte {
                b'(' => 1i32,
                b')' => -1i32,
                _ => 032,
            }
        })
        .sum();
    
    let basement_pos: i32 = {
        let mut sum = 0i32;
        let mut pos: usize = 0usize;
        for byte in BufReader::new(File::open("input").expect("Can't open file")).bytes().flatten().enumerate() {
            sum += match byte.1 {
                b'(' => 1i32,
                b')' => -1i32,
                _ => 032,
            };
            if sum < 0i32 {
                pos =  byte.0 + 1;
                break
            }
        }
        pos as i32
    };

    println!("The floor santa arrives at is: {}", floor);
    println!("The position santa reaches the basement at is: {}", basement_pos);
}
