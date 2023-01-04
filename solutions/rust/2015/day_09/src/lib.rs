use itertools::Itertools;
use std::collections::{HashMap, HashSet};

pub struct PartOne;
pub struct PartTwo;

impl aoclib::Solvable<&str, u32> for PartOne {
    fn solve(input: &str) -> aoclib::Solution<u32> {
        let (distance_table, cities) = populate_distance_table(input);
        Ok(cities
            .iter()
            .permutations(cities.len())
            .map(|x| distance_table.calculate_route_distance(&x))
            .min()
            .unwrap())
    }
}

impl aoclib::Solvable<&str, u32> for PartTwo {
    fn solve(input: &str) -> aoclib::Solution<u32> {
        let (distance_table, cities) = populate_distance_table(input);
        Ok(cities
            .iter()
            .permutations(cities.len())
            .map(|x| distance_table.calculate_route_distance(&x))
            .max()
            .unwrap())
    }
}

#[derive(Debug)]
struct DistanceTable(HashMap<String, u32>);

impl core::ops::Deref for DistanceTable {
    type Target = HashMap<String, u32>;

    fn deref(&'_ self) -> &'_ Self::Target {
        &self.0
    }
}
impl core::ops::DerefMut for DistanceTable {
    fn deref_mut(&'_ mut self) -> &'_ mut Self::Target {
        &mut self.0
    }
}

impl DistanceTable {
    fn record_distance(&mut self, city_a: &str, city_b: &str, distance: u32) {
        self.insert(format!("{city_a} {city_b}"), distance);
        self.insert(format!("{city_b} {city_a}"), distance);
    }

    fn get_distance(&self, city_a: &str, city_b: &str) -> u32 {
        *self.get(&format!("{city_a} {city_b}")).unwrap()
    }

    fn calculate_route_distance(&self, route: &Vec<&String>) -> u32 {
        route
            .windows(2)
            .map(|x| self.get_distance(x[0], x[1]))
            .sum()
    }
}

fn split_line(line: &str) -> (&str, &str, u32) {
    let lhs_rhs = line.split(" = ").collect::<Vec<&str>>();
    let cities = lhs_rhs[0].split(" to ").collect::<Vec<&str>>();
    let distance = lhs_rhs[1].parse::<u32>().unwrap();
    (cities[0], cities[1], distance)
}

fn populate_distance_table(input: &str) -> (DistanceTable, HashSet<String>) {
    let mut distance_table = DistanceTable(HashMap::new());
    let mut cities: HashSet<String> = HashSet::new();
    for line in input.lines() {
        let (city_a, city_b, distance) = split_line(line.trim());
        distance_table.record_distance(city_a, city_b, distance);
        cities.insert(city_a.to_string());
        cities.insert(city_b.to_string());
    }

    (distance_table, cities)
}
