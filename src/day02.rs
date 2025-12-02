use std::fs;

pub fn solve() {
    let contents = fs::read_to_string("inputs/02.txt").expect("WHERE IS THE FILE");
    println!("part 1: {}", solve_p1(contents.clone()));
    println!("part 2: {}", solve_p2(contents.clone()));
}

fn parse_input(contents: String) -> Vec<(i64, i64)> {
    contents.split(",").filter_map(|pair| {
        let pair = pair.trim();
        if pair.len() == 0 {
            return None
        }
        match pair.split_once("-") {
            Some((a, b)) => {
                Some((a.parse().unwrap(), b.parse().unwrap()))
            },
            None => panic!("Could not parse {}", pair),
        }
    })
    .collect()
}

fn is_invalid(id_str: String) -> bool {
    if id_str.len() % 2 != 0 {
        return false;
    }
    let (mut i, mut j) = (0, id_str.len()/2);
    while j < id_str.len() {
        if id_str[i..i+1] != id_str[j..j+1] {
            return false;
        }
        i += 1;
        j += 1;
    }
    return true;
}

fn solve_p1(contents: String) -> i64 {
    let ranges: Vec<(i64, i64)> = parse_input(contents);
    let mut total = 0;
    for (start, end) in ranges {
        for t in start..=end {
            if is_invalid(t.to_string()) {
                total += t;
            }
        }
    }
    total
}

#[test]
fn test_case_1() {
    let input: Vec<& 'static str> = vec![
        "11-22,95-115,998-1012,1188511880-1188511890,222220-222224,1698522-1698528,446443-446449,38593856-38593862,565653-565659,824824821-824824827,2121212118-2121212124"
    ];
    let result = solve_p1(input.join("\n"));
    println!("result: {result:?}");
    assert!(result == 1227775554)
}

fn is_invalid_2(id_str: String) -> bool {
    // cycle through possible lengths of substrings
    let id_len = id_str.len();
    for div in 1..=(id_len/2) {
        if id_len % div == 0 && repeats_n(&id_str, div) {
            return true;
        }
    }
    return false;
}

fn repeats_n(id_str: &String, div: usize) -> bool {
    let mut s = div;
    let control = &id_str[0..div];
    while s < id_str.len() {
        if &id_str[s..s+div] != control {
            return false;
        }
        s += div;
    }
    return true;
}

fn solve_p2(contents: String) -> i64 {
    let ranges = parse_input(contents);
    let mut total = 0;
    for (start, end) in ranges {
        for t in start..=end {
            if is_invalid_2(t.to_string()) {
                total += t;
            }
        }
    }
    total
}

#[test]
fn test_case_2() {
    let input: Vec<& 'static str> = vec![
        "11-22,95-115,998-1012,1188511880-1188511890,222220-222224,1698522-1698528,446443-446449,38593856-38593862,565653-565659,824824821-824824827,2121212118-2121212124"
    ];
    let result = solve_p2(input.join("\n"));
    println!("result: {result:?}");
    assert!(result == 4174379265)
}
