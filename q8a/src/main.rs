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
                    let rowdiff = rownum - node.0;
                    let coldiff = colnum - node.1;

                    antinodes.insert((rownum + rowdiff, colnum + coldiff));
                    antinodes.insert((node.0 - rowdiff, node.1 - coldiff));
                }

                nodes.push((rownum, colnum));
            } else {
                nodemap.insert(c, vec![(rownum, colnum)]);
            }
        }
    }

    let an = antinodes
        .iter()
        .filter(|n| n.0 >= 0 && n.0 < maxrow && n.1 >= 0 && n.1 < maxcol)
        .map(|p| (p.0, p.1))
        .collect::<Vec<(i32, i32)>>();

    an.len() as i32
}

fn main() {
    println!("Solution: {}", solve())
}
