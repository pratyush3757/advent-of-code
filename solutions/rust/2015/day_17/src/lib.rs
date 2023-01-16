use aoclib::{bail, Context, Result};
use itertools::Itertools;

pub struct PartOne;
pub struct PartTwo;

impl aoclib::Solvable<&str, usize> for PartOne {
    fn solve(input: &str) -> aoclib::Result<usize> {
        part_one_general(input, 150)
    }
}

impl aoclib::Solvable<&str, usize> for PartTwo {
    fn solve(input: &str) -> aoclib::Result<usize> {
        part_two_general(input, 150)
    }
}

fn part_one_general(input: &str, total: u16) -> Result<usize> {
    let containers = parse_input(input)?;
    let mut running_total = 0;
    for i in 0..containers.len() {
        running_total += count_ways_to_fill_upto(&containers, i, total);
    }
    Ok(running_total)
}

fn part_two_general(input: &str, total: u16) -> Result<usize> {
    let containers = parse_input(input)?;
    for i in 0..containers.len() {
        let count = count_ways_to_fill_upto(&containers, i, total);
        if count > 0 {
            return Ok(count);
        }
    }
    bail!("Not found")
}

fn count_ways_to_fill_upto(containers: &[u16], using: usize, total: u16) -> usize {
    containers
        .iter()
        /*
         * Copy so that combinations aren't Vec of refs, which yielded &&u16, but a
         * vec of integers, which yields &u16 (supported by sum)
         * See: https://users.rust-lang.org/t/solved-summing-a-vector-of-refs/52169
         * */
        .copied()
        .combinations(using)
        .filter(|x| x.iter().sum::<u16>() == total)
        .count()
}

fn parse_input(input: &str) -> Result<Vec<u16>> {
    input
        .lines()
        .map(|line| line.parse::<u16>().context("Invalid Input"))
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    static SAMPLE_INPUT: &str = "20\n\
        15\n\
        10\n\
        5\n\
        5";

    #[test]
    fn aoc_2015_17_part_one_samples() {
        assert_eq!(part_one_general(SAMPLE_INPUT, 25).unwrap(), 4);
    }

    #[test]
    fn aoc_2015_17_part_two_samples() {
        assert_eq!(part_two_general(SAMPLE_INPUT, 25).unwrap(), 3);
    }
}
