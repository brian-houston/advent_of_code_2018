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
        .map(Reverse)
        .collect::<BinaryHeap<_>>();

    let mut in_progress_heap = BinaryHeap::new();
    let mut total_time = 0;

    while !no_edges_heap.is_empty() || ! in_progress_heap.is_empty() {
        let mut curr_time = None;

        while let Some(Reverse((time, label))) = in_progress_heap.peek() {
            let time = *time;
            let label = *label;

            if curr_time.is_none() { 
                curr_time = Some(time); 
                total_time = if time > total_time { time } else { total_time };
            }

            if time != curr_time.unwrap() { break; }

            in_progress_heap.pop();

            for neighbor in out_edges_map[&label].iter() {
                let in_degree = in_degree_map.get_mut(neighbor).unwrap();
                *in_degree -= 1;

                if *in_degree == 0 {
                    no_edges_heap.push(Reverse(*neighbor));
                }
            }
        }

        while in_progress_heap.len() < 5 && !no_edges_heap.is_empty() {
            let Reverse(label) = no_edges_heap.pop().unwrap(); 
            let time_cost = label as u32 - 'A' as u32 + 61;
            let finish_time = curr_time.unwrap_or(0) + time_cost;
            in_progress_heap.push(Reverse((finish_time, label)));
        }
    }

    total_time
}
