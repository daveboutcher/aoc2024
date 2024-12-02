use std::fs::read_to_string;

fn read_input() -> Vec<String> {
    read_to_string("input.txt")
        .unwrap() // panic on possible file-reading errors
        .lines() // split the string into an iterator of string slices
        .map(String::from) // make each slice into a string
        .collect() // gather them together into a vector
}

fn is_safe_sub<'a>(vals: impl Iterator<Item = &'a i32> + Clone) -> bool {
    let result = vals
    .clone()
    .skip(1)
    .zip(vals.clone().take(vals.count() - 1))
    .map(|r| {
        let (a, b) = r;
        if (a - b).abs() < 1 || (a - b).abs() > 3 {
            0
        } else {
            a - b
        }
    })
    .collect::<Vec<i32>>();

    if result.iter().all(|v| *v < 0) || result.iter().all(|v| *v > 0) {
        true
    } else {
        false
    }
}

fn is_safe(vals: &Vec<i32>) -> bool {
    for i in 0..vals.len() {
        if is_safe_sub(vals.iter().enumerate().filter(|&(j, _)| j != i).map(|(_, v)| v)) {
            return true;
        }
    }

    false
}

fn solve() -> i32 {
    read_input()
        .iter()
        .map(|l| {
            l.split_whitespace()
                .map(|s| s.parse::<i32>().unwrap())
                .collect::<Vec<i32>>()
        })
        .filter(|v| is_safe(v))
        .count() as i32
}

fn main() {
    println!("Solution: {}", solve())
}
