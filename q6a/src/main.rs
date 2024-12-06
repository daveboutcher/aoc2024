use std::fs::read_to_string;

enum Dir {
    N,
    S,
    E,
    W,
}

fn read_input() -> Vec<String> {
    read_to_string("input.txt")
        .unwrap() // panic on possible file-reading errors
        .lines() // split the string into an iterator of string slices
        .map(String::from) // make each slice into a string
        .collect() // gather them together into a vector
}

fn solve() -> i32 {
    let mut map = read_input()
        .iter()
        .map(|l| l.chars().collect::<Vec<char>>())
        .collect::<Vec<Vec<char>>>();

    let maxrow = map.len() - 1;
    let maxcol = map[0].len() - 1;

    let Some(mut row) = map.iter().position(|row| row.iter().any(|c| *c == '^')) else {
        todo!()
    };
    let Some(mut col) = map[row].iter().position(|c| *c == '^') else {
        todo!()
    };

    let mut direction = Dir::N;

    loop {
        map[row][col] = 'X';
        (direction, row, col) = match direction {
            Dir::N => {
                if row == 0 {
                    break;
                };
                if map[row - 1][col] == '#' {
                    (Dir::W, row, col)
                } else {
                    (direction, row - 1, col)
                }
            }
            Dir::W => {
                if col == maxcol {
                    break;
                };
                if map[row][col + 1] == '#' {
                    (Dir::S, row, col)
                } else {
                    (direction, row, col + 1)
                }
            }
            Dir::S => {
                if row == maxrow {
                    break;
                };
                if map[row + 1][col] == '#' {
                    (Dir::E, row, col)
                } else {
                    (direction, row + 1, col)
                }
            }
            Dir::E => {
                if col == 0 {
                    break;
                };
                if map[row][col - 1] == '#' {
                    (Dir::N, row, col)
                } else {
                    (direction, row, col - 1)
                }
            }
        }
    }

    map.iter().map(|row| row.iter().filter(|&c| *c == 'X').count()).sum::<usize>() as i32

}

fn main() {
    println!("Solution: {}", solve())
}
