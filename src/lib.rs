#[cfg(test)]
use std::fs::File;
#[cfg(test)]
use std::io::BufReader;
#[cfg(test)]
use std::io::prelude::*;

pub mod day1;
pub mod day2;
pub mod day3;
pub mod day4;
pub mod day5;
pub mod day6;
pub mod day7;

#[cfg(test)]
mod tests {
    use super::*;

    fn get_lines_from_file(file_path: &str) -> Vec<String> {
        let file = File::open(file_path).unwrap();
        let buf_reader = BufReader::new(file);

        buf_reader.lines().map(|line_result| line_result.unwrap()).collect::<Vec<String>>()
    }

    #[test]
    fn day1_1() {
        let lines = get_lines_from_file("inputs/day1.txt");
        let lines = lines.iter().map(|line_string| line_string.parse().unwrap()).collect::<Vec<i32>>();
        let answer = day1::day1_1(&lines);

        assert_eq!(answer, 493);
    }

    #[test]
    fn day1_2() {
        let lines = get_lines_from_file("inputs/day1.txt");
        let lines = lines.iter().map(|line_string| line_string.parse().unwrap()).collect::<Vec<i32>>();
        let answer = day1::day1_2(&lines);

        assert_eq!(answer, 413);
    }

    #[test]
    fn day2_1() {
        let lines = get_lines_from_file("inputs/day2.txt");
        let answer = day2::day2_1(&lines);

        assert_eq!(answer, 8820);
    }

    #[test]
    fn day2_2() {
        let lines = get_lines_from_file("inputs/day2.txt");
        let answer = day2::day2_2(&lines);

        assert_eq!(answer, "bpacnmglhizqygfsjixtkwudr");
    }

    #[test]
    fn day3_1() {
        let lines = get_lines_from_file("inputs/day3.txt");
        let answer = day3::day3_1(&lines);

        assert_eq!(answer, 108961);
    }

    #[test]
    fn day3_2() {
        let lines = get_lines_from_file("inputs/day3.txt");
        let answer = day3::day3_2(&lines);

        assert_eq!(answer, 681);
    }

    #[test]
    fn day4_1() {
        let lines = get_lines_from_file("inputs/day4.txt");
        let answer = day4::day4_1(&lines);

        assert_eq!(answer, 12169);
    }

    #[test]
    fn day4_2() {
        let lines = get_lines_from_file("inputs/day4.txt");
        let answer = day4::day4_2(&lines);

        assert_eq!(answer, 16164);
    }

    #[test]
    fn day5_1() {
        let lines = get_lines_from_file("inputs/day5.txt");
        let answer = day5::day5_1(&lines[0]);

        assert_eq!(answer, 10878);
    }

    #[test]
    fn day5_2() {
        let lines = get_lines_from_file("inputs/day5.txt");
        let answer = day5::day5_2(&lines[0]);

        assert_eq!(answer, 6874);
    }

    #[test]
    fn day6_1() {
        let lines = get_lines_from_file("inputs/day6.txt");
        let answer = day6::day6_1(&lines);

        assert_eq!(answer, 4754);
    }

    #[test]
    fn day6_2() {
        let lines = get_lines_from_file("inputs/day6.txt");
        let answer = day6::day6_2(&lines);

        assert_eq!(answer, 42344);
    }

    #[test]
    fn day7_1() {
        let lines = get_lines_from_file("inputs/day7.txt");
        let answer = day7::day7_1(&lines);

        assert_eq!(answer, "ABLCFNSXZPRHVEGUYKDIMQTWJO");
    }

    #[test]
    fn day7_2() {
        let lines = get_lines_from_file("inputs/day7.txt");
        let answer = day7::day7_2(&lines);

        assert_eq!(answer, 1157);
    }
}
