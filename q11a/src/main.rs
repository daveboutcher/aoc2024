use std::fs::read_to_string;

fn read_input() -> Vec<String> {
    read_to_string("input.txt")
        .unwrap() // panic on possible file-reading errors
        .lines() // split the string into an iterator of string slices
        .map(String::from) // make each slice into a string
        .collect() // gather them together into a vector
}

fn solve() -> usize {
    let mut nums = read_input()[0].split(' ').map(|s| s.parse::<u64>().unwrap()).collect::<Vec<u64>>();

    for _ in 0..25 {
        let mut newnums = Vec::<u64>::new();

        for n in nums {
            if n == 0 {
                newnums.push(1);
            } else {
                let s = n.to_string();
                if s.len() % 2 == 0 {
                    newnums.push(s[0..(s.len()/2)].parse::<u64>().unwrap());
                    newnums.push(s[(s.len()/2)..].parse::<u64>().unwrap());
                } else {
                    newnums.push(n * 2024);
                }
            }
        }
        nums = newnums;
    }

    nums.len()
}

fn main() {
    println!("Solution: {}", solve())
}
