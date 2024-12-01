use itertools::Itertools;
use std::fs::read_to_string;

fn read_input() -> Vec<String> {
    read_to_string("input.txt")
        .unwrap() // panic on possible file-reading errors
        .lines() // split the string into an iterator of string slices
        .map(String::from) // make each slice into a string
        .collect() // gather them together into a vector
}

fn solve() -> i32 {
    let (mut a, mut b) = read_input()
        .iter()
        .map(|l| {
            l.split_whitespace()
                .map(|s| s.parse::<i32>().unwrap())
                .collect_tuple::<(i32, i32)>()
                .unwrap()
        })
        .fold((Vec::new(), Vec::new()), |mut acc, ns| {
            acc.0.push(ns.0);
            acc.1.push(ns.1);
            acc
        });

    a.sort();
    b.sort();

    a.iter()
        .zip(b.iter())
        .fold(0, |acc, ns| acc + (ns.0 - ns.1).abs())
}

fn main() {
    println!("Solution: {}", solve())
}
