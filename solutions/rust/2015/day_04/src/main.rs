use aoc_2015_04::{PartOne, PartTwo};
use aoclib::Solvable;

pub fn main() -> aoclib::Result<()> {
    let input = aoclib::reader(2015, 04, "input.txt")?;
    let result_part_one = PartOne::solve(&input)?; // 0, ~0μs
    let result_part_two = PartTwo::solve(&input)?; // 0, ~0μs

    println!(
        "Results for 2015 Day 04:\n\tPart One: {:?}\n\tPart Two: {:?}",
        result_part_one, result_part_two
    );
    Ok(())
}

#[cfg(test)]
mod tests {
    use aoc_2015_04;
    use aoclib::Solvable;

    fn reader() -> String {
        aoclib::reader(2015, 04, "input.txt").unwrap()
    }

    #[test]
    fn aoc_2015_04_part_one() {
        assert_eq!(aoc_2015_04::PartOne::solve(&reader()).unwrap(), 282749);
    }

    #[test]
    fn aoc_2015_04_part_two() {
        assert_eq!(aoc_2015_04::PartTwo::solve(&reader()).unwrap(), 9962624);
    }
}
