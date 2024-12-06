use std::collections::HashSet;
use std::fs::read_to_string;

#[derive(PartialEq, Eq, Hash, Clone)]
enum Dir {
    N,
    S,
    E,
    W,
}

#[derive(PartialEq)]
enum Result {
    Loop,
    Exit,
}

fn read_input() -> Vec<String> {
    read_to_string("input.txt")
        .unwrap() // panic on possible file-reading errors
        .lines() // split the string into an iterator of string slices
        .map(String::from) // make each slice into a string
        .collect() // gather them together into a vector
}

fn walk(map: &[Vec<char>], mut row: usize, mut col: usize) -> Result {
    let mut path = HashSet::new();

    let maxrow = map.len() - 1;
    let maxcol = map[0].len() - 1;

    let mut direction = Dir::N;

    loop {
        if !path.insert((direction.clone(), row, col)) {
            return Result::Loop;
        }
        (direction, row, col) = match direction {
            Dir::N => {
                if row == 0 {
                    return Result::Exit;
                };
                if map[row - 1][col] == '#' {
                    (Dir::W, row, col)
                } else {
                    (direction, row - 1, col)
                }
            }
            Dir::W => {
                if col == maxcol {
                    return Result::Exit;
                };
                if map[row][col + 1] == '#' {
                    (Dir::S, row, col)
                } else {
                    (direction, row, col + 1)
                }
            }
            Dir::S => {
                if row == maxrow {
                    return Result::Exit;
                };
                if map[row + 1][col] == '#' {
                    (Dir::E, row, col)
                } else {
                    (direction, row + 1, col)
                }
            }
            Dir::E => {
                if col == 0 {
                    return Result::Exit;
                };
                if map[row][col - 1] == '#' {
                    (Dir::N, row, col)
                } else {
                    (direction, row, col - 1)
                }
            }
        }
    }
}

fn solve() -> i32 {
    let mut map = read_input()
        .iter()
        .map(|l| l.chars().collect::<Vec<char>>())
        .collect::<Vec<Vec<char>>>();

    let Some(row) = map.iter().position(|row| row.iter().any(|c| *c == '^')) else {
        todo!()
    };
    let Some(col) = map[row].iter().position(|c| *c == '^') else {
        todo!()
    };

    let mut count: i32 = 0;
    for currow in 0..map.len() {
        for curcol in 0..map[0].len() {
            if map[currow][curcol] != '.' {
                continue;
            }

            map[currow][curcol] = '#';
            if walk(&map, row, col) == Result::Loop {
                count += 1;
            }
            map[currow][curcol] = '.';
        }
    }

    count
}

fn main() {
    println!("Solution: {}", solve())
}
