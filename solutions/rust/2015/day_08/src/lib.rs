use fancy_regex::Regex;
use lazy_static::lazy_static;

pub struct PartOne;
pub struct PartTwo;

lazy_static! {
    static ref BACKSLASH_ESCAPED: Regex = Regex::new(r"\\\\").unwrap();
    static ref QUOTE_ESCAPED: Regex = Regex::new(r#"\\""#).unwrap();
    static ref UNICODE_CHAR_RAW: Regex = Regex::new(r"\\x..").unwrap();
    static ref QUOTE_UNESCAPED: Regex = Regex::new(r#"""#).unwrap();
    static ref BACKSLASH_UNESCAPED: Regex = Regex::new(r"\\").unwrap();
}

impl aoclib::Solvable<&str, usize> for PartOne {
    fn solve(input: &str) -> aoclib::Result<usize> {
        let mut raw_length = 0;
        let mut escaped_length = 0;
        for line in input.lines() {
            raw_length += line.trim().len();
            escaped_length += shorten(line.trim()).len();
        }
        let diff = raw_length - escaped_length;

        Ok(diff)
    }
}

impl aoclib::Solvable<&str, usize> for PartTwo {
    fn solve(input: &str) -> aoclib::Result<usize> {
        let mut raw_length = 0;
        let mut expanded_length = 0;
        for line in input.lines() {
            raw_length += line.trim().len();
            expanded_length += expand(line.trim()).len() + 2;
        }
        let diff = expanded_length - raw_length;
        Ok(diff)
    }
}

fn shorten(line: &str) -> String {
    let replace_with_dot = ".";
    let changing_text = BACKSLASH_ESCAPED.replace_all(line, replace_with_dot);
    let changing_text = QUOTE_ESCAPED.replace_all(&changing_text, replace_with_dot);
    let changing_text = UNICODE_CHAR_RAW.replace_all(&changing_text, replace_with_dot);
    let changing_text = QUOTE_UNESCAPED.replace_all(&changing_text, "");

    changing_text.to_string()
}

fn expand(line: &str) -> String {
    let changing_text = BACKSLASH_UNESCAPED.replace_all(line, r"\\");
    let changing_text = QUOTE_UNESCAPED.replace_all(&changing_text, r#"\""#);

    changing_text.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_case::test_case;

    #[test_case(r#""""#, r#""#; "empty quotes")]
    #[test_case(r#""abc""#, r#"abc"#; "abc")]
    #[test_case(r#""aaa\"aaa""#, r#"aaa.aaa"#; "escaped quote")]
    #[test_case(r#""\x27""#, r#"."#)]
    fn aoc_2015_08_part_one_samples(input: &str, result: &str) {
        assert_eq!(shorten(input), result);
    }

    #[test_case(r#""""#, r#"\"\""#; "empty quotes")]
    #[test_case(r#""abc""#, r#"\"abc\""#; "abc")]
    #[test_case(r#""aaa\"aaa""#, r#"\"aaa\\\"aaa\""#; "escaped quote")]
    #[test_case(r#""\x27""#, r#"\"\\x27\""#)]
    fn aoc_2015_08_part_two_samples(input: &str, result: &str) {
        assert_eq!(expand(input), result);
    }
}
