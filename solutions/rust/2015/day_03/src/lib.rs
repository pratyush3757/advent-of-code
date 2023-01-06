use std::collections::HashSet;

pub struct PartOne;
pub struct PartTwo;

impl aoclib::Solvable<&str, usize> for PartOne {
    fn solve(input: &str) -> aoclib::Solution<usize> {
        let mut pos_x = 0;
        let mut pos_y = 0;
        let mut visited_houses = HashSet::new();
        visited_houses.insert((0,0));

        for direction in input.chars() {
            match direction {
                '^' => pos_y += 1,
                'v' => pos_y -= 1,
                '>' => pos_x += 1,
                '<' => pos_x -= 1,
                _ => (),
            };

            visited_houses.insert((pos_x, pos_y));
        }

        Ok(visited_houses.len())
    }
}

impl aoclib::Solvable<&str, usize> for PartTwo {
    fn solve(input: &str) -> aoclib::Solution<usize> {
        let mut pos_santa_x = 0;
        let mut pos_santa_y = 0;
        let mut pos_robo_x = 0;
        let mut pos_robo_y = 0;
        let mut visited_houses = HashSet::new();
        visited_houses.insert((0,0));

        for (turn, direction) in input.chars().enumerate() {
            let mut increment_y = 0;
            let mut increment_x = 0;
            match direction {
                '^' => increment_y += 1,
                'v' => increment_y -= 1,
                '>' => increment_x += 1,
                '<' => increment_x -= 1,
                _ => (),
            };

            if turn % 2 == 0 {
                pos_santa_x += increment_x;
                pos_santa_y += increment_y;
                visited_houses.insert((pos_santa_x, pos_santa_y));
            } else {
                pos_robo_x += increment_x;
                pos_robo_y += increment_y;
                visited_houses.insert((pos_robo_x, pos_robo_y));
            }
        }

        Ok(visited_houses.len())
    }
}

#[cfg(test)]
mod tests {
    use super::{PartOne, PartTwo};
    use aoclib::Solvable;
    use test_case::test_case;

    #[test_case(">", 2; "sample_1")]
    #[test_case("^>v<", 4; "sample_2")]
    #[test_case("^v^v^v^v^v", 2; "sample_3")]
    fn aoc_2015_03_part_one_samples(input: &str, result: usize) {
        assert_eq!(PartOne::solve(input).unwrap(), result);
    }

    #[test_case("^v", 3; "sample_1")]
    #[test_case("^>v<", 3; "sample_2")]
    #[test_case("^v^v^v^v^v", 11; "sample_3")]
    fn aoc_2015_03_part_two_samples(input: &str, result: usize) {
        assert_eq!(PartTwo::solve(input).unwrap(), result);
    }
}
