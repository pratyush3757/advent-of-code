use aoc_2015_03::{PartOne, PartTwo};
use aoclib::Solvable;

pub fn main() -> aoclib::Result<()> {
    let input = aoclib::reader(2015, 03, "input.txt")?;
    let result_part_one = PartOne::solve(&input)?; // 0, ~0μs
    let result_part_two = PartTwo::solve(&input)?; // 0, ~0μs

    println!(
        "Results for 2015 Day 03:\n\tPart One: {:?}\n\tPart Two: {:?}",
        result_part_one, result_part_two
    );
    Ok(())
}

#[cfg(test)]
mod tests {
    use aoclib::Solvable;

    fn reader() -> String {
        aoclib::reader(2015, 03, "input.txt").unwrap()
    }

    #[test]
    #[ignore]
    fn aoc_2015_03_part_one() {
        assert_eq!(aoc_2015_03::PartOne::solve(&reader()).unwrap(), 2565);
    }

    #[test]
    #[ignore]
    fn aoc_2015_03_part_two() {
        assert_eq!(aoc_2015_03::PartTwo::solve(&reader()).unwrap(), 2639);
    }
}
