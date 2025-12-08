use std::fs;

pub fn solve() {
    let contents = fs::read_to_string("inputs/08.txt").expect("WHERE IS THE FILE");
    println!("part 1: {}", solve_p1(contents.clone(), true));
    println!("part 2: {}", solve_p2(contents.clone()));
}

use std::collections::HashMap;

type Pos3 = (i64,i64,i64);

fn parse_line(line: &str) -> Pos3 {
    let coords: Vec<i64> = line.split(",").map(|c| {
        c.trim().parse().unwrap()
    }).collect();
    (coords[0], coords[1], coords[2])
}

fn pos_dist(a: Pos3, b: Pos3) -> i64 {
    ((a.0-b.0).pow(2) + (a.1-b.1).pow(2) + (a.2-b.2).pow(2)).isqrt()
}

fn parse_input(contents: String) -> (HashMap<Pos3, HashMap<Pos3, i64>>, Vec<(Pos3, Pos3, i64)>) {
    let mut map: HashMap<Pos3, HashMap<Pos3, i64>> = HashMap::new();
    let mut edges: Vec<(Pos3, Pos3, i64)> = vec![];
    contents.lines().for_each(|line| {
        let curr = parse_line(line);
        let mut curr_map = HashMap::new();
        let curr_keys: Vec<Pos3> = map.keys().map(|k| k.clone() ).collect();
        for k in curr_keys.into_iter() {
            // compute distance to every other point
            // store distance in map[k][curr]
            let dist = pos_dist(curr, k);
            if let Some(nbors) = map.get_mut(&k) {
                nbors.insert(curr, dist);
            } else {
                unreachable!("cant find key in map that we pulled from map? {:?}", k);
            }
            curr_map.insert(k, dist);
            edges.push((curr, k, dist));
        }
        map.insert(curr, curr_map);
    });
    (map, edges)
}

use std::collections::HashSet;

fn solve_p1(contents: String, non_test: bool) -> i64 {
    let (_map, mut edges) = parse_input(contents);
    edges.sort_by(|a, b| {
        a.2.cmp(&b.2)
    });
    let mut clusters: Vec<HashSet<Pos3>> = vec![];
    let mut clustered: HashMap<Pos3, usize> = HashMap::new();

    let mut e_idx = 0;
    while e_idx < if non_test { 1000 } else { 10 } {
        // put the edge in a cluster
        let (a, b, _) = edges[e_idx];
        let a_c = clustered.get(&a);
        let b_c = clustered.get(&b);
        match (a_c, b_c) {
            (Some(&a_c), Some(&b_c)) => {
                if a_c != b_c {
                    // always just merge b in to a.
                    let b_cluster = clusters[b_c].clone();
                    for b_pos in b_cluster.into_iter() {
                        // put them into clusters[*a_c]
                        // update their idx in clustered to *a_c
                        clusters[a_c].insert(b_pos);
                        clustered.insert(b_pos, a_c);
                    }
                    clusters[b_c] = HashSet::new();
                }
            },
            (None, Some(b_c)) => {
                // a goes in clusters[b_c]
                clusters[*b_c].insert(a);
                clustered.insert(a, *b_c);
            },
            (Some(a_c), None) => {
                clusters[*a_c].insert(b);
                clustered.insert(b, *a_c);
            },
            (None, None) => {
                clusters.push(vec![a, b].into_iter().collect());
                clustered.insert(a, clusters.len()-1);
                clustered.insert(b, clusters.len()-1);
            },
        }
        e_idx += 1;
    }

    clusters.sort_by(|a, b| {
        b.len().cmp(&a.len())
    });

    clusters[0..3].iter().fold(1, |x, acc| x * acc.len() as i64)
}

#[test]
fn test_case_1() {
    let input: Vec<& 'static str> = vec![
        "162,817,812",
        "57,618,57",
        "906,360,560",
        "592,479,940",
        "352,342,300",
        "466,668,158",
        "542,29,236",
        "431,825,988",
        "739,650,466",
        "52,470,668",
        "216,146,977",
        "819,987,18",
        "117,168,530",
        "805,96,715",
        "346,949,466",
        "970,615,88",
        "941,993,340",
        "862,61,35",
        "984,92,344",
        "425,690,689",
    ];
    let result = solve_p1(input.join("\n"), false);
    println!("result: {result:?}");
    assert!(result == 40)
}

fn solve_p2(contents: String) -> i64 {
    let (map, mut edges) = parse_input(contents);
    edges.sort_by(|a, b| {
        a.2.cmp(&b.2)
    });
    let mut nodes: HashSet<Pos3> = map.into_keys().collect();
    let mut clusters: Vec<HashSet<Pos3>> = vec![];
    let mut clustered: HashMap<Pos3, usize> = HashMap::new();

    let mut e_idx = 0;
    while e_idx < edges.len() {
        // put the edge in a cluster
        let (a, b, _) = edges[e_idx];
        let a_c = clustered.get(&a);
        let b_c = clustered.get(&b);
        match (a_c, b_c) {
            (Some(&a_c), Some(&b_c)) => {
                if a_c != b_c {
                    // always just merge b in to a.
                    let b_cluster = clusters[b_c].clone();
                    for b_pos in b_cluster.into_iter() {
                        // put them into clusters[*a_c]
                        // update their idx in clustered to *a_c
                        clusters[a_c].insert(b_pos);
                        clustered.insert(b_pos, a_c);
                    }
                    clusters[b_c] = HashSet::new();
                }
            },
            (None, Some(b_c)) => {
                // a goes in clusters[b_c]
                clusters[*b_c].insert(a);
                clustered.insert(a, *b_c);
                nodes.remove(&a);
            },
            (Some(a_c), None) => {
                clusters[*a_c].insert(b);
                clustered.insert(b, *a_c);
                nodes.remove(&b);
            },
            (None, None) => {
                clusters.push(vec![a, b].into_iter().collect());
                clustered.insert(a, clusters.len()-1);
                clustered.insert(b, clusters.len()-1);
                nodes.remove(&a);
                nodes.remove(&b);
            },
        }
        if clusters.iter().filter(|c| !c.is_empty()).collect::<Vec<_>>().len() == 1 && nodes.is_empty() {
            // if we only have one group and all nodes have been used, we're done
            break;
        }
        e_idx += 1;
    }

    edges[e_idx].0.0 * edges[e_idx].1.0
}

#[test]
fn test_case_2() {
    let input: Vec<& 'static str> = vec![
        "162,817,812",
        "57,618,57",
        "906,360,560",
        "592,479,940",
        "352,342,300",
        "466,668,158",
        "542,29,236",
        "431,825,988",
        "739,650,466",
        "52,470,668",
        "216,146,977",
        "819,987,18",
        "117,168,530",
        "805,96,715",
        "346,949,466",
        "970,615,88",
        "941,993,340",
        "862,61,35",
        "984,92,344",
        "425,690,689",
    ];
    let result = solve_p2(input.join("\n"));
    println!("result: {result:?}");
    assert!(result == 25272)
}
