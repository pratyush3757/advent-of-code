pub struct PartOne;
pub struct PartTwo;

impl aoclib::Solvable<&str, i32> for PartOne {
    fn solve(input: &str) -> aoclib::Solution<i32> {
        Ok(input.chars().fold(0, |acc, ch| {
            acc + match ch {
                '(' => 1,
                ')' => -1,
                _ => 0,
            }
        }))
    }
}

impl aoclib::Solvable<&str, i32> for PartTwo {
    fn solve(input: &str) -> aoclib::Solution<i32> {
        let mut rolling_sum = 0;
        for (pos, ch) in std::iter::zip(1.., input.chars()) {
            rolling_sum += match ch {
                '(' => 1,
                _ => -1,
            };
            if rolling_sum < 0 {
                return Ok(pos);
            }
        }

        Err("Not found".into())
    }
}

#[cfg(test)]
mod tests {
    use super::{PartOne, PartTwo};
    use aoclib::Solvable;
    use test_case::test_case;

    #[test_case("(())", 0; "sample_1")]
    #[test_case("()()", 0; "sample_2")]
    #[test_case("(((", 3; "sample_3")]
    #[test_case("(()(()(", 3; "sample_4")]
    #[test_case("))(((((", 3; "sample_5")]
    #[test_case("())", -1; "sample_6")]
    #[test_case("))(", -1; "sample_7")]
    #[test_case(")))", -3; "sample_8")]
    #[test_case(")())())", -3; "sample_9")]
    fn aoc_2015_01_part_one_samples(input: &str, result: i32) {
        assert_eq!(PartOne::solve(input).unwrap(), result);
    }

    #[test_case(")", 1; "sample_1")]
    #[test_case("()())", 5; "sample_2")]
    fn aoc_2015_01_part_two_samples(input: &str, result: i32) {
        assert_eq!(PartTwo::solve(input).unwrap(), result);
    }
}
