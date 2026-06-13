use std::{fs::File, io::{BufRead, BufReader}};

type LightBoard = [[i16; 1000]; 1000];

struct Coordinates {
    x: usize,
    y: usize
}

fn parse_coordinates(coordinates: Option<&str>) -> Option<Coordinates> {
    // Given: `x,y`
    let existing_coords = match coordinates {
        Some(c) => c,
        None => return None,
    };

    let split_coordinates = match existing_coords.split_once(",") {
        Some(split) => split,
        None => return None,
    };

    let x: usize = match split_coordinates.0.parse::<usize>() {
        Ok(x) => x,
        Err(_) => return None,
    };

    let y: usize = match split_coordinates.1.parse::<usize>() {
        Ok(y) => y,
        Err(_) => return None,
    };
    
    Some(Coordinates { x, y })
}

fn parse_instruction(line: String) -> Option<(String, Coordinates, Coordinates)> {
    let mut split_line = line.split(" ");
    // Get light operation. `on`, `off`, or `toggle`
    let operation = match split_line.next() {
        Some("toggle") => "toggle",
        Some("turn") => {
            // Second round of this because there is a space
            //   in the input file.
            // Effectively filtering out the "turn" word.
            match split_line.next() {
                Some("on") => "on",
                Some("off") => "off",
                Some(_) => return None,
                None => return None
            }
        },
        Some(_) => return None, // invalid operation
        None => return None // broken input,
    }.to_string();

    let from_coords: Coordinates = match parse_coordinates(split_line.next()) {
        Some(coords) => coords,
        None => return None,
    };

    // Throw away the middle "through" in instruction
    let _ = split_line.next();

    let to_coords: Coordinates = match parse_coordinates(split_line.next()) {
        Some(coords) => coords,
        None => return None,
    };

    Some((operation, from_coords, to_coords))
}

fn set_lights(
    lights: &mut LightBoard, 
    from: Coordinates, 
    to: Coordinates, 
    value: i16
) {
    for x in from.x..=to.x {
        for y in from.y..=to.y {
            lights[x][y] += value;
            lights[x][y] = lights[x][y].clamp(0, 32767);
        }
    }
}

fn main() {
    // Initialize 2D array of 1 Million lights. 
    // (brute force, but couldn't think of a simple way to keep track of them)
    let mut lights: LightBoard = [[0i16; 1000]; 1000];

    // Get instructions to run on lights
    BufReader::new(File::open("input").unwrap())
        .lines()
        .flatten()
        // By now, the lines have been collected.
        // Format into usable list of tuples
        // (operation, Coordinates1, Coordinates2)
        .flat_map(|line| parse_instruction(line))
        .for_each(|instruction| {
            // Currently have instruction tuples.
            // All broken lines are filtered out above.
            match instruction.0.as_str() {
                "toggle" => {
                    set_lights(
                        &mut lights, 
                        instruction.1, 
                        instruction.2,
                        2
                    )
                },
                "on" => {
                    set_lights(
                        &mut lights, 
                        instruction.1, 
                        instruction.2, 
                        1
                    )
                },
                "off" => {
                    set_lights(
                        &mut lights, 
                        instruction.1, 
                        instruction.2, 
                        -1
                    )
                },
                _ => {}
            }
        });
    
    // Now, hopefully, the board is done
    let lit_lights: i64 = lights
        .iter()
        .map(|row| {
            row.iter().map(|num| *num as i64).sum::<i64>()
        })
        .sum();

    println!("The total brightness is {}", lit_lights);
}
