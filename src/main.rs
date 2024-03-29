#![allow(unused)]

use std::time::Instant;
fn main() {
    let start = Instant::now();
    // let mut numbers = vec![1, 2, 3, 4, 5];
    let numbers = [1, 2, 3, 4, 5];
    let numbers_squared: Vec<i32> = numbers.iter().map(|x| x * x).collect();
    // numbers.push(6);
    println!("{:?}", numbers_squared);

    let duration = start.elapsed();

    println!("Time: {:?}", duration);
}
