use std::collections::HashSet;
use std::fs::read_to_string;

#[derive(PartialEq, Debug, Clone, Copy)]
enum Direction {
    N,
    S,
    E,
    W,
}

#[derive(Debug, Clone, Copy)]
struct Edge {
    dir: Direction,
    rowmin: usize,
    rowmax: usize,
    colmin: usize,
    colmax: usize,
}

fn add_edge(edges: &mut Vec<Edge>, edge: Edge) -> bool {
    for e in edges.iter_mut() {
        if e.dir == edge.dir {
            if e.dir == Direction::N || e.dir == Direction::S {
                if e.rowmax != edge.rowmax {
                    continue;
                }

                if e.colmax + 1 == edge.colmin {
                    e.colmax = edge.colmax;
                    return true;
                }

                if edge.colmax + 1 == e.colmin {
                    e.colmin = edge.colmin;
                    return true;
                }
            }
            if e.dir == Direction::E || e.dir == Direction::W {
                if e.colmax != edge.colmax {
                    continue;
                }

                if e.rowmax + 1 == edge.rowmin {
                    e.rowmax = edge.rowmax;
                    return true;
                }

                if edge.rowmax + 1 == e.rowmin {
                    e.rowmin = edge.rowmin;
                    return true;
                }
            }
        }
    }

    edges.push(edge);
    return false;
}

fn combine_edges(edges: &mut Vec<Edge>) -> bool {
    let mut newedges = Vec::<Edge>::new();

    let mut result = false;

    for e in edges.iter() {
        result |= add_edge(&mut newedges, *e);
    }

    *edges = newedges;
    result
}

fn read_input() -> Vec<String> {
    read_to_string("input.txt")
        .unwrap() // panic on possible file-reading errors
        .lines() // split the string into an iterator of string slices
        .map(String::from) // make each slice into a string
        .collect() // gather them together into a vector
}

fn visit(
    map: &[Vec<char>],
    visited: &mut HashSet<(usize, usize)>,
    edges: &mut Vec<Edge>,
    plant: char,
    row: usize,
    col: usize,
) -> usize {
    let mut area: usize = 0;

    if visited.contains(&(row, col)) || map[row][col] != plant {
        return 0;
    }

    visited.insert((row, col));

    area += 1;

    if row == 0 || map[row - 1][col] != plant {
        add_edge(
            edges,
            Edge {
                dir: Direction::N,
                colmin: col,
                colmax: col,
                rowmin: row,
                rowmax: row,
            },
        );
    }

    if row == map.len() - 1 || map[row + 1][col] != plant {
        add_edge(
            edges,
            Edge {
                dir: Direction::S,
                colmin: col,
                colmax: col,
                rowmin: row,
                rowmax: row,
            },
        );
    }

    if col == 0 || map[row][col - 1] != plant {
        add_edge(
            edges,
            Edge {
                dir: Direction::W,
                colmin: col,
                colmax: col,
                rowmin: row,
                rowmax: row,
            },
        );
    }

    if col == map[0].len() - 1 || map[row][col + 1] != plant {
        add_edge(
            edges,
            Edge {
                dir: Direction::E,
                colmin: col,
                colmax: col,
                rowmin: row,
                rowmax: row,
            },
        );
    }

    if row != 0 {
        area += visit(map, visited, edges, plant, row - 1, col);
    }

    if row != map.len() - 1 {
        area += visit(map, visited, edges, plant, row + 1, col);
    }

    if col != 0 {
        area += visit(map, visited, edges, plant, row, col - 1);
    }

    if col != map[0].len() - 1 {
        area += visit(map, visited, edges, plant, row, col + 1);
    }
    area
}

fn solve() -> usize {
    let map = read_input()
        .iter()
        .map(|l| l.chars().collect::<Vec<char>>())
        .collect::<Vec<Vec<char>>>();

    let mut visited = HashSet::<(usize, usize)>::new();

    let mut total: usize = 0;

    for row in 0..map.len() {
        for col in 0..map[0].len() {
            if visited.contains(&(row, col)) {
                continue;
            }

            let mut edges = Vec::<Edge>::new();

            let area = visit(&map, &mut visited, &mut edges, map[row][col], row, col);

            while combine_edges(&mut edges) {}

            total += edges.len() * area;
        }
    }

    total
}

fn main() {
    println!("Solution: {}", solve())
}
