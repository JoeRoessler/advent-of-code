use std::{fs, vec};


pub fn main() {
    let file_path = "src/day_2_input.txt";
    let mut safe_reports = 0;

    // read the data as string
    let data  = fs::read_to_string(file_path).expect("unable to read input file");

    // parse each line into a vector
    // examine
    for line in data.lines() {
        // parse the line into a vector
        let report: Vec<i32> = line.split_whitespace().filter_map(|x| x.parse::<i32>().ok()).collect();

        // create a shifted version of the vector
        let mut report_shift: Vec<i32> = report[..report.len()-1].to_vec();

        // println!("original vector: {:?}\nshifted vector {:?}", report, report_shift);

        // do element-wise subtraction
        // using only indices 1.. on the original vector
        let levels: Vec<i32> = report[1..].iter().zip(report_shift.iter()).map(|(x, y)| x - y).collect();

        // println!("levels {:?}", levels);

        // check if all levels are same sign
        // AND 1<= level <= 3
        // if it passes, add one to safe_reports
        if same_sign(&levels) && level_check(&&levels) {
            safe_reports += 1
        } else {
            continue;
        }
    }

    println!("There are {} safe reports", safe_reports);

}

fn same_sign(vector: &[i32]) -> bool {
    if vector.iter().all(|&x| x > 0) {
        true
    } else if  vector.iter().all(|&x| x < 0) {
        true
    } else {
        false
    }
}

fn level_check(vector: &[i32]) -> bool {
    vector.iter().all(|&x| 1 <= x.abs() && x.abs() <= 3)
}
