#![allow(dead_code, unused_imports, unused_variables, unused_mut, unreachable_code)]
mod day_1;
mod day_2;
mod day_3;
mod day_4;
mod day_5;
use std::time::Instant;

fn main() {
    let start = std::time::Instant::now();
    day_5::main();
    eprintln!("{:?}", start.elapsed());

}
