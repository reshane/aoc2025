use std::fs;

pub fn solve() {
    let contents = fs::read_to_string("inputs/07.txt").expect("WHERE IS THE FILE");
    println!("part 1: {}", solve_p1(contents.clone()));
    println!("part 2: {}", solve_p2(contents.clone()));
}

fn parse_input(contents: String) -> Vec<Vec<i64>> {
    let mut beams: Vec<Vec<i64>> = vec![];
    contents.lines().enumerate().for_each(|(y, line)| {
        if y % 2 == 0 {
            beams.push(vec![]);
            line.char_indices().for_each(|(x, c)| {
                match c {
                    'S' | '^' => beams[y/2].push(x as i64),
                    _ => {},
                }
            });
        }
    });
    beams
}

use std::collections::HashSet;

fn solve_p1(contents: String) -> i64 {
    let mut beams = parse_input(contents);
    let mut total = 0;

    for i in 1..beams.len() {
        let incoming = beams[i-1].clone();
        let splitters = beams[i].clone();
        // for each beam, if there is a splitter
        // put a beam in the new_beams row on either side of it
        // and not where the splitter is
        // if there is no splitter, beam in new_beams
        let mut new_beams = HashSet::<i64>::new();
        for b in incoming.iter() {
            if splitters.contains(b) {
                total += 1;
                if !splitters.contains(&(b-1)) {
                    new_beams.insert(b-1);
                }
                if !splitters.contains(&(b+1)) {
                    new_beams.insert(b+1);
                }
            } else {
                new_beams.insert(*b);
            }
        }
        beams[i] = new_beams.into_iter().collect();
    }
    total
}

#[test]
fn test_case_1() {
    let input: Vec<& 'static str> = vec![
        ".......S.......",
        "...............",
        ".......^.......",
        "...............",
        "......^.^......",
        "...............",
        ".....^.^.^.....",
        "...............",
        "....^.^...^....",
        "...............",
        "...^.^...^.^...",
        "...............",
        "..^...^.....^..",
        "...............",
        ".^.^.^.^.^...^.",
        "...............",
    ];
    let result = solve_p1(input.join("\n"));
    println!("result: {result:?}");
    assert!(result == 21)
}

use std::collections::HashMap;

fn solve_p2(contents: String) -> i64 {
    let mut beams = parse_input(contents.clone());
    let mut curr: HashMap<i64, i64> = HashMap::new();
    beams[0].iter().for_each(|b| {
        curr.insert(*b, 1);
    });
    for i in 1..beams.len() {
        let incoming = beams[i-1].clone();
        let splitters = beams[i].clone();
        let mut next = HashMap::new();
        let mut new_beams = HashSet::<i64>::new();
        for b in incoming.iter() {
            if splitters.contains(b) {
                new_beams.insert(b-1);
                if let Some(v) = next.get_mut(&(b-1)) {
                    *v += curr.get(b).unwrap();
                } else {
                    next.insert(b-1, *curr.get(b).unwrap());
                }
                new_beams.insert(b+1);
                if let Some(v) = next.get_mut(&(b+1)) {
                    *v += curr.get(b).unwrap();
                } else {
                    next.insert(b+1, *curr.get(b).unwrap());
                }
            } else {
                new_beams.insert(*b);
                if let Some(v) = next.get_mut(b) {
                    *v += *curr.get(b).unwrap();
                } else {
                    next.insert(*b, *curr.get(b).unwrap());
                }
            }
        }
        beams[i] = new_beams.into_iter().collect();
        curr = next;
    }
    curr.values().fold(0, |v, acc| {
        v + acc
    })
}

#[test]
fn test_case_2() {
    let input: Vec<& 'static str> = vec![
        ".......S.......",
        "...............",
        ".......^.......",
        "...............",
        "......^.^......",
        "...............",
        ".....^.^.^.....",
        "...............",
        "....^.^...^....",
        "...............",
        "...^.^...^.^...",
        "...............",
        "..^...^.....^..",
        "...............",
        ".^.^.^.^.^...^.",
        "...............",
    ];
    let result = solve_p2(input.join("\n"));
    println!("result: {result:?}");
    assert!(result == 40)
}
