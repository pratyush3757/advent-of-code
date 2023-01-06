pub struct PartOne;
pub struct PartTwo;

impl aoclib::Solvable<&str, String> for PartOne {
    fn solve(input: &str) -> aoclib::Solution<String> {
        let mut output_str: Vec<u8> = input.trim().as_bytes().to_owned();

        while !check_if_valid(&output_str) {
            increment(&mut output_str, 7);
        }

        Ok(String::from_utf8(output_str).unwrap())
    }
}

impl aoclib::Solvable<&str, String> for PartTwo {
    fn solve(input: &str) -> aoclib::Solution<String> {
        let mut output_str: Vec<u8> = PartOne::solve(input).unwrap().as_bytes().to_owned();
        increment(&mut output_str, 7);

        while !check_if_valid(&output_str) {
            increment(&mut output_str, 7);
        }

        Ok(String::from_utf8(output_str).unwrap())
    }
}

fn increment(input_sequence: &mut Vec<u8>, index: usize) {
    if index == 0 && input_sequence[index] == 122 {
        // z
        // return early, as zzzzzzzz wraps around to aaaaaaaa
        input_sequence[index] = 97;
        return;
    }
    match input_sequence.get(index).unwrap() {
        97..=121 => {
            // a..=y
            input_sequence[index] += 1;
        }
        122 => {
            // z
            input_sequence[index] = 97;
            increment(input_sequence, index - 1);
        }
        _ => panic!("Invalid input sequence"),
    }
}

fn check_if_valid(input_sequence: &[u8]) -> bool {
    let consecutive_run: bool = input_sequence
        .windows(3)
        .any(|triplet| (triplet[0] + 1 == triplet[1]) && (triplet[1] + 1 == triplet[2]));
    if !consecutive_run {
        return false;
    }

    let forbidden_letters_present: bool = input_sequence
        .iter()
        .any(|&ch| ch == 105 || ch == 111 || ch == 108); // equality with [iol] chars
    if forbidden_letters_present {
        return false;
    }

    let mut double_patterns_count = 0;
    let mut last_double_pattern_idx = 9; // set to be out of array range.
    for i in 1..=7 {
        if (input_sequence[i] == input_sequence[i - 1]) && (i - 1 != last_double_pattern_idx) {
            // second condition prevents overlapping patterns like 'aaa'
            double_patterns_count += 1;
            last_double_pattern_idx = i;
        }
    }
    if double_patterns_count < 2 {
        return false;
    }

    true
}

#[cfg(test)]
mod tests {
    use super::{PartOne, check_if_valid};
    use aoclib::Solvable;
    use test_case::test_case;

    #[test_case("hijklmmn", false)]
    #[test_case("abbceffg", false)]
    #[test_case("abbcegjk", false)]
    fn aoc_2015_11_part_one_validity(input: &str, result: bool) {
        let output_str: Vec<u8> = input.trim().as_bytes().to_owned();
        assert_eq!(check_if_valid(&output_str), result);
    }
    
    #[test_case("abcdefgh", "abcdffaa")]
    #[test_case("ghijklmn", "ghjaabcc")]
    fn aoc_2015_11_part_one_next(input: &str, result: &str) {
        assert_eq!(PartOne::solve(input).unwrap(), result);
    }
}
