pub struct PartOne;
pub struct PartTwo;

impl aoclib::Solvable<&str, u32> for PartOne {
    fn solve(input: &str) -> aoclib::Result<u32> {
        let gifts = to_gifts(input);
        let mut sum_area = 0;
        for entry in gifts {
            let largest_dimension = entry.iter().max().unwrap_or(&0);
            let (l, w, h) = (entry[0], entry[1], entry[2]);

            let wrap_area = 2 * l * w + 2 * l * h + 2 * w * h;
            let extra_area = (l * w * h) / largest_dimension;
            sum_area += wrap_area + extra_area;
        }

        Ok(sum_area)
    }
}

impl aoclib::Solvable<&str, u32> for PartTwo {
    fn solve(input: &str) -> aoclib::Result<u32> {
        let gifts = to_gifts(input);
        let mut sum_ribbon = 0;
        for entry in gifts {
            let largest_dimension = entry.iter().max().unwrap_or(&0);
            let (l, w, h) = (entry[0], entry[1], entry[2]);

            let smallest_face_perimeter = 2 * (l + w + h) - 2 * (largest_dimension);
            let volume = l * w * h;
            sum_ribbon += smallest_face_perimeter + volume;
        }

        Ok(sum_ribbon)
    }
}

fn to_gifts(input: &str) -> Vec<[u32; 3]> {
    input
        .lines()
        .map(|line| {
            let mut num = line.split('x').map(|num| num.parse().unwrap_or(0));
            [
                num.next().unwrap(),
                num.next().unwrap(),
                num.next().unwrap(),
            ]
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::{PartOne, PartTwo};
    use aoclib::Solvable;
    use test_case::test_case;

    #[test_case("2x3x4", 58)]
    #[test_case("1x1x10", 43)]
    fn aoc_2015_02_part_one_samples(input: &str, result: u32) {
        assert_eq!(PartOne::solve(input).unwrap(), result);
    }

    #[test_case("2x3x4", 34)]
    #[test_case("1x1x10", 14)]
    fn aoc_2015_02_part_two_samples(input: &str, result: u32) {
        assert_eq!(PartTwo::solve(input).unwrap(), result);
    }
}
