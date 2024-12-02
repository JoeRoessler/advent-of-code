use std::fs;

fn part1(left_list: &mut [i32], right_list: &mut [i32]) -> i32{
    left_list.sort();
    right_list.sort();

    // for each element compute the distance
    // this is abs element-wise subtraction
    let distance: Vec<i32> = left_list.iter().zip(right_list.iter()).map(|(x, y)| (x - y).abs()).collect();
    let sum: i32 = distance.into_iter().sum();

    sum
}


pub fn main() {
    // read input
    let data  = fs::read_to_string("src/day_1_input.txt").expect("unable to read input file");
    let mut left_list: Vec<i32> = Vec::new();
    let mut right_list: Vec<i32> = Vec::new();

    // split into lists
    // run part 1
    for line in data.lines() {

        let parts: Vec<&str> = line.split_whitespace().collect();

        if let (Ok(left), Ok(right)) = (parts[0].parse::<i32>(), parts[1].parse::<i32>()) {
            left_list.push(left);
            right_list.push(right);
        }

    }


    let part1_sum = part1(&mut left_list, &mut right_list);

    println!("The distance between the lists is {part1_sum}")

}
