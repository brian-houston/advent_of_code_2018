use std::collections::HashMap;
use std::collections::HashSet;

pub fn day3_1(lines: &[String]) -> u32 {
    let mut map = HashMap::new();
    let mut overlaps = 0;

    for line in lines.iter() {
        let parsed_output = parse_line(&line);

        let left_offset = parsed_output[1];
        let top_offset = parsed_output[2];
        let width = parsed_output[3];
        let height = parsed_output[4];

        for i in left_offset..left_offset+width {
            for j in top_offset..top_offset+height {
                let count = map.entry((i, j)).or_insert(0);
                *count += 1;

                if *count == 2 {
                    overlaps += 1;
                }
            }
        }
    }

    overlaps
}

pub fn day3_2(lines: &[String]) -> u32 {
    let mut map = HashMap::new();
    let mut has_no_overlaps = HashSet::new(); 

    for line in lines.iter() {
        let parsed_output = parse_line(&line);

        let id = parsed_output[0];
        let left_offset = parsed_output[1];
        let top_offset = parsed_output[2];
        let width = parsed_output[3];
        let height = parsed_output[4];

        has_no_overlaps.insert(id);

        for i in left_offset..left_offset+width {
            for j in top_offset..top_offset+height {
                let prev_id = map.entry((i, j)).or_insert(0);

                if *prev_id != 0 {
                    has_no_overlaps.remove(prev_id);
                    has_no_overlaps.remove(&id);
                } else {
                    *prev_id = id;
                }
            }
        }
    }

    *has_no_overlaps.iter().next().unwrap()
}

fn parse_line(line: &str) -> Vec<u32> {
    let mut splits = line.split_whitespace();
    
    let mut id_chars = splits.next().unwrap().chars();
    id_chars.next();
    let id: u32 = id_chars.as_str().parse().unwrap();
    splits.next();

    let mut offset_splits = splits.next().unwrap().split(",");
    let mut size_splits = splits.next().unwrap().split("x");

    let left_offset: u32 = offset_splits.next().unwrap().parse().unwrap();
    let mut top_offset_chars = offset_splits.next().unwrap().chars();
    top_offset_chars.next_back();
    let top_offset: u32 = top_offset_chars.as_str().parse().unwrap();

    let width: u32 = size_splits.next().unwrap().parse().unwrap();
    let height: u32 = size_splits.next().unwrap().parse().unwrap();

    vec![id, left_offset, top_offset, width, height]
}
