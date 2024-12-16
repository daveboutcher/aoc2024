use std::collections::HashMap;
use std::fs::read_to_string;

fn read_input() -> Vec<String> {
    read_to_string("input.txt")
        .unwrap() // panic on possible file-reading errors
        .lines() // split the string into an iterator of string slices
        .map(String::from) // make each slice into a string
        .collect() // gather them together into a vector
}

fn nextpos(dir: char, posx: usize, posy: usize) -> (usize, usize) {
    match dir {
        '<' => (posx - 1, posy),
        '>' => (posx + 1, posy),
        '^' => (posx, posy - 1),
        'v' => (posx, posy + 1),
        _ => todo!(),
    }
}

fn do_move(map: &mut Vec<Vec<char>>, dir: char, posx: usize, posy: usize, width: usize) -> bool {
    let next = nextpos(dir, posx, posy);
    if width == 1 {
        match map[next.1][next.0] {
            '#' => false,
            '.' => {
                map[next.1][next.0] = map[posy][posx];
                true
            }
            '[' => {
                if do_move(map, dir, next.0, next.1, 2) {
                    map[next.1][next.0] = map[posy][posx];
                    true
                } else {
                    false
                }
            }
            ']' => {
                if do_move(map, dir, next.0 - 1, next.1, 2) {
                    map[next.1][next.0] = map[posy][posx];
                    true
                } else {
                    false
                }
            }
            '@' => {
                println!("Unexpected @");
                false
            }
            c => {
                println!("UNEXPECTED CHAR {}", c);
                false
            }
        }
    } else {
        match dir {
            '<' => match map[next.1][next.0] {
                '#' => false,
                '.' => {
                    map[next.1][next.0] = map[posy][posx];
                    map[next.1][next.0 + 1] = map[posy][posx + 1];
                    true
                }
                ']' => {
                    if do_move(map, dir, next.0 - 1, next.1, 2) {
                        map[next.1][next.0] = map[posy][posx];
                        map[next.1][next.0 + 1] = map[posy][posx + 1];
                        true
                    } else {
                        false
                    }
                }
                '@' => {
                    println!("Unexpected @");
                    false
                }
                c => {
                    println!("UNEXPECTED CHAR {}", c);
                    false
                }
            },
            '>' => match map[next.1][next.0 + 1] {
                '#' => false,
                '.' => {
                    map[next.1][next.0 + 1] = map[posy][posx + 1];
                    map[next.1][next.0] = map[posy][posx];
                    true
                }
                '[' => {
                    if do_move(map, dir, next.0 + 1, next.1, 2) {
                        map[next.1][next.0 + 1] = map[posy][posx + 1];
                        map[next.1][next.0] = map[posy][posx];
                        true
                    } else {
                        false
                    }
                }
                '@' => {
                    println!("Unexpected @");
                    false
                }
                c => {
                    println!("UNEXPECTED CHAR {}", c);
                    false
                }
            },
            '^' | 'v' => {
                if map[next.1][next.0] == '#' || map[next.1][next.0 + 1] == '#' {
                    false
                } else if map[next.1][next.0] == '.' && map[next.1][next.0 + 1] == '.' {
                    map[next.1][next.0] = map[posy][posx];
                    map[next.1][next.0 + 1] = map[posy][posx + 1];
                    map[posy][posx] = '.';
                    map[posy][posx + 1] = '.';
                    true
                } else if map[next.1][next.0] == '[' {
                    if do_move(map, dir, next.0, next.1, 2) {
                        map[next.1][next.0 + 1] = map[posy][posx + 1];
                        map[next.1][next.0] = map[posy][posx];
                        map[posy][posx] = '.';
                        map[posy][posx + 1] = '.';
                        true
                    } else {
                        false
                    }
                } else if map[next.1][next.0 - 1] == '[' && map[next.1][next.0 + 1] == '.' {
                    if do_move(map, dir, next.0 - 1, next.1, 2) {
                        map[next.1][next.0] = map[posy][posx];
                        map[next.1][next.0 + 1] = map[posy][posx + 1];
                        map[posy][posx] = '.';
                        map[posy][posx + 1] = '.';
                        true
                    } else {
                        false
                    }
                } else if map[next.1][next.0 + 1] == '[' && map[next.1][next.0] == '.' {
                    if do_move(map, dir, next.0 + 1, next.1, 2) {
                        map[next.1][next.0] = map[posy][posx];
                        map[next.1][next.0 + 1] = map[posy][posx + 1];
                        map[posy][posx] = '.';
                        map[posy][posx + 1] = '.';
                        true
                    } else {
                        false
                    }
                } else {
                    assert!(map[next.1][next.0 - 1] == '[' && map[next.1][next.0 + 1] == '[');
                    let savedmap = map.clone();
                    if do_move(map, dir, next.0 - 1, next.1, 2)
                        && do_move(map, dir, next.0 + 1, next.1, 2)
                    {
                        map[next.1][next.0] = map[posy][posx];
                        map[next.1][next.0 + 1] = map[posy][posx + 1];
                        map[posy][posx] = '.';
                        map[posy][posx + 1] = '.';
                        true
                    } else {
                        *map = savedmap;
                        false
                    }
                }
            }
            _ => todo!(),
        }
    }
}

fn solve() -> usize {
    let mut i = read_input().into_iter().peekable();

    let mut map = Vec::<Vec<char>>::new();

    let mut r = 0;

    let mut posx = 0usize;
    let mut posy = 0usize;
    while !i.peek().unwrap().is_empty() {
        let mut row = Vec::<char>::new();
        for c in i.next().unwrap().chars() {
            match c {
                '#' => {
                    row.push('#');
                    row.push('#');
                }
                'O' => {
                    row.push('[');
                    row.push(']');
                }
                '@' => {
                    row.push('@');
                    row.push('.');
                }
                '.' => {
                    row.push('.');
                    row.push('.');
                }
                _ => todo!(),
            }
        }
        map.push(row);

        if let Some(col) = map[map.len() - 1].iter().position(|c| *c == '@') {
            posx = col;
            posy = r;
        }

        r += 1;
    }

    i.next();

    for l in i {
        for c in l.chars() {
            if do_move(&mut map, c, posx, posy, 1) {
                map[posy][posx] = '.';
                (posx, posy) = nextpos(c, posx, posy);
            }
        }
    }

    map.iter()
        .enumerate()
        .map(|(row, r)| {
            r.iter()
                .enumerate()
                .map(|(col, c)| if *c == '[' { 100 * row + col } else { 0 })
                .sum::<usize>()
        })
        .sum::<usize>()
}

fn main() {
    println!("Solution: {}", solve())
}
