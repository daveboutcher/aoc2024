use std::fs::read_to_string;

fn read_input() -> Vec<String> {
    read_to_string("input.txt")
        .unwrap() // panic on possible file-reading errors
        .lines() // split the string into an iterator of string slices
        .map(String::from) // make each slice into a string
        .collect() // gather them together into a vector
}

fn printmap(map: &[Vec<char>]) {
    for r in map.iter() {
        for c in r {
            print!("{}", c);
        }
        println!();
    }
}

fn nextpos(dir: char, posx: usize, posy: usize) -> (usize, usize) {
    match dir {
        '<' => (posx - 1, posy),
        '>' => (posx + 1, posy),
        '^' => (posx, posy - 1),
        'v' => (posx, posy + 1),
        _ => todo!()
    }

}

fn do_move(map: &mut [Vec<char>], dir: char, posx: usize, posy: usize) -> bool {
    let next = nextpos(dir, posx, posy);

    match map[next.1][next.0] {
        '#' => false,
        '.' => {map[next.1][next.0] = map[posy][posx]; true},
        'O' => if do_move(map, dir, next.0, next.1) {
            map[next.1][next.0] = map[posy][posx];
            true
        } else {
            false
        },
        '@' => {
            println!("Unexpected @");
            false
        },
        c => {println!("UNEXPECTED CHAR {}", c); false}

    }
}

fn solve() -> usize {
    let mut i = read_input().into_iter().peekable();

    let mut map = Vec::<Vec<char>>::new();

    let mut r = 0;

    let mut posx = 0usize;
    let mut posy = 0usize;
    while !i.peek().unwrap().is_empty() {
        map.push(i.next().unwrap().chars().collect::<Vec<char>>());

        if let Some(col) = map[map.len() - 1].iter().position(|c| *c == '@') {
            posx = col;
            posy = r;
        }

        r += 1;
    }

    printmap(&map);

    i.next();

    for l in i {
        for c in l.chars() {
            if do_move(&mut map, c, posx, posy) {
                map[posy][posx] = '.';
                (posx, posy) = nextpos(c, posx, posy);
            }
        }
    }

    map.iter().enumerate().map(|(row, r)| r.iter().enumerate().map(|(col, c)| if *c == 'O' { 100 * row + col} else {0}).sum::<usize>()).sum::<usize>()
}

fn main() {
    println!("Solution: {}", solve())
}
