pub struct PartOne;
pub struct PartTwo;

impl aoclib::Solvable<&str, u16> for PartOne {
    fn solve(input: &str) -> aoclib::Result<u16> {
        Ok(input
            .lines()
            .map(|line| {
                let (_, speed, fly_time, rest_time) = split_line(line);
                calculate_distance_traveled(speed, fly_time, rest_time, 2503)
            })
            .max()
            .unwrap())
    }
}

impl aoclib::Solvable<&str, u16> for PartTwo {
    fn solve(input: &str) -> aoclib::Result<u16> {
        Ok(0)
    }
}

fn split_line(input: &str) -> (&str, u16, u16, u16) {
    let divided_line: Vec<&str> = input.split_whitespace().collect();
    match &divided_line[..] {
        [deer, _, _, speed, _, _, fly_time, _, _, _, _, _, _, rest_time, _] => (
            deer,
            speed.parse().unwrap(),
            fly_time.parse().unwrap(),
            rest_time.parse().unwrap(),
        ),
        _ => unreachable!("No other variation in input."),
    }
}

fn calculate_distance_traveled(speed: u16, fly_time: u16, rest_time: u16, finish_time: u16) -> u16 {
    let full_sprints = finish_time / (fly_time + rest_time);
    let partial_sprint_time = finish_time % (fly_time + rest_time);
    let partial_fly_time = std::cmp::min(partial_sprint_time, fly_time);

    speed * ((full_sprints * fly_time) + (partial_fly_time))
}

#[cfg(test)]
mod tests {
    use super::calculate_distance_traveled;

    /*    static _SAMPLE_INPUT: &str = "Comet can fly 14 km/s for 10 seconds, \
    but then must rest for 127 seconds.\n\
    Dancer can fly 16 km/s for 11 seconds, but then must rest for 162 seconds."; */

    #[test]
    fn aoc_2015_14_given_sample_distance() {
        assert_eq!(calculate_distance_traveled(14, 10, 127, 1000), 1120);
        assert_eq!(calculate_distance_traveled(16, 11, 162, 1000), 1056);
    }
}
