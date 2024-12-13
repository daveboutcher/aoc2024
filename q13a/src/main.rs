use regex::Regex;
use std::fs::read_to_string;

fn read_input() -> Vec<String> {
    read_to_string("input.txt")
        .unwrap() // panic on possible file-reading errors
        .lines() // split the string into an iterator of string slices
        .map(String::from) // make each slice into a string
        .collect() // gather them together into a vector
}

fn eq(ax: i64, ay: i64, bx: i64, by: i64, px: i64, py: i64) -> Option<(i64, i64)> {
    // Calculate the determinant of the coefficient matrix
    let det = ax * by - bx * ay;

    // If determinant is zero, the system has no unique solution
    if det == 0 {
        println!("Unsolvable!");
        return None;
    }

    // Calculate x using Cramer's rule
    let det_x = px * by - bx* py;
    let x = det_x / det;

    // Calculate y using Cramer's rule
    let det_y = ax * py - px * ay;
    let y = det_y / det;


    if ax * x + bx * y == px &&
       ay * x + by * y == py {

        Some((x, y))
       } else {
        None
       }
}


fn solve() -> i64 {
    let re_a = Regex::new(r"Button .: X\+(\d+), Y\+(\d+)").unwrap();
    let re_p = Regex::new(r"Prize: X=(\d+), Y=(\d+)").unwrap();

    let lines = read_input();

    let mut ax: i64 = 0;
    let mut ay: i64 = 0;
    let mut bx: i64 = 0;
    let mut by: i64 = 0;
    let mut px: i64 = 0;
    let mut py: i64 = 0;

    let mut total: i64 = 0;

    for l in lines.iter() {
        if l.starts_with("Button A:") {
            let r = re_a.captures(l).unwrap();
            ax = r[1].parse::<i64>().unwrap();
            ay = r[2].parse::<i64>().unwrap();
        } else if l.starts_with("Button B:") {
            let r = re_a.captures(l).unwrap();
            bx = r[1].parse::<i64>().unwrap();
            by = r[2].parse::<i64>().unwrap();
        } else if l.starts_with("Prize:") {
            let r = re_p.captures(l).unwrap();
            px = r[1].parse::<i64>().unwrap();
            py = r[2].parse::<i64>().unwrap();

            if let Some((x, y)) = eq(ax, ay, bx, by, px, py) {
                println!("Solution: {} {}", x, y);
                total += (x * 3 + y);
            } else {
                println!("No solution");
            }

        }
    }

    total
}

fn main() {
    println!("Solution: {}", solve())
}
