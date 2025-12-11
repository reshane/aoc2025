use std::{fs, i64};

pub fn solve() {
    let contents = fs::read_to_string("inputs/10.txt").expect("WHERE IS THE FILE");
    println!("part 1: {}", solve_p1(contents.clone()));
    println!("part 2: {}", solve_p2(contents.clone()));
}

fn parse_input(contents: String) -> Vec<(u64, Vec<u64>, Vec<i64>)> {
    let mut res = vec![];
    contents.lines().for_each(|line| {
        let s1 = line.find("(").unwrap();
        let s2 = line.find("{").unwrap();
        let lights = &line[0..s1-1];
        let buttons = &line[s1..s2-1];
        let mut t: u64 = 0;
        let digits = lights[1..lights.len()-1].len();
        lights[1..lights.len()-1].chars().for_each(|c| {
            t = t << 1;
            match c {
                '#' => {
                    t += 1;
                },
                '.' => {},
                _ => {},
            }
        });
        let mut b: Vec<u64> = vec![];
        buttons.split(" ").for_each(|button| {
            let button = &button[1..button.len()-1];
            let mut bi: u64 = 0;
            button.split(",").for_each(|b| {
                let idx = b.parse::<usize>().unwrap();
                bi |= 1 << ((digits-1)-idx);
            });
            b.push(bi);
        });

        res.push((t, b, vec![]));
    });
    res
}

fn solve_p1(contents: String) -> i64 {
    let input = parse_input(contents);
    let mut total = 0;
    // result & number of nodes it took to get there
    for (lights, buttons, _) in input.iter() {
        // start with 0
        // create a node in the tree for each button by xoring it with parent node
        let mut generation = 0_i64;
        let mut queue: Vec<u64> = vec![];
        queue.push(0_u64);
        'outer: loop {
            // we want to drain the queue in here
            generation += 1;
            let mut new_queue = vec![];
            while let Some(start) = queue.pop() {
                for b in buttons.iter() {
                    if start ^ b == *lights {
                        total += generation;
                        break 'outer;
                    }
                    new_queue.push(start^b);
                }
            } 
            queue = new_queue;
        }
        // println!("{total}");
    }
    total
}

#[test]
fn test_case_1() {
    let input: Vec<& 'static str> = vec![
        "[.##.] (3) (1,3) (2) (2,3) (0,2) (0,1) {3,5,4,7}",
        "[...#.] (0,2,3,4) (2,3) (0,4) (0,1,2) (1,2,3,4) {7,5,12,7,2}",
        "[.###.#] (0,1,2,3,4) (0,3,4) (0,1,2,4,5) (1,2) {10,11,11,5,10,5}",
    ];
    let result = solve_p1(input.join("\n"));
    println!("result: {result:?}");
    assert!(result == 7)
}

fn parse_input_2(contents: String) -> Vec<(u64, Vec<Vec<i64>>, Vec<i64>)> {
    let mut res = vec![];
    contents.lines().for_each(|line| {
        let s1 = line.find("(").unwrap();
        let s2 = line.find("{").unwrap();
        let lights = &line[0..s1-1];
        let buttons = &line[s1..s2-1];
        let jolts = &line[s2..];
        let mut t: u64 = 0;
        lights[1..lights.len()-1].chars().for_each(|c| {
            t = t << 1;
            match c {
                '#' => {
                    t += 1;
                },
                '.' => {},
                _ => {},
            }
        });
        let b = buttons.split(" ").map(|button| {
            let button = &button[1..button.len()-1];
            button.split(",").map(|b| {
                b.parse().unwrap()
            }).collect()
        }).collect();


        let joltages = jolts[1..jolts.len()-1].split(",").map(|j| {
            j.parse().unwrap()
        }).collect();

        res.push((t, b, joltages));
    });
    res
}

fn solve_p2(contents: String) -> i64 {
    let input = parse_input_2(contents);
    let mut total = 0;
    for (_, buttons, joltages) in input.iter() {
        let mut mat = vec![vec![0; buttons.len()+1]; joltages.len()];
        for idx in 0..mat.len() {
            mat[idx][buttons.len()] = joltages[idx];
            for i in 0..buttons.len() {
                if buttons[i].contains(&(idx as i64)) {
                    mat[idx][i] = 1;
                }
            }
        }
        // we have constructed our matrix
        // we need to reduce our matrix
        // gaussian elimination
        let reduced = reduce_matrix(&mut mat);
        let bounds = calc_bounds(&buttons, &joltages);
        if let Some(rs) = search(&reduced,  &vec![None; reduced[0].len()-1], &bounds, None) {
            total += rs;
        } else {
            panic!("Could not solve problem!");
        }
    }
    total
}

fn calc_bounds(buttons: &Vec<Vec<i64>>, joltages: &Vec<i64>) -> Vec<i64> {
    let mut bounds = vec![0; buttons.len()];
    for (idx, b) in buttons.iter().enumerate() {
        let mut min = i64::MAX;
        for s in b.iter() {
            if min > joltages[*s as usize] {
                min = joltages[*s as usize];
            }
        }
        bounds[idx] = min;
    }
    bounds
}

fn get_gcd(row: &Vec<i64>) -> i64 {
    let mut prev = row[0];
    for i in 1..row.len() {
        let g = gcd(prev, row[i]);
        if g == 0 {
            return 0;
        }
        prev = g;
    }
    prev
}

fn gcd(m: i64, n: i64) -> i64 {
    let mut m = m.abs();
    let mut n = n.abs();
    loop {
        if m == 0 {
            return n;
        } else {
            let old_m = m;
            m = n % m;
            n = old_m;
        }
    }
}

fn reduce_matrix(mat: &mut Vec<Vec<i64>>) -> Vec<Vec<i64>> {
    let mut reduced = vec![];
    while !mat.is_empty() {
        // take whatever row
        let curr = mat.pop().unwrap();
        match curr.iter().position(|b| *b != 0) {
            Some(b_idx) => {
                // zero out every other row's b_idx
                for r_idx in 0..mat.len() {
                    let a = curr[b_idx];
                    let b = mat[r_idx][b_idx];
                    for i in 0..mat[r_idx].len() {
                        mat[r_idx][i] = mat[r_idx][i] * a - curr[i] * b;
                    }
                    let mut row_gcd = get_gcd(&mat[r_idx]);
                    if row_gcd != 0 {
                        if mat[r_idx][mat[r_idx].len()-1] < 0 {
                            row_gcd *= -1;
                        }
                        for i in 0..mat[r_idx].len() {
                            mat[r_idx][i] /= row_gcd;
                        }
                    }
                }
                reduced.push(curr);
            },
            None => { }
        }
    }
    reduced
}

fn search(mat: &Vec<Vec<i64>>, known: &Vec<Option<i64>>, bounds: &Vec<i64>, result_sum: Option<i64>) -> Option<i64> {
    let known_values = match get_known_values(&mat, known) {
        Some(known) => known,
        None => {
            return result_sum;
        },
    };

    if let Some(rs) = result_sum {
        if rs < known_values.iter().filter_map(|e| *e).fold(0, |acc, x| acc+x) {
            return result_sum;
        }
    }

    let all_known = known_values.iter().fold(true, |acc, x| acc && x.is_some());

    if all_known {
        let sum_known = known_values.iter().fold(0, |acc, x| acc + x.unwrap());
        match result_sum {
            Some(rs) if rs > sum_known => {
                return Some(sum_known);
            },
            Some(_) => {
                return result_sum;
            },
            None => {
                return Some(sum_known);
            },
        }
    }

    let mut selected_btn = None;
    let mut selected_count_unknown = mat[0].len()-1;
    for row in mat.iter() {
        let mut count_unknown = 0;
        let mut unknown_idx = None;
        for (idx, c) in row[0..row.len()-1].iter().enumerate() {
            if known_values[idx].is_none() && *c != 0 {
                count_unknown += 1;
                unknown_idx = Some(idx);
            }
        }
        if count_unknown > 0 && count_unknown < selected_count_unknown {
            selected_btn = unknown_idx;
            selected_count_unknown = count_unknown;
        }
    }

    if selected_btn.is_none() {
        panic!("Expected selected btn to be Some");
    }

    let mut min_result = i64::MAX;
    for val in 0..bounds[selected_btn.unwrap()]+1 {
        let mut t_known = known_values.clone();
        t_known[selected_btn.unwrap()] = Some(val);
        if let Some(rs) = search(mat, &t_known, bounds, result_sum) {
            if rs < min_result {
                min_result = rs;
            }
        }
    }
    return Some(min_result);
}

fn get_known_values(mat: &Vec<Vec<i64>>, known: &Vec<Option<i64>>) -> Option<Vec<Option<i64>>> {
    let mut new_known = known.clone();
    for row in mat.iter() {
        let target = row[row.len()-1];
        let mut sum_known = 0;
        let mut unknown_idx = None;
        let mut count_unknown = 0;
        for ri in 0..row.len()-1 {
            let c = row[ri];
            if let Some(m) = new_known[ri] {
                sum_known += m * c;
            } else {
                if c != 0 {
                    count_unknown += 1;
                    unknown_idx = Some(ri);
                }
            }
        }

        if count_unknown == 0 {
            if sum_known != target {
                return None;
            }
        } else if count_unknown == 1 {
            let rhs = target - sum_known;
            if rhs % row[unknown_idx.unwrap()] != 0 {
                return None;
            }
            let val = rhs / row[unknown_idx.unwrap()];
            if val < 0 {
                return None;
            }
            new_known[unknown_idx.unwrap()] = Some(val);
        }
    }
    Some(new_known)
}

#[test]
fn test_case_2() {
    let input: Vec<& 'static str> = vec![
        "[.##.] (3) (1,3) (2) (2,3) (0,2) (0,1) {3,5,4,7}",
        "[...#.] (0,2,3,4) (2,3) (0,4) (0,1,2) (1,2,3,4) {7,5,12,7,2}",
        "[.###.#] (0,1,2,3,4) (0,3,4) (0,1,2,4,5) (1,2) {10,11,11,5,10,5}",
    ];
    let _input: Vec<& 'static str> = vec![
        "[.##.] (3) (1,3) (2) (2,3) (0,2) (0,1) {3,5,4,7}",
    ];
    let _input = vec![
        "[.##.##] (4,5) (0,5) (2,3) (1,3,5) (0,3,5) (0,2,3,5) (0,1,4) (0,2,4,5) {198,181,22,50,173,65}",
    ];
    let _input = vec![
        "[...#.] (0,2,3,4) (2,3) (0,4) (0,1,2) (1,2,3,4) {7,5,12,7,2}",
    ];
    let result = solve_p2(input.join("\n"));
    println!("result: {result:?}");
    assert!(result == 33)
}
