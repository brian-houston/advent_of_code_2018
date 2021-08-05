use std::collections::{HashMap, HashSet};

#[derive(Debug, Clone, Copy, Hash, Eq, PartialEq)]
struct Point {
    x: i32,
    y: i32,
}

fn parse_line(line: &str) -> Point {
    let mut split_line = line.split(", ");
    let x: i32 = split_line.next().unwrap().parse().unwrap();
    let y: i32 = split_line.next().unwrap().parse().unwrap();

    Point { x, y }
}

fn distance(a: Point, b: Point) -> i32 {
    (a.x - b.x).abs() + (a.y - b.y).abs()
}

fn find_closest_point(point: Point, candidates: &[Point]) -> Option<Point> {
    let mut closest_point = point;
    let mut closest_distance = 99999;
    let mut is_tie = false;

    for candidate in candidates.iter() {
        let d = distance(point, *candidate);
        if d < closest_distance {
            closest_point = *candidate;
            closest_distance = d;
            is_tie = false
        } else if d == closest_distance {
            is_tie = true;
        }
    }

    if is_tie {
        None
    } else {
        Some(closest_point) 
    }
}

pub fn day6_1(lines: &[String]) -> u32 {
    let points = lines.iter().map(|line| parse_line(&line)).collect::<Vec<Point>>();
    let mut map = HashMap::new();
    let mut on_edge = HashSet::new();

    let start = -500;
    let end = 1000;

    for x in start..end {
        for y in start..end {
            let p = Point { x, y };
            let closest_opt = find_closest_point(p, &points);

            if let Some(closest) = closest_opt {
                let count = map.entry(closest).or_insert(0);
                *count += 1;

                if x == start || y == start || x == end - 1 || y == end -1 {
                    on_edge.insert(closest);
                }
            }

        }
    }

    let max_point = map.keys().filter(|p| !on_edge.contains(p)).max_by_key(|p| *map.get(&p).unwrap()).unwrap();
    *map.get(&max_point).unwrap()
}

pub fn day6_2(lines: &[String]) -> u32 {
    let points = lines.iter().map(|line| parse_line(&line)).collect::<Vec<Point>>();

    let start = -500;
    let end = 1000;
    let mut count = 0;

    for x in start..end {
        for y in start..end {
            let curr = Point { x, y };

            let total_distance: i32 = points.iter().map(|p| distance(*p, curr)).sum();

            if total_distance < 10000 {
                count += 1;
            }
        }
    }

    count
}
