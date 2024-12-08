use gcd::Gcd;
use std::collections::HashMap;
use std::collections::HashSet;
use std::fs::read_to_string;

fn read_input() -> Vec<String> {
    read_to_string("input.txt")
        .unwrap() // panic on possible file-reading errors
        .lines() // split the string into an iterator of string slices
        .map(String::from) // make each slice into a string
        .collect() // gather them together into a vector
}

fn generate_line(
    x: i32,
    y: i32,
    xdiff: i32,
    ydiff: i32,
    max_x: i32,
    max_y: i32,
) -> Vec<(i32, i32)> {
    let mut multiplier = 0;
    let mut cur_x = x + (xdiff * multiplier);
    let mut cur_y = y + (ydiff * multiplier);

    let mut result = Vec::<(i32, i32)>::new();

    while cur_x >= 0 && cur_y >= 0 && cur_x <= max_x && cur_y <= max_y {
        result.push((cur_x, cur_y));

        multiplier += 1;
        cur_x = x + (xdiff * multiplier);
        cur_y = y + (ydiff * multiplier);
    }

    multiplier = -1;
    cur_x = x + (xdiff * multiplier);
    cur_y = y + (ydiff * multiplier);

    while cur_x >= 0 && cur_y >= 0 && cur_x <= max_x && cur_y <= max_y {
        result.push((cur_x, cur_y));

        multiplier -= 1;
        cur_x = x + (xdiff * multiplier);
        cur_y = y + (ydiff * multiplier);
    }

    result
}

fn solve() -> i32 {
    let lines = read_input();

    let maxrow = lines.len() as i32;
    let maxcol = lines[0].len() as i32;

    let mut nodemap = HashMap::<char, Vec<(i32, i32)>>::new();
    let mut antinodes = HashSet::new();

    for (rownuml, row) in lines.iter().enumerate() {
        for (colnuml, c) in row.chars().enumerate() {
            let rownum = rownuml as i32;
            let colnum = colnuml as i32;

            if c == '.' {
                continue;
            }

            if let Some(nodes) = nodemap.get_mut(&c) {
                for node in nodes.iter() {
                    let mut rowdiff = rownum - node.0;
                    let mut coldiff = colnum - node.1;

                    let gcd = (rowdiff.unsigned_abs()).gcd(coldiff.unsigned_abs()) as i32;

                    rowdiff /= gcd;
                    coldiff /= gcd;

                    for p in generate_line(
                        rownum,
                        colnum,
                        rowdiff,
                        coldiff,
                        (maxrow - 1) as i32,
                        (maxcol - 1) as i32,
                    ) {
                            antinodes.insert(p);
                    }
                }

                nodes.push((rownum, colnum));
            } else {
                nodemap.insert(c, vec![(rownum, colnum)]);
            }
        }
    }

    antinodes.len() as i32
}

fn main() {
    println!("Solution: {}", solve())
}
