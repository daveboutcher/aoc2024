use regex::Regex;
use std::fs::read_to_string;

fn read_input() -> Vec<String> {
    read_to_string("input.txt")
        .unwrap() // panic on possible file-reading errors
        .lines() // split the string into an iterator of string slices
        .map(String::from) // make each slice into a string
        .collect() // gather them together into a vector
}

fn solve() -> i64 {
    let re = Regex::new(r"p=(\d+),(\d+) v=(-?\d+),(-?\d+)").unwrap();

    let rows: i32 =103;
    let cols:i32 = 101;

    let mut totals = vec![0; 4];

    for l in read_input().iter() {
        let r = re.captures(l).unwrap();
        let (x, y, vx, vy) = (
            r[1].parse::<i32>().unwrap(),
            r[2].parse::<i32>().unwrap(),
            r[3].parse::<i32>().unwrap(),
            r[4].parse::<i32>().unwrap(),
        );

        let final_x = (x + vx * 100).rem_euclid(cols);
        let final_y = (y + vy * 100).rem_euclid(rows);

        if (final_y == rows / 2) || (final_x == cols / 2) {
            continue;
        }

        let mut idx: usize = 0;
        if final_x / (cols / 2) > 0 {
            idx += 2;
        }
        if final_y / (rows / 2) > 0 {
            idx += 1;
        }

        totals[idx] += 1;
    }

    totals[0] * totals[1] * totals[2] * totals[3]
}

fn main() {
    println!("Solution: {}", solve())
}
