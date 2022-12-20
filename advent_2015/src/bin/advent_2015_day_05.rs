use advent_2015::file_input;
use fancy_regex::Regex;
use lazy_static::lazy_static;

lazy_static! {
    static ref PATTERN_VOWELS: Regex = Regex::new(r"[aeiou]").unwrap();
    static ref PATTERN_CONSECUTIVE_LETTERS: Regex = Regex::new(r"(.)\1").unwrap();
    static ref PATTERN_FORBIDDEN: Regex = Regex::new(r"(ab)|(cd)|(pq)|(xy)").unwrap();
    static ref PATTERN_REPEATING_TWICE: Regex = Regex::new(r"((\w\w))(?=.*\1)").unwrap();
    static ref PATTERN_LETTER_BETWEEN_SAME: Regex = Regex::new(r"(\w).(\1)").unwrap();
}

fn part_1(line: &str) -> bool {
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

fn part_2(line: &str) -> bool {
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

fn main() {
    let lines = file_input::read_input_file();
    // println!("{:#?}", lines);
    let part_1_res = lines.iter().filter(|&line| part_1(line)).count();
    let part_2_res = lines.iter().filter(|&line| part_2(line)).count();
    println!("{}", part_1_res);
    println!("{}", part_2_res);
    // println!("{}", part_2(lines));
}
