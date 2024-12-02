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

        if assess_safety(&report) {
            safe_reports += 1
        } else {
            continue;
        }

    }

    println!("There are {} safe reports", safe_reports);

    // part 2
    // parse each line into a vector
    // examine
    // if it doesn't pass, start removing elements and examining
    let mut safe_damper_reports = 0;
    for line in data.lines() {
        // parse the line into a vector
        let report: Vec<i32> = line.split_whitespace().filter_map(|x| x.parse::<i32>().ok()).collect();

        if assess_safety(&report) {
            safe_damper_reports += 1
        } else {
            // start removing elements
            for i_remove in 0..report.len() {
                // remove element i_remove
                let modified_report: Vec<i32> = report
                    .iter()
                    .enumerate()
                    .filter(|&(i, _)| i != i_remove)
                    .map(|(_, &x)| x)
                    .collect();

                // examine
                if assess_safety(&modified_report) {
                    safe_damper_reports += 1;
                    break;
                } else {
                    continue;
                }
            }
        }

    }

    println!("There are {} safe-dampered reports", safe_damper_reports);

}

fn assess_safety(report: &[i32]) -> bool {
    // shifted version of report
    let mut report_shift: Vec<i32> = report[..report.len()-1].to_vec();
    // do element-wise subtraction
    // using only indices 1.. on the original vector
    let levels: Vec<i32> = report[1..].iter().zip(report_shift.iter()).map(|(x, y)| x - y).collect();

    // check if all levels are same sign
    // AND 1<= level <= 3
    // if it passes, add one to safe_reports
    if same_sign(&levels) && level_check(&&levels) {
        true
    } else {
        false
    }

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
