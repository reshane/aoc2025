use std::fs;

pub fn solve() {
    let contents = fs::read_to_string("inputs/12.txt").expect("WHERE IS THE FILE");
    println!("part 1: {}", solve_p1(contents.clone()));
}

fn parse_input(contents: String) -> (Vec<[[u8; 3]; 3]>, Vec<((i64,i64), Vec<i64>)>) {
    let mut shapes = vec![];
    let mut problems = vec![];
    contents.split("\n\n").enumerate().for_each(|(idx, block)| {
        if idx < 6 {
            let shape: &mut [[u8; 3]; 3] = &mut [[0; 3]; 3];
            block.lines().enumerate().for_each(|(y, line)| {
                if y != 0 {
                    line.char_indices().for_each(|(x, c)| {
                        shape[y-1][x] = match c {
                            '#' => 1,
                            '.' => 0,
                            _ => {
                                panic!("unrecognized char: {}", c);
                            },
                        };
                    });
                }
            });
            shapes.push(shape.clone());
        } else {
            block.lines().for_each(|line| {
                if let Some((dims, reqs)) = line.split_once(": ") {
                    let (width, height): (i64,i64) = match dims.split_once("x") {
                        Some((w, h)) => (w.parse().unwrap(), h.parse().unwrap()),
                        None => {
                            panic!("Could not parse width & height for {}", dims);
                        },
                    };
                    let requirements = reqs.split(" ").map(|r| r.parse().unwrap()).collect();
                    problems.push(((width, height), requirements));
                }
            });
        }
    });
    (shapes, problems)
}

#[allow(unused)]
fn print_shape(shape: &[[u8; 3]; 3]) {
    for y in 0..3 {
        for x in 0..3 {
            if shape[y][x] == 1 {
                print!("#");
            } else {
                print!(".");
            }
        }
        println!("");
    }
}


fn shape_min_spaces(shape: &[[u8; 3]; 3]) -> u8 {
    shape.iter().fold(0_u8, |r_acc, row| {
        let row_sum: u8 = row.iter().fold(0, |acc, e| acc + e);
        row_sum + r_acc
    })
}

fn solve_p1(contents: String) -> i64 {
    let (shapes, problems) = parse_input(contents);
    let min_per_shape: Vec<u8> = shapes.iter().map(|s| shape_min_spaces(s)).collect();
    let mut total_yes = 0;
    #[allow(unused)]
    let mut total_no = 0;
    for ((w,h), reqs) in problems.iter() {
        let min_spaces = reqs.iter().enumerate().fold(0_i64, |acc, (idx, c)| {
            acc + (c * min_per_shape[idx] as i64)
        });
        let max_spaces = reqs.iter().fold(0_i64, |acc, c| {
            acc + (c * 9_i64)
        });
        if w*h >= max_spaces {
            total_yes += 1;
        }
        if w*h < min_spaces {
            total_no += 1;
        }
    }
    // println!("{total_yes} {total_no} {} ", problems.len() - (total_yes + total_no));
    total_yes as i64
}

// #[test]
#[allow(unused)]
fn test_case_1() {
    let contents = fs::read_to_string("inputs/12.txt").expect("WHERE IS THE FILE");
    let result = solve_p1(contents);
    println!("result: {result:?}");
    assert!(result == 505)
}
