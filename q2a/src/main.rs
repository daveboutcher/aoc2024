use std::fs::read_to_string;

fn read_input() -> Vec<String> {
    read_to_string("input.txt")
        .unwrap() // panic on possible file-reading errors
        .lines() // split the string into an iterator of string slices
        .map(String::from) // make each slice into a string
        .collect() // gather them together into a vector
}

fn solve() -> i32 {
    read_input()
        .iter()
        .map(|l| l.split_whitespace().map(|s| s.parse::<i32>().unwrap()))
        .map(|v| {
            v.clone()
                .skip(1)
                .zip(v.clone().take(v.count() - 1))
                .map(|r| {
                    let (a, b) = r;
                    if (a - b).abs() < 1 || (a - b).abs() > 3 {
                        0
                    } else {
                        a - b
                    }
                })
                .collect::<Vec<i32>>()
        })
        .map(|r| {
            if r.iter().all(|v| *v < 0) || r.iter().all(|v| *v > 0) {
                1
            } else {
                0
            }
        })
        .sum()
}

fn main() {
    println!("Solution: {}", solve())
}
