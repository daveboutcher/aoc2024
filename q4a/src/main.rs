use std::fs::read_to_string;

fn read_input() -> Vec<String> {
    read_to_string("input.txt")
        .unwrap() // panic on possible file-reading errors
        .lines() // split the string into an iterator of string slices
        .map(String::from) // make each slice into a string
        .collect() // gather them together into a vector
}

fn check_xmas(
    xoffset: isize,
    yoffset: isize,
    data: &[Vec<char>],
    x: &isize,
    y: &isize,
    rows: &isize,
    cols: &isize,
) -> i32 {
    if xoffset == 0 && yoffset == 0 {
        0
    } else if (x + (xoffset * 3) < 0)
        || (x + (xoffset * 3) > *cols)
        || (y + (yoffset * 3) < 0)
        || (y + (yoffset * 3) > *rows)
    {
        0
    } else if data[(x + xoffset) as usize][(y + yoffset) as usize] == 'M'
        && data[(x + xoffset * 2) as usize][(y + yoffset * 2) as usize] == 'A'
        && data[(x + xoffset * 3) as usize][(y + yoffset * 3) as usize] == 'S'
    {
        1
    } else {
        0
    }
}

fn check_all_xmas(data: &[Vec<char>], x: &isize, y: &isize, rows: isize, cols: isize) -> i32 {
    let mut count: i32 = 0;
    for xoffset in -1..=1 {
        for yoffset in -1..=1 {
            count += check_xmas(xoffset, yoffset, data, x, y, &rows, &cols)
        }
    }
    count
}

fn solve() -> i32 {
    let data = read_input()
        .iter()
        .map(|l| l.chars().collect())
        .collect::<Vec<Vec<char>>>();

    let rows: isize = (data.len()) as isize;
    let cols: isize = (data[0].len()) as isize;

    let mut count: i32 = 0;
    for x in 0..rows {
        for y in 0..cols {
            if data[x as usize][y as usize] == 'X' {
                count += check_all_xmas(&data, &x, &y, rows - 1, cols - 1);
            }
        }
    }

    count
}

fn main() {
    println!("Solution: {}", solve())
}
