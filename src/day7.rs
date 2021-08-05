use std::collections::{HashMap, BinaryHeap};
use std::cmp::Reverse;

struct Edge {
    source: char,
    destination: char,
}

fn parse_line(line: &str) -> Edge {
    let mut line_chars = line.chars();
    let source = line_chars.nth(5).unwrap();
    let destination = line_chars.nth(30).unwrap();

    Edge { source, destination }
}

pub fn day7_1(lines: &[String]) -> String {
    let edges = lines.iter().map(|line| parse_line(&line)).collect::<Vec<Edge>>();
    let mut out_edges_map = HashMap::new();
    let mut in_degree_map = HashMap::new();

    for e in edges {
        let out_edges = out_edges_map.entry(e.source).or_insert(vec![]);
        out_edges.push(e.destination);

        let in_degree = in_degree_map.entry(e.destination).or_insert(0);
        *in_degree += 1;

        out_edges_map.entry(e.destination).or_insert(vec![]);
        in_degree_map.entry(e.source).or_insert(0);
    }

    let mut result = String::from("");
    let mut no_edges_heap = in_degree_map.keys()
        .filter(|n| *in_degree_map.get(n).unwrap() == 0)
        .copied()
        .map(Reverse)
        .collect::<BinaryHeap<_>>();

    while !no_edges_heap.is_empty() {
        let Reverse(curr) = no_edges_heap.pop().unwrap(); 
        result.push(curr);
        for node in out_edges_map[&curr].iter() {
            let in_degree = in_degree_map.get_mut(node).unwrap();
            *in_degree -= 1;
            if *in_degree == 0 {
                no_edges_heap.push(Reverse(*node));
            }
        }
    }

    result
}

pub fn day7_2(lines: &[String]) -> u32 {
    let edges = lines.iter().map(|line| parse_line(&line)).collect::<Vec<Edge>>();
    let mut out_edges_map = HashMap::new();
    let mut in_degree_map = HashMap::new();

    for e in edges {
        let out_edges = out_edges_map.entry(e.source).or_insert(vec![]);
        out_edges.push(e.destination);

        let in_degree = in_degree_map.entry(e.destination).or_insert(0);
        *in_degree += 1;

        out_edges_map.entry(e.destination).or_insert(vec![]);
        in_degree_map.entry(e.source).or_insert(0);
    }

    let mut no_edges_heap = in_degree_map.keys()
        .filter(|n| *in_degree_map.get(n).unwrap() == 0)
        .copied()
        .map(|c| Reverse((0, c)))
        .collect::<BinaryHeap<_>>();

    let mut finish_time = 0;

    while !no_edges_heap.is_empty() {
        let Reverse((time, curr)) = no_edges_heap.pop().unwrap(); 
        let time_spent = curr as u32 - 'A' as u32 + 61;

        for node in out_edges_map[&curr].iter() {
            let in_degree = in_degree_map.get_mut(node).unwrap();
            *in_degree -= 1;
            if *in_degree == 0 {
                no_edges_heap.push(Reverse((time + time_spent, *node)));
            }
        }

        if no_edges_heap.is_empty() {
            finish_time = time + time_spent;
        }
    }

    finish_time
}
