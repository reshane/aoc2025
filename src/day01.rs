use std::fs;

pub fn solve() {
    let contents = fs::read_to_string("inputs/01.txt").expect("WHERE IS THE FILE");
    println!("part 1: {}", solve_p1(contents.clone()));
    println!("part 2: {}", solve_p2(contents.clone()));
}

fn parse_input(contents: String) -> Vec<i64> {
    let dirs = contents.lines()
        .map(|line| {
            match &line[0..1] {
                "L" => {
                    let mag: i64 = match &line[1..].parse() {
                        Ok(i) => *i,
                        Err(_) => panic!("Could not parse {}", line),
                    };
                    -1 * mag
                },
                "R" => {
                    let mag: i64 = match &line[1..].parse() {
                        Ok(i) => *i,
                        Err(_) => panic!("Could not parse {}", line),
                    };
                    mag
                },
                _ => panic!("Unrecognized line {}", line)
            }
        })
        .collect();
    dirs
}

fn solve_p1(contents: String) -> i64 {
    let dirs: Vec<i64> = parse_input(contents);
    let mut curr = 50;
    let mut total = 0;
    for d in dirs {
        if d < 0 {
            curr += 100 + d;
        } else {
            curr += d;
        }

        curr = curr % 100;
        
        if curr == 0 {
            total += 1
        }
    }
    total
}

#[test]
fn test_case_1() {
    let input: Vec<& 'static str> = vec![
        "L68", "L30", "R48", "L5", "R60", "L55", "L1", "L99", "R14", "L82"
    ];
    let result = solve_p1(input.join("\n"));
    println!("result: {result:?}");
    assert!(result == 3)
}

fn solve_p2(contents: String) -> i64 {
    let dirs: Vec<i64> = parse_input(contents);
    let mut curr = 50;
    let mut total = 0;
    for d in dirs {

        let prev = curr;
        if d >= 100 {
            total += d / 100
        } else if d <= -100 {
            total += (-1 * d) / 100
        }
        curr += d % 100;
        if curr < 0 {
            // negative
            if prev != 0 {
                total += 1;
            }
            curr = (curr % 100) + 100;
        } else if curr >= 100 {
            total += 1;
            curr = curr % 100;
        } else if curr == 0 {
            total += 1;
        }

    }
    total
}

#[test]
fn test_case_2() {
    let input: Vec<& 'static str> = vec![
        "L68", "L30", "R48", "L5", "R60", "L55", "L1", "L99", "R14", "L82"
    ];
    let result = solve_p2(input.join("\n"));
    println!("result: {result:?}");
    assert!(result == 6)
}
