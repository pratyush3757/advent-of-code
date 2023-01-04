fn lookandsay(input_sequence: &str) -> String {
    let mut output_sequence: String = String::new();
    let mut run_char = input_sequence.chars().next().unwrap();
    let mut run_length = 1;
    for curr_char in input_sequence.chars().skip(1) {
        if curr_char == run_char {
            run_length += 1;
        } else {
            output_sequence.push_str(&format!("{run_length}{run_char}"));
            run_length = 1;
            run_char = curr_char;
        }
    }
    output_sequence.push_str(&format!("{run_length}{run_char}"));
    output_sequence
}

fn repeat_lookandsay(repeation_times: u32, input_sequence: &str) -> usize {
    let mut output_sequence = input_sequence.to_string();
    for _ in 0..repeation_times {
        output_sequence = lookandsay(&output_sequence);
    }

    output_sequence.len()
}

fn main() {
    let input_str: &str = "1113222113";
    println!("{}", repeat_lookandsay(40, input_str));
    println!("{}", repeat_lookandsay(50, input_str));
}
