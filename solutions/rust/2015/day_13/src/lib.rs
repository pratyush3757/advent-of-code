use itertools::Itertools;
use std::collections::{HashMap, HashSet};

pub struct PartOne;
pub struct PartTwo;

impl aoclib::Solvable<&str, i32> for PartOne {
    fn solve(input: &str) -> aoclib::Result<i32> {
        let (happiness_table, people) = populate_happiness_table(input);
        Ok(people
            .iter()
            .permutations(people.len())
            .map(|x| happiness_table.calculate_route_happiness(&x))
            .max()
            .unwrap())
    }
}

impl aoclib::Solvable<&str, i32> for PartTwo {
    fn solve(input: &str) -> aoclib::Result<i32> {
        let (mut happiness_table, mut people) = populate_happiness_table(input);
        let neutral = "You";
        people.iter().for_each(|person| {
            happiness_table.record_happiness(person, neutral, 0);
            happiness_table.record_happiness(neutral, person, 0);
        });
        people.insert(neutral.to_string());

        Ok(people
            .iter()
            .permutations(people.len())
            .map(|x| happiness_table.calculate_route_happiness(&x))
            .max()
            .unwrap())
    }
}

#[derive(Debug)]
struct HappinessTable(HashMap<String, i32>);

impl core::ops::Deref for HappinessTable {
    type Target = HashMap<String, i32>;

    fn deref(&'_ self) -> &'_ Self::Target {
        &self.0
    }
}
impl core::ops::DerefMut for HappinessTable {
    fn deref_mut(&'_ mut self) -> &'_ mut Self::Target {
        &mut self.0
    }
}

impl HappinessTable {
    fn record_happiness(&mut self, person_a: &str, person_b: &str, happiness: i32) {
        self.insert(format!("{person_a} {person_b}"), happiness);
    }

    fn get_happiness(&self, person_a: &str, person_b: &str) -> i32 {
        *self.get(&format!("{person_a} {person_b}")).unwrap()
    }

    fn calculate_route_happiness(&self, route: &[&String]) -> i32 {
        let first_person = route[0];
        let last_person = route[route.len() - 1];
        route
            .windows(2)
            .map(|x| self.get_happiness(x[0], x[1]) + self.get_happiness(x[1], x[0]))
            .sum::<i32>()
            + self.get_happiness(first_person, last_person)
            + self.get_happiness(last_person, first_person)
    }
}

fn split_line(line: &str) -> (&str, &str, i32) {
    let divided_line: Vec<&str> = line.split_whitespace().collect();
    match &divided_line[..] {
        [person_a, _, "gain", x, _, _, _, _, _, _, person_b] => (
            person_a,
            person_b.strip_suffix('.').unwrap(),
            x.parse::<i32>().unwrap(),
        ),
        [person_a, _, "lose", x, _, _, _, _, _, _, person_b] => (
            person_a,
            person_b.strip_suffix('.').unwrap(),
            -x.parse::<i32>().unwrap(),
        ),
        _ => unreachable!("No more variations in the input."),
    }
}

fn populate_happiness_table(input: &str) -> (HappinessTable, HashSet<String>) {
    let mut happiness_table = HappinessTable(HashMap::new());
    let mut people: HashSet<String> = HashSet::new();
    for line in input.lines() {
        let (person_a, person_b, happiness) = split_line(line.trim());
        happiness_table.record_happiness(person_a, person_b, happiness);
        people.insert(person_a.to_string());
        people.insert(person_b.to_string());
    }

    (happiness_table, people)
}

#[cfg(test)]
mod tests {
    use super::PartOne;
    use aoclib::Solvable;

    static SAMPLE_INPUT: &str = "Alice would gain 54 happiness units by sitting next to Bob.\n\
        Alice would lose 79 happiness units by sitting next to Carol.\n\
        Alice would lose 2 happiness units by sitting next to David.\n\
        Bob would gain 83 happiness units by sitting next to Alice.\n\
        Bob would lose 7 happiness units by sitting next to Carol.\n\
        Bob would lose 63 happiness units by sitting next to David.\n\
        Carol would lose 62 happiness units by sitting next to Alice.\n\
        Carol would gain 60 happiness units by sitting next to Bob.\n\
        Carol would gain 55 happiness units by sitting next to David.\n\
        David would gain 46 happiness units by sitting next to Alice.\n\
        David would lose 7 happiness units by sitting next to Bob.\n\
        David would gain 41 happiness units by sitting next to Carol.";

    #[test]
    fn aoc_2015_13_part_one_sample() {
        assert_eq!(PartOne::solve(SAMPLE_INPUT).unwrap(), 330);
    }
}
