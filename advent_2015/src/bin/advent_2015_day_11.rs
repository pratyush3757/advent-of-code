fn increment(input_sequence: &mut Vec<u8>, index: usize) {
    if index == 0 && input_sequence[index] == 122 {
        return;
    }
    match input_sequence.get(index).unwrap() {
        97..=121 => {
            input_sequence[index] += 1;
        }
        122 => {
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
            // second check prevents overlapping patterns like 'aaa'
            double_patterns_count += 1;
            last_double_pattern_idx = i;
        }
    }
    if double_patterns_count < 2 {
        return false;
    }

    true
}

fn part1(input_str: &str) -> String {
    let mut output_str: Vec<u8> = input_str.as_bytes().to_owned();

    while !check_if_valid(&output_str) {
        increment(&mut output_str, 7);
    }

    String::from_utf8(output_str).unwrap()
}

fn part2(input_str: &str) -> String {
    let mut output_str: Vec<u8> = input_str.as_bytes().to_owned();
    increment(&mut output_str, 7);

    while !check_if_valid(&output_str) {
        increment(&mut output_str, 7);
    }

    String::from_utf8(output_str).unwrap()
}

fn main() {
    let input_str: &str = "hxbxwxba";
    let part1_res = part1(input_str);
    let part2_res = part2(&part1_res);

    println!("{} {}", part1_res, part2_res);
}
