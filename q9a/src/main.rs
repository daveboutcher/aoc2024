use std::fs::read_to_string;

fn read_input() -> Vec<String> {
    read_to_string("input.txt")
        .unwrap() // panic on possible file-reading errors
        .lines() // split the string into an iterator of string slices
        .map(String::from) // make each slice into a string
        .collect() // gather them together into a vector
}

fn solve() -> i64 {

    let mut blocks = read_input()[0].as_bytes().iter().map(|c| (c - b'0') as i32).collect::<Vec<i32>>();

    let mut checksum: i64 = 0;
    let mut curblock: i64 = 0;
    let mut lastidx = blocks.len() - 1;

    for idx in 0..blocks.len() {
        let mut count = blocks[idx] as i64;

        if idx & 0x01 == 0{
            let id = (idx / 2) as i64;

            checksum += id * (count + 1) * (curblock + curblock + count) / 2;
            curblock += count;
        } else {
            while count > 0 {
                while blocks[lastidx] > 0 && count > 0 {
                    let id = (lastidx / 2) as i64;

                    checksum += id * curblock;
                    count -= 1;
                    curblock += 1;
                    blocks[lastidx] -= 1;
                }

                if blocks[lastidx] == 0 {
                    lastidx -= 1;
                    blocks[lastidx] = 0;
                    lastidx -= 1;
                }
            }
        }
    }

    checksum

}

fn main() {
    println!("Solution: {}", solve())
}
