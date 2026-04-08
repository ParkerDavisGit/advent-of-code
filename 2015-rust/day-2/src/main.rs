use std::{cmp::min, fs::File, io::{BufRead, BufReader}};

fn main() {
    let paper_sum: i32 = BufReader::new(File::open("input").unwrap())
        .lines()
        .flatten()
        .map(|line| {
            let mut spliterator = line.split("x");
            let (x, y, z) = (
                spliterator.next().unwrap().parse::<i32>().unwrap(),
                spliterator.next().unwrap().parse::<i32>().unwrap(),
                spliterator.next().unwrap().parse::<i32>().unwrap()
            );

            let xy = x*y; let xz = x*z; let yz = y*z;
            let min = min(min(xy, xz), yz);

            2*(xy + xz + yz) + min
        })
        .sum();


    let ribbon_sum: i32 = BufReader::new(File::open("input").unwrap())
        .lines()
        .flatten()
        .map(|line| {
            let mut spliterator = line.split("x");
            let mut dims: Vec<i32> = vec![
                spliterator.next().unwrap().parse::<i32>().unwrap(),
                spliterator.next().unwrap().parse::<i32>().unwrap(),
                spliterator.next().unwrap().parse::<i32>().unwrap()
            ];

            dims.sort();

            let perimeter = 2*(dims[0] + dims[1]);
            let bow_length: i32 = dims.iter().product();

            perimeter + bow_length
        })
        .sum();


    println!("The elves need {} square feet of wrapping paper.", paper_sum);
    println!("The elves need {} feet of ribbon.", ribbon_sum);
}
