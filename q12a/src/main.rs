use std::collections::HashSet;
use std::fs::read_to_string;

fn read_input() -> Vec<String> {
    read_to_string("input.txt")
        .unwrap() // panic on possible file-reading errors
        .lines() // split the string into an iterator of string slices
        .map(String::from) // make each slice into a string
        .collect() // gather them together into a vector
}

fn visit(
    map: &[Vec<char>],
    visited: &mut HashSet<(usize, usize)>,
    plant: char,
    row: usize,
    col: usize,
) -> (usize, usize) {
    let mut perimeter: usize = 0;
    let mut area: usize = 0;

    if visited.contains(&(row, col)) || map[row][col] != plant {
        return (0, 0);
    }

    visited.insert((row, col));

    area += 1;

    if row == 0 || map[row - 1][col] != plant {
        perimeter += 1;
    }

    if row == map.len() -1 || map[row + 1][col] != plant {
        perimeter += 1;
    }

    if col == 0 || map[row][col-1] != plant {
        perimeter += 1;
    }

    if col == map[0].len() -1 || map[row][col+1] != plant {
        perimeter += 1;
    }

    if row != 0 {
        let (p, a) = visit(map, visited, plant, row - 1, col);
        perimeter += p;
        area += a;
    }

    if row != map.len() - 1 {
        let (p, a) = visit(map, visited, plant, row + 1, col);
        perimeter += p;
        area += a;
    }

    if col != 0 {
        let (p, a) = visit(map, visited, plant, row, col - 1);
        perimeter += p;
        area += a;
    }

    if col != map[0].len() - 1 {
        let (p, a) = visit(map, visited, plant, row, col + 1);
        perimeter += p;
        area += a;
    }
    (perimeter, area)
}

fn solve() -> usize {
    let map = read_input()
        .iter()
        .map(|l| l.chars().collect::<Vec<char>>())
        .collect::<Vec<Vec<char>>>();

    let mut visited = HashSet::<(usize, usize)>::new();

    let mut total: usize = 0;

    for row in 0..map.len() {
        for col in 0..map[0].len() {
            if visited.contains(&(row, col)) {
                continue;
            }

            let (perimeter, area) = visit(&map, &mut visited, map[row][col], row, col);

            total += perimeter * area;
        }
    }

    total
}

fn main() {
    println!("Solution: {}", solve())
}
