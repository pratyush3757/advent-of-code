use advent_2015::file_input;

fn part_1(input_str: &str) -> usize {
    input_str.matches('(').count() - input_str.matches(')').count()
}

fn part_2(input_str: &str) -> usize {
    let mut rolling_sum = 0;
    for (pos, ch) in std::iter::zip(1.., input_str.chars()) {
        rolling_sum += match ch {
            '(' => 1,
            _ => -1,
        };
        if rolling_sum < 0 {
            return pos;
        }
    }
    input_str.len()
}

fn main() {
    let lines = file_input::read_input_file();
    let input_str = &lines[0];
    println!("{}", part_1(input_str));
    println!("{}", part_2(input_str));
}
