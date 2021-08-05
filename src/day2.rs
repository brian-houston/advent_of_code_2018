use std::collections::HashMap;

pub fn day2_1(lines: &[String]) -> u32 {
    let counts = lines.iter().map(|line| {
        let mut map = HashMap::new();

        for c in line.chars() {
            // count characters in string
            let counter = map.entry(c).or_insert(0);
            *counter += 1; 
        }

        // return tuple (bool, bool)
        // first bool true if map has value of 2
        // second bool true if map has value of 3
        map.values().fold((false, false), |acc, n| (acc.0 | (*n == 2), acc.1 | (*n == 3)))
    }).fold((0, 0), |acc, n| (acc.0 + n.0 as u32, acc.1 + n.1 as u32)); // count number of trues in each tuple position

    counts.0 * counts.1
}

pub fn day2_2(lines: &[String]) -> String {
    for (i, str1) in lines.iter().enumerate() {
        for str2 in lines.iter().skip(i+1) {
            let mut iter2 = str2.chars();
            let result = str1.chars().filter(|c| *c == iter2.next().unwrap()).collect::<String>();
            if result.len() == str1.len() - 1 {
                return result;
            }
        }
    }

    "".to_owned()
}
