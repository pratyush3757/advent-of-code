use advent_2015::file_input;
use std::collections::HashMap;

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

    fn solve(&self, operands: &[&str], operator: &str) -> u16 {
        let operands_val = operands
            .iter()
            .map(|x| self.get_val(x))
            .collect::<Vec<u16>>();

        if operands_val.len() == 1 && operator.is_empty() {
            return operands_val[0];
        }

        let mask = u16::MAX;
        match operator {
            "NOT" => !operands_val[0],
            "AND" => operands_val[0] & operands_val[1],
            "OR" => operands_val[0] | operands_val[1],
            "XOR" => operands_val[0] ^ operands_val[1],
            "LSHIFT" => (operands_val[0] << operands_val[1]) & mask,
            "RSHIFT" => (operands_val[0] >> operands_val[1]) & mask,
            _ => unreachable!("All operators accounted for"),
        }
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

fn parse_connections(lines: &Vec<String>) -> (KnownWires, UnknownWires) {
    let mut known_wires: KnownWires = KnownWires(HashMap::new());
    let mut unknown_wires: UnknownWires = UnknownWires(HashMap::new());

    for line in lines {
        let lhs_rhs = line
            .trim()
            .split("->")
            .map(str::trim)
            .collect::<Vec<&str>>();
        let (incoming, outgoing) = (lhs_rhs[0], lhs_rhs[1]);
        let (operands, operator) = split_lhs(incoming);

        if operands.iter().all(|&x| known_wires.is_resolvable(x)) {
            let num = known_wires.solve(&operands, operator);
            known_wires.insert(outgoing.to_string(), num);
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
                let num = known_wires.solve(&operands, operator);
                known_wires.insert(key.to_string(), num);
                resolved_keys.push(key.clone());
            }
        }
        for x in &resolved_keys {
            unknown_wires.remove(x);
        }
    }

    known_wires
}

fn part_1(lines: &Vec<String>) -> u16 {
    let (known_wires, unknown_wires) = parse_connections(lines);
    let known_wires = resolve_unknown_wires(known_wires, unknown_wires);
    if let Some(value) = known_wires.get("a") {
        println!("Part 1: {}", value);

        return *value;
    }
    unreachable!("Will always have a value");
}

fn part_2(lines: &Vec<String>, wire_a_val: u16) -> u16 {
    let (mut known_wires, unknown_wires) = parse_connections(lines);
    known_wires.insert("b".to_string(), wire_a_val);
    let known_wires = resolve_unknown_wires(known_wires, unknown_wires);
    if let Some(value) = known_wires.get("a") {
        println!("Part 2: {}", value);

        return *value;
    }
    unreachable!("Will always have a value");
}

fn main() {
    let lines = file_input::read_input_file();
    let wire_a_val = part_1(&lines);
    println!("{}", wire_a_val);
    println!("{}", part_2(&lines, wire_a_val));
}
