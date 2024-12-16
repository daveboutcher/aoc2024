use regex::Regex;
use std::fs::read_to_string;

fn read_input() -> Vec<String> {
    read_to_string("input.txt")
        .unwrap() // panic on possible file-reading errors
        .lines() // split the string into an iterator of string slices
        .map(String::from) // make each slice into a string
        .collect() // gather them together into a vector
}

struct Robot {
    x: i32,
    y: i32,
    vx: i32,
    vy: i32,
}

fn solve() -> i64 {
    let re = Regex::new(r"p=(\d+),(\d+) v=(-?\d+),(-?\d+)").unwrap();

    let rows: i32 = 103;
    let cols: i32 = 101;

    let robots = read_input().iter()
        .map(|l| {
            let r = re.captures(l).unwrap();
            Robot {
                x: r[1].parse::<i32>().unwrap(),
                y: r[2].parse::<i32>().unwrap(),
                vx: r[3].parse::<i32>().unwrap(),
                vy: r[4].parse::<i32>().unwrap(),
            }
        })
        .collect::<Vec<Robot>>();

    let mut total_distance: f64 = 0.0;
    let mut min_distance: f64 = f64::INFINITY;

    for i in 0..(103 * 101) {
        let mut points = Vec::<(i32, i32)>::new();

        let mut distance: f64 = 0.0;
        for r in robots.iter() {
            let final_x = (r.x + r.vx * i).rem_euclid(cols);
            let final_y = (r.y + r.vy * i).rem_euclid(rows);

            for p in points.iter() {
                distance +=
                    (((final_x - p.0) * (final_x - p.0) + (final_y - p.1) * (final_y - p.1))
                        as f64)
                        .sqrt();
            }

            points.push((final_x, final_y));
        }
        total_distance += distance;

        min_distance = f64::min(min_distance, distance);

        if distance < 4000000.0 {
            println!("Iteration {}", i);

            let mut grid = vec![vec!['.'; cols as usize]; rows as usize];
            for r in robots.iter() {
                let final_x = (r.x + r.vx * i).rem_euclid(cols);
                let final_y = (r.y + r.vy * i).rem_euclid(rows);
                
                grid[final_y as usize][final_x as usize] = '*';
            }

            for r in grid.iter() {
                for c in r.iter() {
                    print!("{}", c);
                }
                println!();
            }

        }

    }

    println!(
        "Min distance is {} average is {}",
        min_distance,
        total_distance / (103.101)
    );

    0
}

fn main() {
    println!("Solution: {}", solve())
}
