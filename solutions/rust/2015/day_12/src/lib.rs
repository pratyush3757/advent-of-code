pub struct PartOne;
pub struct PartTwo;
use serde_json::{Map, Value};

#[derive(Debug, PartialEq)]
enum IgnoreRed {
    Yes,
    No,
}

impl aoclib::Solvable<&str, i64> for PartOne {
    fn solve(input: &str) -> aoclib::Result<i64> {
        let input_object: Value = serde_json::from_str(input)?;

        Ok(parse_and_sum(&input_object, &IgnoreRed::No))
    }
}

impl aoclib::Solvable<&str, i64> for PartTwo {
    fn solve(input: &str) -> aoclib::Result<i64> {
        let input_object: Value = serde_json::from_str(input)?;

        Ok(parse_and_sum(&input_object, &IgnoreRed::Yes))
    }
}

fn parse_and_sum(obj: &Value, red_flag: &IgnoreRed) -> i64 {
    match obj {
        Value::Null | Value::Bool(_) | Value::String(_) => 0,
        Value::Number(x) => x.as_i64().expect("Invalid i64"),
        Value::Array(x) => sum_array(x, red_flag),
        Value::Object(x) => sum_object(x, red_flag),
    }
}

fn sum_array(arr: &[Value], red_flag: &IgnoreRed) -> i64 {
    arr.iter().map(|val| parse_and_sum(val, red_flag)).sum()
}

fn sum_object(obj_map: &Map<String, Value>, red_flag: &IgnoreRed) -> i64 {
    match red_flag {
        IgnoreRed::No => obj_map
            .iter()
            .map(|(_key, val)| parse_and_sum(val, red_flag))
            .sum(),
        IgnoreRed::Yes => {
            let mut rolling_sum = 0;
            for (_key, val) in obj_map.iter() {
                if let Value::String(x) = val {
                    if x == "red" {
                        return 0;
                    }
                } else {
                    rolling_sum += parse_and_sum(val, red_flag);
                }
            }

            rolling_sum
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{PartOne, PartTwo};
    use aoclib::Solvable;
    use test_case::test_case;

    #[test_case("[1,2,3]", 6; "sample_1")]
    #[test_case(r#"{"a":2,"b":4}"#, 6; "sample_2")]
    #[test_case("[[[3]]]", 3; "sample_3")]
    #[test_case(r#"{"a":{"b":4},"c":-1}"#, 3; "sample_4")]
    #[test_case(r#"{"a":[-1,1]}"#, 0; "sample_5")]
    #[test_case(r#"[-1,{"a":1}]"#, 0; "sample_6")]
    #[test_case("[]", 0; "sample_7")]
    #[test_case("{}", 0; "sample_8")]
    fn aoc_2015_12_part_one_samples(input: &str, result: i64) {
        assert_eq!(PartOne::solve(input).unwrap(), result);
    }

    #[test_case("[1,2,3]", 6; "sample_1")]
    #[test_case(r#"[1,{"c":"red","b":2},3]"#, 4; "sample_2")]
    #[test_case(r#"{"d":"red","e":[1,2,3,4],"f":5}"#, 0; "sample_3")]
    #[test_case(r#"[1,"red",5]"#, 6; "sample_4")]
    fn aoc_2015_12_part_two_samples(input: &str, result: i64) {
        assert_eq!(PartTwo::solve(input).unwrap(), result);
    }
}
