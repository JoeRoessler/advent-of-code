/*
work right to left
only * and + are valid
so we're checking for the inverse
div and subtract
*/
use std::fs;

#[derive(Debug)]
struct Equation {
    total: f64,
    values: Vec<f64>,

}

fn is_calibrated(total: f64, values: Vec<f64>) -> bool {
    // base case is len == 1
    // return true is total == value
    // println!("TOTAL: {total} VALUES: {:?}", values);
    if values.len() == 1 {
        return total == values[0];
    }

    // work right to left
    // if we can divide the total by a number
    // continue testing
    // if total - number >= 0 continue
    if total % values[values.len() - 1] == 0.0
    && is_calibrated(total / values[values.len() - 1], values[..values.len() - 1].to_vec()) {
        return true;
    }
    if total - values[values.len() - 1] >= 0.0
    && is_calibrated(total - values[values.len() - 1], values[..values.len() - 1].to_vec()) {
        return true;
    }

    return false;

}

pub fn main() {
    // read in the data and format into
    // vec of Equation
    let input = fs::read_to_string("src/day_7_input.txt").expect("Could not read file");
    let mut equations: Vec<Equation> = Vec::new();
    for line in input.lines() {
        let parts: Vec<&str> = line.split(":").collect();

        if let Ok(total) = parts[0].trim().parse::<f64>() {
            let values: Vec<f64> = parts[1]
                .trim()
                .split_whitespace()
                .map(|s| s.parse::<f64>().unwrap())
                .collect();

        equations.push(Equation { total, values });
        }

    }

    // PART 1 ----------------------------------------
    let mut part1_total: f64 = 0.0;

    for eq in &equations {
        // println!("equation: {:?}", eq);
        if is_calibrated(eq.total, eq.values.clone()) {
            // println!("is true\n");
            part1_total += eq.total;
        }
    }
    println!("\nPART 1: {}", part1_total);

}
