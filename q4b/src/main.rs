use std::fs::read_to_string;

fn read_input() -> Vec<String> {
    read_to_string("input.txt")
        .unwrap() // panic on possible file-reading errors
        .lines() // split the string into an iterator of string slices
        .map(String::from) // make each slice into a string
        .collect() // gather them together into a vector
}

fn check_all_xmas(data: &[Vec<char>], x: &usize, y: &usize) -> i32 {
    if data[x + 1][y + 1] != 'A' {
        0
    } else if (data[*x][*y] == 'M' && data[x + 2][y + 2] == 'S')
        || (data[*x][*y] == 'S' && data[x + 2][y + 2] == 'M')
    {
        if (data[x + 2][*y] == 'M' && data[*x][y + 2] == 'S')
            || (data[x + 2][*y] == 'S' && data[*x][y + 2] == 'M')
        {
            1
        } else {
            0
        }
    } else {
        0
    }
}

fn solve() -> i32 {
    let data = read_input()
        .iter()
        .map(|l| l.chars().collect())
        .collect::<Vec<Vec<char>>>();

    let rows = data.len() - 1;
    let cols = data[0].len() - 1;

    let mut count: i32 = 0;
    for x in 0..=rows - 2 {
        for y in 0..=cols - 2 {
            if data[x][y] == 'M' || data[x][y] == 'S' {
                count += check_all_xmas(&data, &x, &y);
            }
        }
    }

    count
}

fn main() {
    println!("Solution: {}", solve())
}
