use std::collections::HashMap;


// Trait to house rules
// Has two methods:
// - One to test if all pre-reqs are met
// - And one to calculate and insert value
// Different operations will inherit this trait.
pub trait Operation {
    fn ready(instructions: &mut HashMap<&str, impl Operation>) -> bool;
    fn run(instructions: &mut HashMap<&str, impl Operation>) -> Result<u16, ()>;
}

/// An operation that assigns a single input value to a variable
pub struct Assign {
    
}

impl Assign {
    pub fn new() -> Self {
        todo!()
    }
}

impl Operation for Assign {
    fn ready(instructions: &mut HashMap<&str, impl Operation>) -> bool {
        todo!()
    }

    fn run(instructions: &mut HashMap<&str, impl Operation>) -> Result<u16, ()> {
        todo!()
    }
}


fn main() {
    println!("Hello, world!");
}
