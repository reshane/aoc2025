use std::fs;

pub fn solve() {
    let contents = fs::read_to_string("inputs/09.txt").expect("WHERE IS THE FILE");
    println!("part 1: {}", solve_p1(contents.clone()));
    println!("part 2: {}", solve_p2(contents.clone()));
}

type Pos = (i64, i64);

fn parse_input(contents: String) -> Vec<Pos> {
    contents.lines().map(|line| {
        let parts: Vec<&str> = line.split(",").collect();
        (parts[0].parse().unwrap(), parts[1].parse().unwrap())
    }).collect()
}

fn calc_area(a: Pos, b: Pos) -> i64 {
    let w = 1+(b.0-a.0).abs();
    let h = 1+(b.1-a.1).abs();
    w * h
}

fn solve_p1(contents: String) -> i64 {
    let mut tiles = parse_input(contents);
    tiles.sort_by(|a, b| {
        if a.0.cmp(&b.0) == std::cmp::Ordering::Equal {
            a.1.cmp(&b.1)
        } else {
            a.0.cmp(&b.0)
        }
    });
    let mut i = 0;
    let mut max = 0;
    while i < tiles.len()-1 {
        let mut j = i+1;
        while j < tiles.len() {
            let area = calc_area(tiles[i], tiles[j]);
            if area > max {
                max = area;
            }
            j += 1;
        }
        i += 1;
    }
    max
}

#[test]
fn test_case_1() {
    let input: Vec<& 'static str> = vec![
        "7,1",
        "11,1",
        "11,7",
        "9,7",
        "9,5",
        "2,5",
        "2,3",
        "7,3",
    ];
    let result = solve_p1(input.join("\n"));
    println!("result: {result:?}");
    assert!(result == 50)
}

use std::cmp::{min, max};
use std::collections::HashSet;

fn set_perim(tiles: &Vec<Pos>) -> HashSet<Pos> {
    let mut perim = HashSet::new();
    let mut i = 0;
    while i < tiles.len() {
        let j = (i+1)%tiles.len();
        let (a, b) = (tiles[i], tiles[j]);
        if a.0 == b.0 {
            let mut curr = (a.0, min(a.1, b.1));
            while curr.1 <= max(a.1, b.1) {
                perim.insert(curr);
                curr.1 += 1;
            }
        } else {
            let mut curr = (min(a.0, b.0), a.1);
            while curr.0 <= max(a.0, b.0) {
                perim.insert(curr);
                curr.0 += 1;
            }
        }
        i += 1;
    }
    perim
}

fn validate_rect(i: usize, j: usize, tiles: &Vec<Pos>, perim: &HashSet<Pos>) -> bool {
    let (a, b) = (tiles[i], tiles[j]);
    let (top_y, bottom_y) = (max(a.1, b.1), min(a.1, b.1));
    let (right_x, left_x) = (max(a.0, b.0), min(a.0, b.0));

    for t in perim.iter() {
        if t.0 > left_x && t.0 < right_x && t.1 > bottom_y && t.1 < top_y {
            return false;
        }
    }
    true
}

fn solve_p2(contents: String) -> i64 {
    let tiles = parse_input(contents);
    let perim = set_perim(&tiles);
    let mut i = 0;
    let mut max = 0;
    while i < tiles.len()-1 {
        let mut j = i+1;
        while j < tiles.len() {
            let area = calc_area(tiles[i], tiles[j]);
            if area > max {
                let valid = validate_rect(i, j, &tiles, &perim);
                if valid {
                    max = area;
                }
            }
            j += 1;
        }
        i += 1;
    }
    max
}

#[test]
fn test_case_2() {
    let input: Vec<& 'static str> = vec![
        "7,1",
        "11,1",
        "11,7",
        "9,7",
        "9,5",
        "2,5",
        "2,3",
        "7,3",
    ];
    let result = solve_p2(input.join("\n"));
    println!("result: {result:?}");
    assert!(result == 24)
}
