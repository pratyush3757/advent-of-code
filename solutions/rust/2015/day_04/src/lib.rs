use md5;

pub struct PartOne;
pub struct PartTwo;

impl aoclib::Solvable<&str, usize> for PartOne {
    fn solve(input: &str) -> aoclib::Solution<usize> {
        let input_trimmed = input.trim();
        let mut result = 1;
        for suffix in 1.. {
            let digest = md5::compute(format!("{input_trimmed}{suffix}"));
            if digest[..2] == [0, 0] && digest[2] <= 0x0F {
                result = suffix;
                break;
            }
        }
        Ok(result)
    }
}

impl aoclib::Solvable<&str, usize> for PartTwo {
    fn solve(input: &str) -> aoclib::Solution<usize> {
        let input_trimmed = input.trim();
        let mut result = 1;
        for suffix in 1.. {
            let digest = md5::compute(format!("{input_trimmed}{suffix}"));
            if digest[..3] == [0, 0, 0] {
                result = suffix;
                break;
            }
        }
        Ok(result)
    }
}
