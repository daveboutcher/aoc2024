use std::fs::read_to_string;

fn read_input() -> Vec<String> {
    read_to_string("input.txt")
        .unwrap() // panic on possible file-reading errors
        .lines() // split the string into an iterator of string slices
        .map(String::from) // make each slice into a string
        .collect() // gather them together into a vector
}

fn is_good(array: &[Vec<u8>; 100], vals: &[u8]) -> Option<usize> {
    let mut previous = Vec::<u8>::new();
    for (idx, d) in vals.iter().enumerate() {
        for p in previous.iter() {
            if array[*d as usize].contains(p) {
                return Some(idx);
            }
        }

        previous.push(*d);
    }

    Option::None
}

fn solve() -> i32 {
    let lines = read_input();

    let mut iter = lines.iter();

    let mut array: [Vec<u8>; 100] = [const { Vec::new() }; 100];

    loop {
        let l = iter.next().unwrap();
        if l.is_empty() {
            break;
        }

        let a = l[0..2].parse::<usize>().unwrap();
        let b = l[3..5].parse::<u8>().unwrap();
        array[a].push(b);
    }

    let mut total: i32 = 0;
    for l in iter {
        let mut vals = l
            .split(',')
            .map(|s| s.parse::<u8>().unwrap())
            .collect::<Vec<u8>>();

        let mut idx = match is_good(&array, &vals) {
            None => continue,
            Some(i) => i,
        };

        loop {
            vals.swap(idx - 1, idx);

            idx = match is_good(&array, &vals) {
                None => {
                    total += vals[vals.len() / 2] as i32;
                    break;
                }
                Some(i) => i,
            };
        }
    }

    total
}

fn main() {
    println!("Solution: {}", solve())
}
