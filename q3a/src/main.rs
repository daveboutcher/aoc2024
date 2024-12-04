use regex::Regex;
use std::fs::read_to_string;

fn read_input() -> Vec<String> {
    read_to_string("input.txt")
        .unwrap() // panic on possible file-reading errors
        .lines() // split the string into an iterator of string slices
        .map(String::from) // make each slice into a string
        .collect() // gather them together into a vector
}

fn solve() -> i32 {
    let re = Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)").unwrap();

    read_input()
        .iter()
        .map(|l| {
            re.captures_iter(l)
                .map(|cap| 
                    
                        cap[1].parse::<i32>().unwrap() *
                        cap[2].parse::<i32>().unwrap()
                )
                .sum::<i32>()
        })
        .sum()
}

fn main() {
    println!("Solution: {}", solve())
}
