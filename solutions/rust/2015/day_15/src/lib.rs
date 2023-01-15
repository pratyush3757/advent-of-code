use aoclib::{bail, Context, Result};

pub struct PartOne;
pub struct PartTwo;

static INVALID: &str = "Invalid input";

impl aoclib::Solvable<&str, i64> for PartOne {
    fn solve(input: &str) -> aoclib::Result<i64> {
        let ingredients = parse_input(input)?;
        Ok(calculate_max_score(&ingredients, false))
    }
}

impl aoclib::Solvable<&str, i64> for PartTwo {
    fn solve(input: &str) -> aoclib::Result<i64> {
        let ingredients = parse_input(input)?;
        Ok(calculate_max_score(&ingredients, true))
    }
}

#[derive(Debug)]
struct Ingredient {
    capacity: i64,
    durability: i64,
    flavor: i64,
    texture: i64,
    calories: i64,
}

fn parse_input(input: &str) -> Result<Vec<Ingredient>> {
    input.lines().map(split_line).collect()
}

fn split_line(line: &str) -> Result<Ingredient> {
    let properties = line.split_once(':').context(INVALID)?.1;
    match properties.split_whitespace().collect::<Vec<&str>>()[..] {
        [_, capacity, _, durability, _, flavor, _, texture, _, calories] => Ok(Ingredient {
            capacity: capacity.strip_suffix(',').context(INVALID)?.parse()?,
            durability: durability.strip_suffix(',').context(INVALID)?.parse()?,
            flavor: flavor.strip_suffix(',').context(INVALID)?.parse()?,
            texture: texture.strip_suffix(',').context(INVALID)?.parse()?,
            calories: calories.parse()?,
        }),
        _ => bail!(INVALID),
    }
}

fn calculate_max_score(ingredients: &[Ingredient], is_part_2: bool) -> i64 {
    let mut max_score = 0;
    // let mut proportions: [i64; 4] = [0; 4];
    for i in 0..=100 {
        for j in 0..=100 - i {
            for k in 0..=100 - i - j {
                let l = 100 - i - j - k;
                let calories = i * ingredients[0].calories
                    + j * ingredients[1].calories
                    + k * ingredients[2].calories
                    + l * ingredients[3].calories;
                if !is_part_2 || calories == 500 {
                    let tot_capacity: i64 = i * ingredients[0].capacity
                        + j * ingredients[1].capacity
                        + k * ingredients[2].capacity
                        + l * ingredients[3].capacity;
                    let tot_durability: i64 = i * ingredients[0].durability
                        + j * ingredients[1].durability
                        + k * ingredients[2].durability
                        + l * ingredients[3].durability;
                    let tot_flavor: i64 = i * ingredients[0].flavor
                        + j * ingredients[1].flavor
                        + k * ingredients[2].flavor
                        + l * ingredients[3].flavor;
                    let tot_texture: i64 = i * ingredients[0].texture
                        + j * ingredients[1].texture
                        + k * ingredients[2].texture
                        + l * ingredients[3].texture;
                    let total_score = {
                        if tot_capacity < 0
                            || tot_durability < 0
                            || tot_flavor < 0
                            || tot_texture < 0
                        {
                            0
                        } else {
                            tot_capacity * tot_durability * tot_flavor * tot_texture
                        }
                    };
                    max_score = std::cmp::max(total_score, max_score);
                    /*                     if total_score > max_score {
                        max_score = total_score;
                        proportions = [i, j, k, l];
                    } */
                }
            }
        }
    }
    // dbg!(proportions);
    max_score
}

#[cfg(test)]
mod tests {
    use super::{PartOne, PartTwo};
    use aoclib::Solvable;
    use test_case::test_case;

    #[test_case("part_two_input", 0; "sample_1")]
    fn aoc_2015_15_part_one_samples(input: &str, result: i64) {
        assert_eq!(PartOne::solve(input).unwrap(), result);
    }

    #[test_case("part_two_input", 1; "sample_1")]
    fn aoc_2015_15_part_two_samples(input: &str, result: i64) {
        assert_eq!(PartTwo::solve(input).unwrap(), result);
    }
}
