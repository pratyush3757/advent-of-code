use md5;

pub struct PartOne;
pub struct PartTwo;

impl aoclib::Solvable<&str, usize> for PartOne {
    fn solve(input: &str) -> aoclib::Result<usize> {
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
    fn solve(input: &str) -> aoclib::Result<usize> {
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

#[cfg(test)]
mod tests {
    use super::PartOne;
    use aoclib::Solvable;
    use test_case::test_case;

    #[test_case("abcdef", 609043;)]
    #[test_case("pqrstuv", 1048970)]
    fn aoc_2015_04_part_one_samples(input: &str, result: usize) {
        assert_eq!(PartOne::solve(input).unwrap(), result);
    }
}
