use std::{collections::HashSet, thread::current, thread, time};
use std::fs;

#[derive(PartialEq, Debug)]
enum State {
    Walk((isize, isize)),
    Turn,
    Exit
}

#[derive(Debug, Clone)]
struct Guard {
    position: (isize, isize),
    direction: (isize, isize),
    visted: HashSet<(isize, isize)>,
    turns: usize
}

impl Guard {
    const DIRS: [(isize, isize); 4] = [(-1, 0), (0, 1), (1, 0), (0, -1)];

    fn step(&self, grid: &Vec<Vec<char>> ) -> State {
        let row_upper_bound: isize = (grid.len() - 1).try_into().unwrap();
        let col_upper_bound: isize = (grid[0].len() - 1).try_into().unwrap();

        let next_coord = (self.position.0 + self.direction.0, self.position.1 + self.direction.1);
        // check that next coord is in bounds
        if next_coord.0 > row_upper_bound || next_coord.0 < 0 || next_coord.1 > col_upper_bound || next_coord.1 < 0 {
            State::Exit
        } else if grid[next_coord.0 as usize][next_coord.1 as usize] == '#' {
            State::Turn
        } else {
            State::Walk(next_coord)
        }
    }

    fn patrol(mut self, grid: &Vec<Vec<char>>) -> usize {
        // add the starting location
        self.visted.insert(self.position);

        // take a step
        let mut current_state = self.step(grid);
        // println!("{:?}", current_state);
        // let second = time::Duration::from_secs(1);
        // thread::sleep(second);

        while current_state != State::Exit {
            // println!("{:?}", current_state);
            match current_state {
                State::Walk(pos) => {
                    // println!("WALKING to {:?}", pos);
                    self.visted.insert(pos);
                    self.position = pos;
                    current_state = self.step(grid)
                },
                State::Turn => {
                    self.turns += 1;
                    let dir_idx = self.turns % 4;
                    self.direction = Guard::DIRS[dir_idx];
                    // println!("DIRECTION {:?}", self.direction);
                    current_state = self.step(grid)
                 }
                State::Exit => { continue; }
            }
            // thread::sleep(second);
        }
        self.visted.len()
    }


}

pub fn main() {
    // read in data in grid
    let input = fs::read_to_string("src/day_6_input.txt").expect("Could not read file");

    // mutable vec of vec
    let mut grid: Vec<Vec<char>> = Vec::new();

    // make each char a cell in the matrix
    for line in input.lines() {
        let row: Vec<char> = line.chars().collect();
        grid.push(row);
    }

    // PART 1
    //find starting point
    let mut starting_point: (isize, isize) = (0, 0);
    for row in 0..grid.len() {
        for col in 0..grid[0].len() {
            // println!("AT {:?}: {}", (row, col), grid[row][col]);
            if let '^' = grid[row][col] {
                println!("START FOUND {:?}", (row, col));
                 starting_point = (row as isize, col as isize)
            }
        }
    }
    println!("START: {:?}", starting_point);
    let mut guard = Guard{ position: starting_point, direction: (-1, 0), visted: HashSet::new(), turns: 0 };


    let spaces_visited = guard.patrol(&grid);

    println!("SPACES VISITED: {}", spaces_visited);

}
