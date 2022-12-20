use advent_2015::file_input;
use fancy_regex::Regex;
use lazy_static::lazy_static;

lazy_static! {
    static ref BACKSLASH_ESCAPED: Regex = Regex::new(r"\\\\").unwrap();
    static ref QUOTE_ESCAPED: Regex = Regex::new(r#"\\""#).unwrap();
    static ref UNICODE_CHAR_RAW: Regex = Regex::new(r"\\x..").unwrap();
    static ref QUOTE_UNESCAPED: Regex = Regex::new(r#"""#).unwrap();
    static ref BACKSLASH_UNESCAPED: Regex = Regex::new(r"\\").unwrap();
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

fn part_1(lines: &Vec<String>) {
    let mut raw_length = 0;
    let mut escaped_length = 0;
    for line in lines {
        raw_length += line.trim().len();
        escaped_length += shorten(line.trim()).len();
    }
    let diff = raw_length - escaped_length;
    println!("Raw: {raw_length}, Escaped: {escaped_length}, Diff: {diff}");
}

fn part_2(lines: &Vec<String>) {
    let mut raw_length = 0;
    let mut expanded_length = 0;
    for line in lines {
        raw_length += line.trim().len();
        expanded_length += expand(line.trim()).len() + 2;
    }
    let diff = expanded_length - raw_length;
    println!("Raw: {raw_length}, Expanded: {expanded_length}, Diff: {diff}");
}

fn main() {
    let lines = file_input::read_input_file();
    part_1(&lines);
    part_2(&lines);
}
