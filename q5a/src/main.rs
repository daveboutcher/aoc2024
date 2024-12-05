use std::fs::read_to_string;

fn read_input() -> Vec<String> {
    read_to_string("input.txt")
        .unwrap() // panic on possible file-reading errors
        .lines() // split the string into an iterator of string slices
        .map(String::from) // make each slice into a string
        .collect() // gather them together into a vector
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
        let mut previous = Vec::<u8>::new();
        let mut good = true;
        'outer: for ds in l.split(',') {
            let d = ds.parse::<u8>().unwrap();
            for p in previous.iter() {
                if array[d as usize].contains(p) {
                    good = false;
                    break 'outer;
                }
            }

            previous.push(d);
        }

        if good {
            total += previous[previous.len() / 2] as i32;
        }
    }

    total
}

fn main() {
    println!("Solution: {}", solve())
}
