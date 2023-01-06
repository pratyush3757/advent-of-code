use aoc_2015_09::{PartOne, PartTwo};
use aoclib::Solvable;

pub fn main() -> aoclib::Result<()> {
    let input = aoclib::reader(2015, 09, "input.txt")?;
    let result_part_one = PartOne::solve(&input)?; // 0, ~0μs
    let result_part_two = PartTwo::solve(&input)?; // 0, ~0μs

    println!(
        "Results for 2015 Day 09:\n\tPart One: {:?}\n\tPart Two: {:?}",
        result_part_one, result_part_two
    );
    Ok(())
}

#[cfg(test)]
mod tests {
    use aoc_2015_09;
    use aoclib::Solvable;

    fn reader() -> String {
        aoclib::reader(2015, 09, "input.txt").unwrap()
    }

    #[test]
    #[ignore]
    fn aoc_2015_09_part_one() {
        assert_eq!(aoc_2015_09::PartOne::solve(&reader()).unwrap(), 251);
    }

    #[test]
    #[ignore]
    fn aoc_2015_09_part_two() {
        assert_eq!(aoc_2015_09::PartTwo::solve(&reader()).unwrap(), 898);
    }
}
