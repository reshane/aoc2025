use std::fs;

pub fn solve() {
    let contents = fs::read_to_string("inputs/11.txt").expect("WHERE IS THE FILE");
    println!("part 1: {}", solve_p1(contents.clone()));
    println!("part 2: {}", solve_p2(contents.clone()));
}

use std::collections::HashMap;

fn parse_input(contents: String) -> HashMap<String, Vec<String>> {
    let mut map: HashMap<String, Vec<String>> = HashMap::new();

    contents.lines().for_each(|line| {
        if let Some((node, nbors)) = line.split_once(": ") {
            map.insert(node.to_string(), nbors.trim().split(" ").map(|s| s.to_string()).collect());
        }
    });

    map
}

fn solve_p1(contents: String) -> i64 {
    let map = parse_input(contents);
    let mut total = 0;
    let mut queue = vec![];
    queue.push(("you", 0));
    while !queue.is_empty() {
        let (curr, dist) = queue.pop().unwrap();
        if let Some(nbors) = map.get(curr) {
            for nbor in nbors.iter() {
                if nbor == "out" {
                    total += 1;
                } else {
                    queue.push((nbor, dist+1));
                }
            }
        }
    }
    total
}

#[test]
fn test_case_1() {
    let input: Vec<& 'static str> = vec![
        "aaa: you hhh",
        "you: bbb ccc",
        "bbb: ddd eee",
        "ccc: ddd eee fff",
        "ddd: ggg",
        "eee: out",
        "fff: out",
        "ggg: out",
        "hhh: ccc fff iii",
        "iii: out",
    ];
    let result = solve_p1(input.join("\n"));
    println!("result: {result:?}");
    assert!(result == 5)
}

use std::collections::HashSet;
use std::collections::VecDeque;

// topo sort
fn topo_sort(map: &HashMap<String, Vec<String>>) -> Vec<String> {
    let mut stack: VecDeque<(String, bool)> = VecDeque::new();
    stack.push_back(("svr".to_string(), false));
    let mut post_order = vec![];
    let mut visited = HashSet::new();
    while !stack.is_empty() {
        let curr = stack.pop_back().unwrap();
        if curr.1 {
            post_order.push(curr.0);
            continue;
        }
        if visited.contains(&curr.0) {
            continue;
        }
        visited.insert(curr.0.clone());
        stack.push_back((curr.0.clone(), true));
        if let Some(nbors) = map.get(&curr.0) {
            nbors.iter().filter(|nbor| !visited.contains(&nbor.to_string())).for_each(|n| {
                stack.push_back((n.to_string(), false));
            });
        }
    }
    post_order
}

fn paths(dst: String, src: String, sorted: &Vec<String>, map: &HashMap<String, Vec<String>>) -> i64 {
    // flood fill from dst to src with generation
    let start = sorted.iter().position(|e| **e == dst).unwrap();
    let end = sorted.iter().position(|e| **e == src).unwrap();
    if start > end {
        return 0;
    }
    let mut visited = HashMap::new();
    visited.insert(dst, 1);
    for idx in start+1..=end {
        let curr = &sorted[idx];
        let mut ct = 0_i64;
        if let Some(nbors) = map.get(curr) {
            for nbor in nbors.iter() {
                if let Some(nct) = visited.get(nbor) {
                    ct += nct;
                }
            }
        }
        visited.insert(curr.to_string(), ct);
    }
    *visited.get(&src).unwrap()
}

fn solve_p2(contents: String) -> i64 {
    // svr -> fft -> dac -> out
    // svr -> dac -> fft -> out
    let map = parse_input(contents);
    let sorted = topo_sort(&map);

    (paths("dac".to_string(), "svr".to_string(), &sorted, &map)
    * paths("fft".to_string(), "dac".to_string(), &sorted, &map)
    * paths("out".to_string(), "fft".to_string(), &sorted, &map))
    +
    (paths("fft".to_string(), "svr".to_string(), &sorted, &map)
    * paths("dac".to_string(), "fft".to_string(), &sorted, &map)
    * paths("out".to_string(), "dac".to_string(), &sorted, &map))
}

#[test]
fn test_case_2() {
    let input: Vec<& 'static str> = vec![
        "svr: aaa bbb",
        "aaa: fft",
        "fft: ccc",
        "bbb: tty",
        "tty: ccc",
        "ccc: ddd eee",
        "ddd: hub",
        "hub: fff",
        "eee: dac",
        "dac: fff",
        "fff: ggg hhh",
        "ggg: out",
        "hhh: out",
    ];
    let result = solve_p2(input.join("\n"));
    println!("result: {result:?}");
    assert!(result == 2)
}
