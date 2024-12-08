use std::fs::read_to_string;

fn read_input() -> Vec<String> {
    read_to_string("input.txt")
        .unwrap() // panic on possible file-reading errors
        .lines() // split the string into an iterator of string slices
        .map(String::from) // make each slice into a string
        .collect() // gather them together into a vector
}

fn calc(testval: &u64, current: u64, vals: &[u64]) -> bool {
    if vals.is_empty() {
        *testval == current
    } else if current > *testval {
        false
    } else {
        calc(testval, current + vals[0], &vals[1..]) ||
        calc(testval, current * vals[0], &vals[1..])
    }
}

fn solve() -> u64 {
    read_input().into_iter().map(|l| {
        let mut iter = l.split(": ");
        let testval = iter.next().unwrap().parse::<u64>().unwrap();

        let vals = iter
            .next()
            .unwrap()
            .split(' ')
            .map(|s| s.parse::<u64>().unwrap())
            .collect::<Vec<u64>>();

        if calc(&testval, vals[0], &vals[1..]) {
            testval
        } else {
            0u64
        }
    }).sum()
}

fn main() {
    println!("Solution: {}", solve())
}
