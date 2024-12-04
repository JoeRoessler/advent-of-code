use std::fs;


pub fn main() {
    // read input
    let input = fs::read_to_string("src/day_4_input.txt").expect("Could not read file");

    // mutable vec of vec
    let mut grid: Vec<Vec<char>> = Vec::new();

    // make each char a cell in the matrix
    for line in input.lines() {
        let row: Vec<char> = line.chars().collect();
        grid.push(row);
    }


    // PART 1 ---------------------------------------------------------
    let mut xmas_count = 0;
    // for each cell
    // search the 8 different directions for XMAS
    println!("{}, {}", grid.len(), grid[0].len());
    for row in 0..grid.len() {
        for col in 0..grid[0].len() {
            if grid[row][col] == 'X' {
                // create bounded search ranges
                let row = row as isize;
                let col = col as isize;

                let i_start = if row > 2 { -1 } else { 0 };
                let i_end = if row < grid.len() as isize - 3 { 2 } else { 1 };
                let j_start = if col > 2 { -1 } else { 0 };
                let j_end = if col < grid[0].len() as isize - 3 { 2 } else { 1 };


                for i in i_start..i_end {
                    for j in j_start..j_end {
                        if grid[(row+ i) as usize][(col + j) as usize] == 'M'
                        && grid[(row + (2*i)) as usize][(col + (2*j)) as usize] == 'A'
                        && grid[(row + (3*i)) as usize][(col + (3*j)) as usize] == 'S' {
                            xmas_count += 1
                        } else {
                            continue;
                        }
                    }
                }
            }
        }
    }

    println!("Total XMAS: {}", xmas_count);

    // PART 2 --------------------------------------------------------
    // just need to check the diagonals of each 'A'
    // those diagonals should contain 'M' and 'S'
    // push the diags into a vec
    // define my vec to validate against: b = vec!['M', 'S']
    // then b.iter().all(|x| diags.contains(x))

    let mut diag_count = 0;
    let validation_set = vec!['M', 'S'];

    for row in 0..grid.len() {
        for col in 0..grid[0].len() {
            // need to be interior
            if row > 0 && row < grid.len() - 1 && col > 0 && col < grid.len() - 1 {
                if grid[row][col] == 'A' {
                    // collect the diags
                    let diag1 = vec![grid[row - 1][col - 1], grid[row + 1][col + 1]];
                    let diag2 = vec![grid[row - 1][col + 1], grid[row + 1][col - 1]];

                    // check contains
                    if validation_set.iter().all(|x| diag1.contains(x)) && validation_set.iter().all(|x| diag2.contains(x)) {
                        diag_count += 1
                    } else {
                        continue;
                    }



                } else {
                    continue;
                }
            } else {
                continue;
            }

        }
    }

    println!("TOTAL CROSS {}", diag_count);

}
