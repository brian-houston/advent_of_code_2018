pub fn day8_1(line: &str) -> i32 {
    let mut iter = line.split_whitespace().map(|n| n.parse::<i32>().unwrap());
    sum_entries_1(&mut iter)
}

fn sum_entries_1(iter: &mut impl Iterator<Item = i32>) -> i32 {
    let mut total = 0;
    let num_children = iter.next().unwrap();
    let num_entries = iter.next().unwrap();

    for _ in 0..num_children {
        total += sum_entries_1(iter);
    }

    for _ in 0..num_entries {
        total += iter.next().unwrap();
    }

    total
}

pub fn day8_2(line: &str) -> i32 {
    let mut iter = line.split_whitespace().map(|n| n.parse::<i32>().unwrap());
    sum_entries_2(&mut iter)
}

fn sum_entries_2(iter: &mut impl Iterator<Item = i32>) -> i32 {
    let mut total = 0;
    let mut children_sums = vec![];

    let num_children = iter.next().unwrap();
    let num_entries = iter.next().unwrap();

    for _ in 0..num_children {
        children_sums.push(sum_entries_2(iter));
    }

    for _ in 0..num_entries {
        let val = iter.next().unwrap();
        if num_children == 0 {
            total += val;
        } else {
            total += *children_sums.get((val - 1) as usize).unwrap_or(&0);
        }
    }

    total
}
