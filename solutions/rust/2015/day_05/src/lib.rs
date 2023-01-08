use fancy_regex::Regex;
use lazy_static::lazy_static;

pub struct PartOne;
pub struct PartTwo;

lazy_static! {
    static ref PATTERN_VOWELS: Regex = Regex::new(r"[aeiou]").unwrap();
    static ref PATTERN_CONSECUTIVE_LETTERS: Regex = Regex::new(r"(.)\1").unwrap();
    static ref PATTERN_FORBIDDEN: Regex = Regex::new(r"(ab)|(cd)|(pq)|(xy)").unwrap();
    static ref PATTERN_REPEATING_TWICE: Regex = Regex::new(r"((\w\w))(?=.*\1)").unwrap();
    static ref PATTERN_LETTER_BETWEEN_SAME: Regex = Regex::new(r"(\w).(\1)").unwrap();
}

impl aoclib::Solvable<&str, usize> for PartOne {
    fn solve(input: &str) -> aoclib::Result<usize> {
        Ok(input.lines().filter(|&line| check_valid_1(line)).count())
    }
}

impl aoclib::Solvable<&str, usize> for PartTwo {
    fn solve(input: &str) -> aoclib::Result<usize> {
        Ok(input.lines().filter(|&line| check_valid_2(line)).count())
    }
}

fn check_valid_1(line: &str) -> bool {
    let res_forbidden = PATTERN_FORBIDDEN.find_iter(line).count();
    if res_forbidden != 0 {
        return false;
    }

    let res_vowels = PATTERN_VOWELS.find_iter(line).count();
    if res_vowels < 3 {
        return false;
    }

    let res_consecutive = PATTERN_CONSECUTIVE_LETTERS.find_iter(line).count();
    if res_consecutive < 1 {
        return false;
    }

    true
}

fn check_valid_2(line: &str) -> bool {
    let res_forbidden = PATTERN_REPEATING_TWICE.find_iter(line).count();
    if res_forbidden == 0 {
        return false;
    }

    let res_vowels = PATTERN_LETTER_BETWEEN_SAME.find_iter(line).count();
    if res_vowels == 0 {
        return false;
    }

    true
}

#[cfg(test)]
mod tests {
    use super::{check_valid_1, check_valid_2};
    use test_case::test_case;

    #[test_case("ugknbfddgicrmopn", true)]
    #[test_case("aaa", true)]
    #[test_case("jchzalrnumimnmhp", false)]
    #[test_case("haegwjzuvuyypxyu", false)]
    #[test_case("dvszwmarrgswjxmb", false)]
    fn aoc_2015_05_part_one_samples(input: &str, result: bool) {
        assert_eq!(check_valid_1(input), result);
    }

    #[test_case("qjhvhtzxzqqjkmpb", true)]
    #[test_case("xxyxx", true)]
    #[test_case("uurcxstgmygtbstg", false)]
    #[test_case("ieodomkazucvgmuy", false)]
    fn aoc_2015_05_part_two_samples(input: &str, result: bool) {
        assert_eq!(check_valid_2(input), result);
    }
}
