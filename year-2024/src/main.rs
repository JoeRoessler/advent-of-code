#![allow(dead_code, unused_imports, unused_variables, unused_mut, unreachable_code)]
mod day_1;
mod day_2;
mod day_3;
mod day_4;
mod day_5;
mod day_7;
use std::time::Instant;

fn main() {
    let start = std::time::Instant::now();
    day_7::main();
    eprintln!("{:?}", start.elapsed());

}
