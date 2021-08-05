pub fn day5_1(polymer: &str) -> usize {
    let mut react_polymer = vec![];

    for curr in polymer.chars() {
        if react_polymer.is_empty() {
            react_polymer.push(curr);
        } else {
            let prev = *react_polymer.last().unwrap();
            if prev.to_ascii_uppercase() == curr.to_ascii_uppercase() && prev != curr {
                react_polymer.pop();
            } else {
                react_polymer.push(curr);
            }
        }
    }

    react_polymer.len()
}

pub fn day5_2(polymer: &str) -> usize {
    let alphabet = (b'A'..b'Z').map(|c| c as char).collect::<Vec<char>>();
    let mut smallest_length = polymer.len();

    for letter in alphabet {
        let mut react_polymer = vec![];
        for curr in polymer.chars() {
            if curr.to_ascii_uppercase() == letter {
                continue;
            } else if react_polymer.is_empty() {
                react_polymer.push(curr);
            } else {
                let prev = *react_polymer.last().unwrap();
                if prev.to_ascii_uppercase() == curr.to_ascii_uppercase() && prev != curr {
                    react_polymer.pop();
                } else {
                    react_polymer.push(curr);
                }
            }
        }

        if react_polymer.len() < smallest_length {
            smallest_length = react_polymer.len();
        }
    }

    smallest_length
}
