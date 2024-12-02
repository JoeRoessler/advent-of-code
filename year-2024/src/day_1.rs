
pub fn main() {
    //given data
    let mut left_list: [i32; 6] = [3, 4, 2, 1, 3, 3];
    let mut right_list: [i32; 6] = [4, 3, 5, 3, 9, 3];

    // sort the arrays
    left_list.sort();
    right_list.sort();

    // for each element compute the distance
    // this is abs element-wise subtraction
    let distance: Vec<i32> = left_list.iter().zip(right_list.iter()).map(|(x, y)| (x - y).abs()).collect();
    let sum: i32 = distance.into_iter().sum();

    println!("The distance between the lists is {sum}")

}
