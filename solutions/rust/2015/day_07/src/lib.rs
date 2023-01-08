use std::collections::HashMap;

pub struct PartOne;
pub struct PartTwo;

impl aoclib::Solvable<&str, u16> for PartOne {
    fn solve(input: &str) -> aoclib::Result<u16> {
        let (known_wires, unknown_wires) = parse_connections(input);
        let known_wires = resolve_unknown_wires(known_wires, unknown_wires);
        if let Some(value) = known_wires.get("a") {
            return Ok(*value);
        }
        unreachable!("Will always have a value");
    }
}

impl aoclib::Solvable<&str, u16> for PartTwo {
    fn solve(input: &str) -> aoclib::Result<u16> {
        let (mut known_wires, unknown_wires) = parse_connections(input);
        known_wires.insert("b".to_string(), PartOne::solve(input).unwrap());
        let known_wires = resolve_unknown_wires(known_wires, unknown_wires);
        if let Some(value) = known_wires.get("a") {
            return Ok(*value);
        }
        unreachable!("Will always have a value");
    }
}

#[derive(Debug)]
struct KnownWires(HashMap<String, u16>);

impl core::ops::Deref for KnownWires {
    type Target = HashMap<String, u16>;

    fn deref(&'_ self) -> &'_ Self::Target {
        &self.0
    }
}
impl core::ops::DerefMut for KnownWires {
    fn deref_mut(&'_ mut self) -> &'_ mut Self::Target {
        &mut self.0
    }
}

impl KnownWires {
    fn is_resolvable(&self, x: &str) -> bool {
        self.contains_key(x) || x.parse::<u16>().is_ok()
    }

    fn solve_and_insert(&mut self, key: String, operands: &[&str], operator: &str) {
        let operands_values = operands
            .iter()
            .map(|x| self.get_val(x))
            .collect::<Vec<u16>>();

        let mask = u16::MAX;
        let value = if operands_values.len() == 1 && operator.is_empty() {
            operands_values[0]
        } else {
            match operator {
                "NOT" => !operands_values[0],
                "AND" => operands_values[0] & operands_values[1],
                "OR" => operands_values[0] | operands_values[1],
                "XOR" => operands_values[0] ^ operands_values[1],
                "LSHIFT" => (operands_values[0] << operands_values[1]) & mask,
                "RSHIFT" => (operands_values[0] >> operands_values[1]) & mask,
                _ => unreachable!("All operators accounted for"),
            }
        };

        self.insert(key, value);
    }

    fn get_val(&self, x: &str) -> u16 {
        if let Ok(val) = x.parse::<u16>() {
            return val;
        } else if let Some(val) = self.get(x) {
            return *val;
        }
        unreachable!("Will always have a value");
    }
}

#[derive(Debug)]
struct UnknownWires(HashMap<String, String>);

impl core::ops::Deref for UnknownWires {
    type Target = HashMap<String, String>;

    fn deref(&'_ self) -> &'_ Self::Target {
        &self.0
    }
}
impl core::ops::DerefMut for UnknownWires {
    fn deref_mut(&'_ mut self) -> &'_ mut Self::Target {
        &mut self.0
    }
}

fn split_lhs(lhs: &str) -> (Vec<&str>, &str) {
    match lhs.trim().split(' ').collect::<Vec<&str>>()[..] {
        [wire] => (vec![wire], ""),
        [op, wire] => (vec![wire], op),
        [wire_1, op, wire_2] => (vec![wire_1, wire_2], op),
        _ => unreachable!("No other pattern for lhs"),
    }
}

fn parse_connections(input: &str) -> (KnownWires, UnknownWires) {
    let mut known_wires: KnownWires = KnownWires(HashMap::new());
    let mut unknown_wires: UnknownWires = UnknownWires(HashMap::new());

    for line in input.lines() {
        let lhs_rhs = line.split("->").map(str::trim).collect::<Vec<&str>>();
        let (incoming, outgoing) = (lhs_rhs[0], lhs_rhs[1]);
        let (operands, operator) = split_lhs(incoming);

        if operands.iter().all(|&x| known_wires.is_resolvable(x)) {
            known_wires.solve_and_insert(outgoing.to_string(), &operands, operator);
        } else {
            unknown_wires.insert(outgoing.to_string(), incoming.to_string());
        }
    }

    (known_wires, unknown_wires)
}

fn resolve_unknown_wires(
    mut known_wires: KnownWires,
    mut unknown_wires: UnknownWires,
) -> KnownWires {
    while unknown_wires.len() != 0 {
        if known_wires.contains_key("a") {
            break;
        }

        let mut resolved_keys: Vec<String> = Vec::new();
        for (key, value) in unknown_wires.iter() {
            let (operands, operator) = split_lhs(value);
            if operands.iter().all(|&x| known_wires.is_resolvable(x)) {
                known_wires.solve_and_insert(key.to_string(), &operands, operator);
                resolved_keys.push(key.clone());
            }
        }

        for x in &resolved_keys {
            unknown_wires.remove(x);
        }
    }

    known_wires
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_case::test_case;

    fn resolve_wires(input: &str) -> KnownWires {
        let (known_wires, unknown_wires) = parse_connections(input);
        resolve_unknown_wires(known_wires, unknown_wires)
    }

    static SAMPLE_INPUT: &str = "123 -> x\n\
    456 -> y\n\
    x AND y -> d\n\
    x OR y -> e\n\
    x LSHIFT 2 -> f\n\
    y RSHIFT 2 -> g\n\
    NOT x -> h\n\
    NOT y -> i";

    #[test_case("d", 72)]
    #[test_case("e", 507)]
    #[test_case("f", 492)]
    #[test_case("g", 114)]
    #[test_case("h", 65412)]
    #[test_case("i", 65079)]
    #[test_case("x", 123)]
    #[test_case("y", 456)]
    fn aoc_2015_07_part_one_samples(wire: &str, value: u16) {
        let known_wires = resolve_wires(SAMPLE_INPUT);
        assert_eq!(known_wires.get(wire), Some(&value));
    }
}
