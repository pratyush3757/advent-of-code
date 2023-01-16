use aoclib::{bail, Result};
pub struct PartOne;
pub struct PartTwo;

impl aoclib::Solvable<&str, i16> for PartOne {
    fn solve(input: &str) -> aoclib::Result<i16> {
        let sue_vec = parse_input(input)?;
        if let Some(matching_sue) = sue_vec.iter().find(|x| x.equals(&ORIG_SUE)) {
            Ok(matching_sue.number)
        } else {
            bail!("No match found")
        }
    }
}

impl aoclib::Solvable<&str, i16> for PartTwo {
    fn solve(input: &str) -> aoclib::Result<i16> {
        let sue_vec = parse_input(input)?;
        if let Some(matching_sue) = sue_vec.iter().find(|x| x.equals_ranged(&ORIG_SUE)) {
            Ok(matching_sue.number)
        } else {
            bail!("No match found")
        }
    }
}

fn parse_input(input: &str) -> Result<Vec<Sue>> {
    input.lines().map(get_sue).collect()
}

fn get_sue(line: &str) -> Result<Sue> {
    let mut sue_n = Sue {
        number: -1,
        children: -1,
        cats: -1,
        samoyeds: -1,
        pomeranians: -1,
        akitas: -1,
        vizslas: -1,
        goldfish: -1,
        trees: -1,
        cars: -1,
        perfumes: -1,
    };
    let split_line: Vec<&str> = line.split_whitespace().collect();
    for word_idx in 0..split_line.len() {
        match split_line.get(word_idx) {
            Some(&"Sue") => {
                sue_n.number = split_line[word_idx + 1]
                    .strip_suffix(':')
                    .unwrap()
                    .parse()?;
            }
            Some(&"children:") => {
                sue_n.children = split_line[word_idx + 1].trim_end_matches(',').parse()?;
            }
            Some(&"cats:") => {
                sue_n.cats = split_line[word_idx + 1].trim_end_matches(',').parse()?;
            }
            Some(&"samoyeds:") => {
                sue_n.samoyeds = split_line[word_idx + 1].trim_end_matches(',').parse()?;
            }
            Some(&"pomeranians:") => {
                sue_n.pomeranians = split_line[word_idx + 1].trim_end_matches(',').parse()?;
            }
            Some(&"akitas:") => {
                sue_n.akitas = split_line[word_idx + 1].trim_end_matches(',').parse()?;
            }
            Some(&"vizslas:") => {
                sue_n.vizslas = split_line[word_idx + 1].trim_end_matches(',').parse()?;
            }
            Some(&"goldfish:") => {
                sue_n.goldfish = split_line[word_idx + 1].trim_end_matches(',').parse()?;
            }
            Some(&"trees:") => {
                sue_n.trees = split_line[word_idx + 1].trim_end_matches(',').parse()?;
            }
            Some(&"cars:") => {
                sue_n.cars = split_line[word_idx + 1].trim_end_matches(',').parse()?;
            }
            Some(&"perfumes:") => {
                sue_n.perfumes = split_line[word_idx + 1].trim_end_matches(',').parse()?;
            }
            _ => (),
        }
    }
    Ok(sue_n)
}

#[derive(Debug)]
struct Sue {
    number: i16,
    children: i16,
    cats: i16,
    samoyeds: i16,
    pomeranians: i16,
    akitas: i16,
    vizslas: i16,
    goldfish: i16,
    trees: i16,
    cars: i16,
    perfumes: i16,
}

static ORIG_SUE: Sue = Sue {
    number: 0,
    children: 3,
    cats: 7,
    samoyeds: 2,
    pomeranians: 3,
    akitas: 0,
    vizslas: 0,
    goldfish: 5,
    trees: 3,
    cars: 2,
    perfumes: 1,
};

impl Sue {
    fn equals(&self, other: &Self) -> bool {
        // Negative value means unknown value. They should be treated as true.
        // Or more specifically: "NOT false".
        // Equality check fail with unknown value does not prove that values aren't equal.
        (self.children < 0 || other.children < 0 || self.children == other.children)
            && (self.cats < 0 || other.cats < 0 || self.cats == other.cats)
            && (self.samoyeds < 0 || other.samoyeds < 0 || self.samoyeds == other.samoyeds)
            && (self.pomeranians < 0
                || other.pomeranians < 0
                || self.pomeranians == other.pomeranians)
            && (self.akitas < 0 || other.akitas < 0 || self.akitas == other.akitas)
            && (self.vizslas < 0 || other.vizslas < 0 || self.vizslas == other.vizslas)
            && (self.goldfish < 0 || other.goldfish < 0 || self.goldfish == other.goldfish)
            && (self.trees < 0 || other.trees < 0 || self.trees == other.trees)
            && (self.cars < 0 || other.cars < 0 || self.cars == other.cars)
            && (self.perfumes < 0 || other.perfumes < 0 || self.perfumes == other.perfumes)
    }

    fn equals_ranged(&self, other: &Self) -> bool {
        // Negative values treated same as in equals().
        // Ranges apply only on self.
        (self.children < 0 || other.children < 0 || self.children == other.children)
            && (self.cats < 0 || other.cats < 0 || self.cats > other.cats)
            && (self.samoyeds < 0 || other.samoyeds < 0 || self.samoyeds == other.samoyeds)
            && (self.pomeranians < 0
                || other.pomeranians < 0
                || self.pomeranians < other.pomeranians)
            && (self.akitas < 0 || other.akitas < 0 || self.akitas == other.akitas)
            && (self.vizslas < 0 || other.vizslas < 0 || self.vizslas == other.vizslas)
            && (self.goldfish < 0 || other.goldfish < 0 || self.goldfish < other.goldfish)
            && (self.trees < 0 || other.trees < 0 || self.trees > other.trees)
            && (self.cars < 0 || other.cars < 0 || self.cars == other.cars)
            && (self.perfumes < 0 || other.perfumes < 0 || self.perfumes == other.perfumes)
    }
}

#[cfg(test)]
mod tests {
    use super::Sue;

    #[test]
    fn aoc_2015_16_sue_equals() {
        assert!(Sue {
            number: 0,
            children: 0,
            cats: 5,
            samoyeds: 7,
            pomeranians: 0,
            akitas: 0,
            vizslas: 0,
            goldfish: 3,
            trees: 10,
            cars: 0,
            perfumes: 0
        }
        .equals(&Sue {
            number: 1,
            children: -1,
            cats: -1,
            samoyeds: 7,
            pomeranians: 0,
            akitas: 0,
            vizslas: -1,
            goldfish: 3,
            trees: 10,
            cars: -1,
            perfumes: -1
        }));
    }

    #[test]
    fn aoc_2015_16_sue_equals_ranged() {
        assert!(Sue {
            number: 0,
            children: 0,
            cats: 5,
            samoyeds: 7,
            pomeranians: 4,
            akitas: 0,
            vizslas: 0,
            goldfish: 3,
            trees: 10,
            cars: 0,
            perfumes: 0
        }
        .equals_ranged(&Sue {
            number: 1,
            children: -1,
            cats: 8,
            samoyeds: 7,
            pomeranians: 0,
            akitas: 0,
            vizslas: -1,
            goldfish: 2,
            trees: 11,
            cars: -1,
            perfumes: -1
        }));
    }
}
