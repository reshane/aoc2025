use std::fs;

pub fn solve() {
    let contents = fs::read_to_string("inputs/04.txt").expect("WHERE IS THE FILE");
    println!("part 1: {}", solve_p1(contents.clone()));
    println!("part 2: {}", solve_p2(contents.clone()));
}

use std::collections::HashMap;

type Pos = (i64, i64);

fn parse_input(contents: String) -> (HashMap<Pos, String>, Pos) {
    let mut map = HashMap::<Pos, String>::new();
    let (mut max_x, mut max_y): Pos = (0,0);
    contents.lines().enumerate().for_each(|(y, line)| {
        if y as i64 > max_y {
            max_y = y as i64;
        }
        for (x, c) in line.char_indices() {
            if x as i64 > max_x {
                max_x = x as i64;
            }
            match c {
                '.' => map.insert((x as i64, y as i64), ".".to_string()),
                '@' => map.insert((x as i64, y as i64), "@".to_string()),
                _ => panic!("Unrecognized char in map {}", c),
            };
        }
    });
    (map, (max_x+1, max_y+1))
}

const DIRS: [Pos; 8] = [
    (-1, -1), ( 0, -1), ( 1, -1),
    (-1,  0),           ( 1,  0), 
    (-1,  1), ( 0,  1), ( 1,  1),
];

fn solve_p1(contents: String) -> i64 {
    let (map, (width, height)) = parse_input(contents);
    let mut total = 0;
    for y in 0..height {
        'xl: for x in 0..width {
            if let Some(curr) = map.get(&(x, y)) {
                match curr.as_str() {
                    "." => continue 'xl,
                    _ => {},
                }
            }
            let mut ct = 0;
            for d in DIRS.iter() {
                if let Some(nbor) = map.get(&(x + d.0, y + d.1)) {
                    match nbor.as_str() {
                        "@" => {
                            ct += 1;
                        },
                        _ => {},
                    }
                }
            }
            if ct < 4 {
                total += 1;
            }
        }
    }
    total
}

#[test]
fn test_case_1() {
    let input: Vec<& 'static str> = vec![
        "..@@.@@@@.",
        "@@@.@.@.@@",
        "@@@@@.@.@@",
        "@.@@@@..@.",
        "@@.@@@@.@@",
        ".@@@@@@@.@",
        ".@.@.@.@@@",
        "@.@@@.@@@@",
        ".@@@@@@@@.",
        "@.@.@@@.@.",
    ];
    let result = solve_p1(input.join("\n"));
    println!("result: {result:?}");
    assert!(result == 13)
}

#[allow(unused)]
fn print_map(map: &HashMap<Pos, String>, width: i64, height: i64) {
    for y in 0..height {
        for x in 0..width {
            if let Some(curr) = map.get(&(x,y)) {
                print!("{curr}");
            }
        }
        println!("");
    }
}

fn remove_and_total(map: &mut HashMap<Pos, String>, width: i64, height: i64) -> Option<i64> {
    let mut to_remove: Vec<Pos> = Vec::new();
    for y in 0..height {
        'xl: for x in 0..width {
            if let Some(curr) = map.get(&(x, y)) {
                match curr.as_str() {
                    "." => continue 'xl,
                    _ => {},
                }
            }
            let mut ct = 0;
            for d in DIRS.iter() {
                if let Some(nbor) = map.get(&(x + d.0, y + d.1)) {
                    match nbor.as_str() {
                        "@" => {
                            ct += 1;
                        },
                        _ => {},
                    }
                }
            }
            if ct < 4 {
                to_remove.push((x, y));
            }
        }
    }
    for (x, y) in to_remove.iter() {
        map.insert((*x, *y), ".".to_string());
    }
    if to_remove.is_empty() {
        None
    } else {
        Some(to_remove.len() as i64)
    }
}

fn solve_p2(contents: String) -> i64 {
    let (mut map, (width, height)) = parse_input(contents);
    let mut total = 0;
    while let Some(removed) = remove_and_total(&mut map, width, height) {
        total += removed;
    }
    total
}

#[test]
fn test_case_2() {
    let input: Vec<& 'static str> = vec![
        "..@@.@@@@.",
        "@@@.@.@.@@",
        "@@@@@.@.@@",
        "@.@@@@..@.",
        "@@.@@@@.@@",
        ".@@@@@@@.@",
        ".@.@.@.@@@",
        "@.@@@.@@@@",
        ".@@@@@@@@.",
        "@.@.@@@.@.",
    ];
    let result = solve_p2(input.join("\n"));
    println!("result: {result:?}");
    assert!(result == 43)
}
