use md5;

fn part_1(input_str: &str) -> usize {
    let mut result = 1;
    for suffix in 1.. {
        let digest = md5::compute(format!("{input_str}{suffix}"));
        if digest[..2] == [0, 0] && digest[2] <= 0x0F {
            result = suffix;
            break;
        }
    }
    result
}

fn part_2(input_str: &str) -> usize {
    let mut result = 1;
    for suffix in 1.. {
        let digest = md5::compute(format!("{input_str}{suffix}"));
        if digest[..3] == [0, 0, 0] {
            result = suffix;
            break;
        }
    }
    result
}

fn main() {
    let input_str: &str = "yzbqklnj";
    println!("{}", part_1(input_str));
    println!("{}", part_2(input_str));
}
