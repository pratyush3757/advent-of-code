pub struct PartOne;
pub struct PartTwo;

impl aoclib::Solvable<&str, i32> for PartOne {
    fn solve(input: &str) -> aoclib::Solution<i32> {
        Ok(input
            .chars()
            .map(|ch| match ch {
                '(' => 1,
                ')' => -1,
                _ => 0,
            })
            .sum())
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
