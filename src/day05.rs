use std::fs;

pub fn solve() {
    let contents = fs::read_to_string("inputs/05.txt").expect("WHERE IS THE FILE");
    println!("part 1: {}", solve_p1(contents.clone()));
    println!("part 2: {}", solve_p2(contents.clone()));
}

fn parse_input(contents: String) -> (Vec<(i64, i64)>, Vec<i64>) {
    if let Some((ranges, ingredients)) = contents.split_once("\n\n") {
        let mut fresh = Vec::<(i64, i64)>::new();
        ranges.lines().for_each(|line| {
            if let Some((start, end)) = line.split_once("-") {
                let start = start.parse().unwrap();
                let end = end.parse().unwrap();
                fresh.push((start, end));
            } else {
                panic!("Could not split range on \"-\" {}", line);
            }
        });
        let mut ingredients = ingredients.lines().map(|line| {
            line.parse().unwrap()
        }).collect::<Vec<i64>>();

        ingredients.sort();

        fresh.sort_by(|a, b| {
            a.0.cmp(&b.0)
        });

        return (fresh, ingredients);
    }
    panic!("Could not split on double newline! {}", contents)
}

fn solve_p1(contents: String) -> i64 {
    let (fresh, ingredients) = parse_input(contents);
    let mut total = 0;
    for ing in ingredients.iter() {
        'r: for range in fresh.iter() {
            if range.0 <= *ing && range.1 >= *ing {
                total += 1;
                break 'r;
            }
        }
    }
    total
}

#[test]
fn test_case_1() {
    let _input: Vec<& 'static str> = vec![
        "3-5",
        "10-14",
        "",
        "5",
    ];
    let input: Vec<& 'static str> = vec![
        "3-5",
        "10-14",
        "16-20",
        "12-18",
        "",
        "1",
        "5",
        "8",
        "11",
        "17",
        "32",
    ];
    let result = solve_p1(input.join("\n"));
    println!("result: {result:?}");
    assert!(result == 3)
}

fn solve_p2(contents: String) -> i64 {
    // combine all the ranges
    // stack of ranges
    // if the stack isn't empty, only push if start > stack.top.end + 1
    // 3-5, next start must be > 6
    // to combine, set stack.top.end to max(stack.top.end, end)
    let mut stack: Vec<(i64,i64)> = vec![];
    let (ranges, _) = parse_input(contents);
    for range in ranges.iter() {
        if stack.is_empty() {
            stack.push(*range);
        } else {
            let top_idx = stack.len()-1;
            let top = stack[top_idx];
            if range.0 > top.1 + 1 {
                stack.push(*range);
            } else if stack[top_idx].1 < range.1 {
                stack[top_idx].1 = range.1;
            }
        }
    }

    let mut total = 0;
    for range in stack.iter() {
        total += range.1 - range.0 + 1
    }
    total
}

#[test]
fn test_case_2() {
    let input: Vec<& 'static str> = vec![
        "3-5",
        "10-14",
        "16-20",
        "12-18",
        "",
        "1",
        "5",
        "8",
        "11",
        "17",
        "32",
    ];
    let result = solve_p2(input.join("\n"));
    println!("result: {result:?}");
    assert!(result == 14)
}
