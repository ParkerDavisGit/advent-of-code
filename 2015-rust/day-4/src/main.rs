use std::str::FromStr;

const INPUT: &str = "bgvyzdsv";

fn main() {
    let mut result: String = String::new();
    let mut num = 0;

    while !result.starts_with("000000") {
        num += 1;
        result = format!("{:?}", md5::compute(INPUT.to_owned() + &num.to_string()));
    }
    
    println!("The first number with a hash that starts with `000000` is {:?}", num);
}
