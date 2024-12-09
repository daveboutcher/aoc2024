use std::fs::read_to_string;

struct Block {
    avail: i64,
    used: i64,
}

fn read_input() -> Vec<String> {
    read_to_string("input.txt")
        .unwrap() // panic on possible file-reading errors
        .lines() // split the string into an iterator of string slices
        .map(String::from) // make each slice into a string
        .collect() // gather them together into a vector
}

fn solve() -> i64 {
    let mut blocks = read_input()[0]
        .as_bytes()
        .iter()
        .enumerate()
        .map(|(pos, c)| {
            if pos & 0x01 == 0 {
                Block {
                    used: (*c - b'0') as i64,
                    avail: 0,
                }
            } else {
                Block {
                    used: 0,
                    avail: (*c - b'0') as i64,
                }
            }
        })
        .collect::<Vec<Block>>();

    let mut checksum: i64 = 0;

    for idx in (0..blocks.len()).rev() {
        if idx & 0x01 != 0 {
            continue;
        }

        if blocks[idx].used == 0 {
            continue;
        }

        let mut curblock: i64 = 0;

        for idx2 in 0..idx {
            if idx2 & 0x01 == 0 {
                curblock += blocks[idx2].used;
            } else {
                let id = idx / 2;
                if (blocks[idx2].avail - blocks[idx2].used) >= blocks[idx].used {
                    curblock += blocks[idx2].used;

                    while blocks[idx].used > 0 {
                        checksum += id as i64 * curblock;

                        curblock += 1;
                        blocks[idx2].used += 1;
                        blocks[idx].used -= 1;
                        blocks[idx].avail += 1;
                    }

                    break;
                } else {
                    curblock += blocks[idx2].avail;
                }
            }
        }
    }

    let mut curblock: i64 = 0;
    for idx in 0..blocks.len() {
        if idx & 0x01 != 0 {
            curblock += blocks[idx].avail;
            continue;
        }

        let id = idx / 2;


        if blocks[idx].avail > 0 {
            curblock += blocks[idx].avail
        } else {
            while blocks[idx].used > 0 {
                checksum += id as i64 * curblock;

                curblock += 1;
                blocks[idx].used -= 1;
            }
        }
    }

    checksum
}

fn main() {
    println!("Solution: {}", solve())
}
