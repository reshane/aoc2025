use std::fs;

pub fn solve() {
    let contents = fs::read_to_string("inputs/03.txt").expect("WHERE IS THE FILE");
    println!("part 1: {}", solve_p1(contents.clone()));
    println!("part 2: {}", solve_p2(contents.clone()));
}

fn parse_input(contents: String) -> Vec<Vec<i64>> {
    contents.lines()
        .map(|line| {
            line.chars().map(|c| { c.to_string().parse().unwrap() }).collect()
        })
        .collect()
}

fn solve_p1(contents: String) -> i64 {
    let banks = parse_input(contents);
    let mut total = 0;
    for bank in banks {
        // need the largest digit from 0-len()-2
        // then second largest digit with idx > largest
        let mut i = 0;
        let mut max_idx = i;
        let mut max = -1;
        while i < bank.len()-1 {
            if bank[i] > max {
                max_idx = i;
                max = bank[i];
            }
            i += 1;
        }
        let mut j = max_idx + 1;
        let mut next_max = -1;
        while j < bank.len() {
            if bank[j] > next_max {
                next_max = bank[j];
            }
            j += 1;
        }
        total += max * 10 + next_max;
    }
    total
}

#[test]
fn test_case_1() {
    let input: Vec<& 'static str> = vec![
        "987654321111111",
        "811111111111119",
        "234234234234278",
        "818181911112111"
    ];
    let result = solve_p1(input.join("\n"));
    println!("result: {result:?}");
    assert!(result == 357)
}

fn get_max(bank: String) -> i64 {
    let num_len = 12;
    let mut m = vec![vec![String::new();num_len + 1]; bank.len() + 1];
    for i in 1..=bank.len() {
        for j in 1..=num_len {
            let q = &bank[i-1..i];
            if (m[i][j-1].clone() + q).len() > j {
                m[i][j] = m[i-1][j].clone();
            } else {
                let a = if m[i-1][j].len() == 0 { 0 } else { m[i-1][j].clone().parse::<i64>().unwrap() };
                let b = (m[i-1][j-1].clone() + q).parse().unwrap();
                if a > b {
                    m[i][j] = a.to_string();
                } else {
                    m[i][j] = b.to_string();
                }
            }
        }
    }
    m[bank.len()][num_len].parse().unwrap()
}

fn solve_p2(contents: String) -> i64 {
    let mut total = 0;
    contents.lines().for_each(|bank| {
        let max = get_max(bank.to_string());
        total += max;
    });
    total
}

#[test]
fn test_case_2() {
    let input: Vec<& 'static str> = vec![
        "987654321111111",
        "811111111111119",
        "234234234234278",
        "818181911112111"
    ];
    let result = solve_p2(input.join("\n"));
    println!("result: {result:?}");
    assert!(result == 3121910778619)
}
