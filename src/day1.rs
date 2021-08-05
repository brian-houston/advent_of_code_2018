use std::collections::HashSet;

pub fn day1_1(lines: &[i32]) -> i32 {
    lines.iter().fold(0, |acc, n| acc + n)
}

pub fn day1_2(lines: &[i32]) -> i32 {
    let mut prev_freqs = HashSet::new();
    let mut curr_freq = 0;

    for freq in lines.iter().cycle() {
        curr_freq += freq;
        if !prev_freqs.insert(curr_freq) { break; } // insert returns false if element is already in set
    }

    curr_freq
}
