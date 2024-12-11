use std::fs::read_to_string;
use memoize::memoize;


fn read_input() -> Vec<String> {
    read_to_string("input.txt")
        .unwrap() // panic on possible file-reading errors
        .lines() // split the string into an iterator of string slices
        .map(String::from) // make each slice into a string
        .collect() // gather them together into a vector
}

const ITERATIONS:usize = 75;

#[memoize]
fn solve_one(count: usize, cur: u64) -> usize {
    if count == 0 {
        return 1;
    } 

    if cur == 0 {
        return solve_one(count-1, 1);
    }

    let s = cur.to_string();
    if s.len() % 2 == 0 {
        return solve_one(count - 1, s[0..(s.len()/2)].parse::<u64>().unwrap()) +
        solve_one(count - 1, s[(s.len()/2)..].parse::<u64>().unwrap())
    }

    solve_one(count - 1, cur * 2024)
}

fn solve() -> usize {
    let nums = read_input()[0].split(' ').map(|s| s.parse::<u64>().unwrap()).collect::<Vec<u64>>();

    nums.iter().map(|n| solve_one(ITERATIONS, *n)).sum()
}

fn main() {
    println!("Solution: {}", solve())
}
