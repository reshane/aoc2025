use std::fs;

pub fn solve() {
    let contents = fs::read_to_string("inputs/06.txt").expect("WHERE IS THE FILE");
    println!("part 1: {}", solve_p1(contents.clone()));
    println!("part 2: {}", solve_p2(contents.clone()));
}

fn parse_input(contents: String) -> (Vec<Vec<i64>>, Vec<String>) {
    let mut problems: Vec<Vec<i64>> = vec![];
    let mut ops: Vec<String> = vec![];
    contents.lines().for_each(|line| {
        let mut probs = vec![];
        line.split(" ").for_each(|x| {
            let x = x.trim();
            // println!("{x}");
            if let Ok(x) = x.parse() {
                probs.push(x);
            } else if x.len() > 0 {
                ops.push(x.to_string())
            }
        });
        if probs.len() != 0 {
            problems.push(probs);
        }
    });
    (problems, ops)
}

fn solve_p1(contents: String) -> i64 {
    let (problems, ops) = parse_input(contents);
    // println!("{problems:?} {ops:?}");
    let mut total = 0;
    for (idx, op) in ops.into_iter().enumerate() {
        let mut sub = if op == "*" { 1 } else { 0 };
        for prob in &problems {
            if op == "*" {
                sub *= prob[idx];
            } else {
                sub += prob[idx];
            }
        }
        total += sub;
    }
    total
}

#[test]
fn test_case_1() {
    let input: Vec<& 'static str> = vec![
        "123 328  51 64 ",
        " 45 64  387 23 ",
        "  6 98  215 314",
        "*   +   *   +  "
    ];
    let result = solve_p1(input.join("\n"));
    println!("result: {result:?}");
    assert!(result == 4277556)
}

fn parse_input_2(contents: String) -> (Vec<Vec<i64>>, Vec<String>) {
    let raw_lines: Vec<&str> = contents.lines().collect();
    let mut col_lines: Vec<String> = vec![];
    for (y, raw_line) in raw_lines.iter().enumerate() {
        if y == raw_lines.len()-1 {
            break;
        }
        for (x, c) in raw_line.char_indices() {
            if col_lines.len() <= x {
                col_lines.push(c.to_string());
            } else {
                col_lines[x] += &c.to_string();
            }
        }
    }
    let mut column_lines: Vec<Vec<String>> = vec![vec![]];
    let mut c_idx = 0;
    col_lines.into_iter().for_each(|line| {
        if line.trim().len() == 0 {
            c_idx += 1;
            column_lines.push(vec![]);
        } else {
            column_lines[c_idx].push(line);
        }
    });
    let problems: Vec<Vec<i64>> = column_lines.into_iter().map(|prob| {
        prob.into_iter().filter_map(|n| {
            n.trim().parse::<i64>().ok()
        }).collect::<Vec<i64>>()
    }).collect();
    let ops = raw_lines[raw_lines.len()-1].chars().filter_map(|c| {
        if c == ' ' {
            None
        } else {
            Some(c.to_string())
        }
    }).collect::<Vec<String>>();
    (problems, ops)
}

fn solve_p2(contents: String) -> i64 {
    let (problems, ops) = parse_input_2(contents);
    let mut total = 0;
    for (idx, op) in ops.into_iter().enumerate() {
        total += problems[idx].iter().fold(if op == "*" { 1 } else { 0 }, |acc, x| {
            if op == "*" { acc * x } else { acc + x }
        });
    }
    total
}

#[test]
fn test_case_2() {
    let input: Vec<& 'static str> = vec![
        "123 328  51 64 ",
        " 45 64  387 23 ",
        "  6 98  215 314",
        "*   +   *   +  "
    ];
    let result = solve_p2(input.join("\n"));
    println!("result: {result:?}");
    assert!(result == 3263827)
}
