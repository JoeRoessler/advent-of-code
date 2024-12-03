use regex::Regex;
use std::fs;

pub fn main() {
    // read input
    let input = fs::read_to_string("src/day_3_input.txt").expect("Could not read file");

    // Part 1
    let re = Regex::new(r"mul\((?<x>\d{1,3})+,(?<y>\d{1,3})+\)").unwrap();

    let mult_vals: Vec<i32> = re.captures_iter(&input).map(|caps| {
        let x: i32 = caps.name("x").unwrap().as_str().parse().unwrap();
        let y: i32 = caps.name("y").unwrap().as_str().parse().unwrap();
        x * y
    }).collect();

    let sum_of_vals: i32 = mult_vals.iter().sum();
    println!("sum: {:?}", sum_of_vals);


    // part 2
    // split by do() | don't()
    // then get indices of the do() and don't()?
    // only use the items in the vec that correspond to the do() indices

    // gather the do and don't indices
    let re_do = Regex::new(r"do\(\)|don't\(\)").unwrap();

    let mut chunks: Vec<String> = Vec::new();

    for cap in re_do.captures_iter(&input) {
        chunks.push(cap[0].to_string());
    }


    //split into lines
    let mut lines: Vec<String> = Vec::new();

    for part in re_do.split(&input) {
        lines.push(part.to_string());
    }

    // for each do() line
    // do part 1 and add it to a master sum
    let mut part2_sum: i32 = 0;

    for (idx, val) in chunks.iter().enumerate() {
        if val == "do()"{
            let mult_vals: Vec<i32> = re.captures_iter(&lines[idx+1]).map(|caps| {
                let x: i32 = caps.name("x").unwrap().as_str().parse().unwrap();
                let y: i32 = caps.name("y").unwrap().as_str().parse().unwrap();
                x * y
            }).collect();

            let temp_sum: i32 = mult_vals.iter().sum();
            part2_sum += temp_sum;
        } else {
            continue;
        }
    }

    println!("Part 2 sum {}", part2_sum);

}
