use aoclib::{Context, Result};
use std::collections::{HashMap, HashSet};

pub struct PartOne;
pub struct PartTwo;

impl aoclib::Solvable<&str, usize> for PartOne {
    fn solve(input: &str) -> aoclib::Result<usize> {
        let (molecule, replacements) = parse_input(input)?;
        let mut res = HashSet::<String>::new();
        for (key, val) in replacements {
            for x in &replace_one_by_one(&molecule, key, &val) {
                res.insert(x.clone());
            }
        }
        Ok(res.len())
    }
}

impl aoclib::Solvable<&str, usize> for PartTwo {
    fn solve(input: &str) -> aoclib::Result<usize> {
        Ok(0)
    }
}

fn replace_one_by_one(molecule: &str, from: &str, to: &[&str]) -> Vec<String> {
    let mut res = Vec::<String>::new();
    let match_indices: Vec<_> = molecule.match_indices(from).collect();
    for (idx, _) in match_indices {
        let (first, last) = molecule.split_at(idx);
        for value in to {
            res.push(format!("{first}{}", last.replacen(from, value, 1)));
        }
    }
    res
}

fn parse_input(input: &str) -> Result<(String, HashMap<&str, Vec<&str>>)> {
    let molecule: String = input.lines().rev().take(1).collect();
    let mut replacements: HashMap<&str, Vec<&str>> = HashMap::new();
    for line in input.lines().rev().skip(2) {
        let (key, val) = split_line(line)?;
        replacements.entry(key).or_default().push(val);
    }

    Ok((molecule, replacements))
}

fn split_line(line: &str) -> Result<(&str, &str)> {
    line.split_once(" => ")
        .context(format!("Invalid input: {line}"))
}

#[cfg(test)]
mod tests {
    use super::{PartOne, PartTwo};
    use aoclib::Solvable;
    use test_case::test_case;

    #[test_case("part_two_input", 0; "sample_1")]
    fn aoc_2015_19_part_one_samples(input: &str, result: usize) {
        assert_eq!(PartOne::solve(input).unwrap(), result);
    }

    #[test_case("part_two_input", 1; "sample_1")]
    fn aoc_2015_19_part_two_samples(input: &str, result: usize) {
        assert_eq!(PartTwo::solve(input).unwrap(), result);
    }
}
