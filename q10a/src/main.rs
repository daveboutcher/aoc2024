use std::collections::HashSet;
use std::fs::read_to_string;

fn read_input() -> Vec<String> {
    read_to_string("input.txt")
        .unwrap() // panic on possible file-reading errors
        .lines() // split the string into an iterator of string slices
        .map(String::from) // make each slice into a string
        .collect() // gather them together into a vector
}
fn routes(
    map: &[Vec<char>],
    ends: &mut HashSet<(i64, i64)>,
    curchar: char,
    rownum: i64,
    colnum: i64,
) -> usize {
    let maxrow: i64 = (map.len() - 1) as i64;
    let maxcol: i64 = (map[0].len() - 1) as i64;

    if rownum < 0
        || rownum > maxrow
        || colnum < 0
        || colnum > maxcol
        || map[rownum as usize][colnum as usize] != curchar
    {
        0
    } else if curchar == '9' {
        if ends.insert((rownum, colnum)) {
            1
        } else {
            0
        }
    } else {
        let nextchar = (curchar as u8 + 1) as char;

        routes(map, ends, nextchar, rownum - 1, colnum)
            + routes(map, ends, nextchar, rownum + 1, colnum)
            + routes(map, ends, nextchar, rownum, colnum - 1)
            + routes(map, ends, nextchar, rownum, colnum + 1)
    }
}

fn solve() -> usize {
    let map = read_input()
        .iter()
        .map(|l| l.chars().collect::<Vec<char>>())
        .collect::<Vec<Vec<char>>>();

    map.iter()
        .enumerate()
        .map(|(rownum, row)| {
            row.iter()
                .enumerate()
                .map(|(colnum, _)| {
                    routes(
                        &map,
                        &mut HashSet::<(i64, i64)>::new(),
                        '0',
                        rownum as i64,
                        colnum as i64,
                    )
                })
                .sum::<usize>()
        })
        .sum::<usize>()
}

fn main() {
    println!("Solution: {}", solve())
}
