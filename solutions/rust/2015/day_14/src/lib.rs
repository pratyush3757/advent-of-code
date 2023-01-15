use aoclib::{Context, Result};

pub struct PartOne;
pub struct PartTwo;

impl aoclib::Solvable<&str, u16> for PartOne {
    fn solve(input: &str) -> aoclib::Result<u16> {
        part_one_general(input, 2503)
    }
}

impl aoclib::Solvable<&str, u16> for PartTwo {
    fn solve(input: &str) -> aoclib::Result<u16> {
        part_two_general(input, 2503)
    }
}

fn part_one_general(input: &str, finish_time: u16) -> Result<u16> {
    input
        .lines()
        .map(|line| {
            let deer = split_line(line);
            calculate_distance_traveled(&deer, finish_time)
        })
        .max()
        .context("Max not found")
}

fn part_two_general(input: &str, finish_time: u16) -> Result<u16> {
    let deers: Vec<Deer> = input.lines().map(split_line).collect();
    let mut points = vec![0; deers.len()];
    let mut deer_distance = vec![0; deers.len()];
    let mut lead_distance = 0;
    for i in 1..=finish_time {
        for (idx, deer) in deers.iter().enumerate() {
            let dist = calculate_distance_traveled(deer, i);
            deer_distance[idx] = dist;
            if dist > lead_distance {
                lead_distance = dist;
            }
        }
        for idx in 0..deers.len() {
            if deer_distance[idx] == lead_distance {
                points[idx] += 1;
            }
        }
    }
    points.into_iter().max().context("Max not found")
}

#[derive(Debug)]
struct Deer {
    speed: u16,
    fly_time: u16,
    rest_time: u16,
}

fn split_line(input: &str) -> Deer {
    let divided_line: Vec<&str> = input.split_whitespace().collect();
    match &divided_line[..] {
        [_, _, _, speed, _, _, fly_time, _, _, _, _, _, _, rest_time, _] => Deer {
            speed: speed.parse().unwrap(),
            fly_time: fly_time.parse().unwrap(),
            rest_time: rest_time.parse().unwrap(),
        },
        _ => unreachable!("No other variation in input."),
    }
}

fn calculate_distance_traveled(deer: &Deer, finish_time: u16) -> u16 {
    let full_sprints = finish_time / (deer.fly_time + deer.rest_time);
    let partial_sprint_time = finish_time % (deer.fly_time + deer.rest_time);
    let partial_fly_time = std::cmp::min(partial_sprint_time, deer.fly_time);

    deer.speed * ((full_sprints * deer.fly_time) + (partial_fly_time))
}

#[cfg(test)]
mod tests {
    use super::*;

    static SAMPLE_INPUT: &str = "Comet can fly 14 km/s for 10 seconds, \
        but then must rest for 127 seconds.\n\
        Dancer can fly 16 km/s for 11 seconds, but then must rest for 162 seconds.";

    #[test]
    fn aoc_2015_14_part_one_sample_at_1000() {
        assert_eq!(part_one_general(SAMPLE_INPUT, 1000).unwrap(), 1120);
        assert_eq!(
            calculate_distance_traveled(
                &Deer {
                    speed: 16,
                    fly_time: 11,
                    rest_time: 162
                },
                1000
            ),
            1056
        );
    }

    #[test]
    fn aoc_2015_14_part_two_sample_at_1000() {
        assert_eq!(part_two_general(SAMPLE_INPUT, 1000).unwrap(), 689);
    }
}
