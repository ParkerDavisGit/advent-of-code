use std::{fs::File, io::{BufReader, Read}};

#[derive(PartialEq, Eq, Debug)]
pub struct Pos(i32, i32);


fn main() {
    let mut instructions = String::new();
    let _ = BufReader::new(File::open("input").unwrap()).read_to_string(&mut instructions);

    let mut santa_pos: (i32, i32) = (0, 0);
    let mut robo_pos: (i32, i32) = (0, 0);
    let mut path: Vec<Pos> = Vec::new();

    for direction in instructions.split("").enumerate() {
        let delta = match direction.1 {
            ">" => (1, 0),
            "<" => (-1, 0),
            "^" => (0, 1),
            "v" => (0, -1),
            _ => { (0, 0) }
        };
        
        let new_pos = if direction.0 % 2 == 0 {
            santa_pos.0 += delta.0;
            santa_pos.1 += delta.1;
            Pos(santa_pos.0, santa_pos.1)
        }
        else {
            robo_pos.0 += delta.0;
            robo_pos.1 += delta.1;
            Pos(robo_pos.0, robo_pos.1)
        };

        if !path.contains(&new_pos) {
            path.push(new_pos);
        }
    }

    println!("Santa visited {} houses multiple times.", path.len());
}
