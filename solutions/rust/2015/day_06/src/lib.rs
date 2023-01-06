pub struct PartOne;
pub struct PartTwo;

impl aoclib::Solvable<&str, usize> for PartOne {
    fn solve(input: &str) -> aoclib::Solution<usize> {
        let mut lights = Blights {
            grid: vec![vec![false; 1000]; 1000],
        };
        for line in input.lines() {
            match line.split(' ').collect::<Vec<&str>>()[..] {
                ["turn", "on", ul, "through", lr] => {
                    lights.turn_on(get_coord_tuple(ul, lr));
                }
                ["turn", "off", ul, "through", lr] => {
                    lights.turn_off(get_coord_tuple(ul, lr));
                }
                ["toggle", ul, "through", lr] => {
                    lights.toggle(get_coord_tuple(ul, lr));
                }
                _ => (),
            }
        }

        Ok(lights
            .grid
            .iter()
            .flat_map(|x: &Vec<_>| x.iter())
            .filter(|b| **b)
            .count())
    }
}

impl aoclib::Solvable<&str, i32> for PartTwo {
    fn solve(input: &str) -> aoclib::Solution<i32> {
        let mut lights = Nlights {
            grid: vec![vec![0; 1000]; 1000],
        };
        for line in input.lines() {
            match line.split(' ').collect::<Vec<&str>>()[..] {
                ["turn", "on", ul, "through", lr] => {
                    lights.turn_bright(get_coord_tuple(ul, lr));
                }
                ["turn", "off", ul, "through", lr] => {
                    lights.turn_dull(get_coord_tuple(ul, lr));
                }
                ["toggle", ul, "through", lr] => {
                    lights.turn_more_bright(get_coord_tuple(ul, lr));
                }
                _ => (),
            }
        }

        Ok(lights
            .grid
            .iter()
            .flat_map(|x: &Vec<_>| x.iter())
            .sum::<i32>())
    }
}

#[derive(Debug)]
struct Blights {
    grid: Vec<Vec<bool>>,
}

impl Blights {
    fn turn_on(&mut self, (ul, lr): (Coord, Coord)) {
        for i in ul.x..=lr.x {
            for j in ul.y..=lr.y {
                self.grid[i as usize][j as usize] = true;
            }
        }
    }
    fn turn_off(&mut self, (ul, lr): (Coord, Coord)) {
        for i in ul.x..=lr.x {
            for j in ul.y..=lr.y {
                self.grid[i as usize][j as usize] = false;
            }
        }
    }
    fn toggle(&mut self, (ul, lr): (Coord, Coord)) {
        for i in ul.x..=lr.x {
            for j in ul.y..=lr.y {
                self.grid[i as usize][j as usize] = !self.grid[i as usize][j as usize];
            }
        }
    }
}

#[derive(Debug)]
struct Nlights {
    grid: Vec<Vec<i32>>,
}

impl Nlights {
    fn turn_bright(&mut self, (ul, lr): (Coord, Coord)) {
        for i in ul.x..=lr.x {
            for j in ul.y..=lr.y {
                self.grid[i as usize][j as usize] += 1;
            }
        }
    }
    fn turn_dull(&mut self, (ul, lr): (Coord, Coord)) {
        for i in ul.x..=lr.x {
            for j in ul.y..=lr.y {
                if self.grid[i as usize][j as usize] != 0 {
                    self.grid[i as usize][j as usize] -= 1;
                }
            }
        }
    }
    fn turn_more_bright(&mut self, (ul, lr): (Coord, Coord)) {
        for i in ul.x..=lr.x {
            for j in ul.y..=lr.y {
                self.grid[i as usize][j as usize] += 2;
            }
        }
    }
}

#[derive(Debug)]
struct Coord {
    x: u32,
    y: u32,
}

fn to_coord(point: &str) -> Coord {
    let x_y_vec: Vec<u32> = point
        .split(',')
        .map(|z| z.parse::<u32>().unwrap())
        .collect();
    let (x, y) = (x_y_vec[0], x_y_vec[1]);
    Coord { x, y }
}

fn get_coord_tuple(ul: &str, lr: &str) -> (Coord, Coord) {
    (to_coord(ul), to_coord(lr))
}

#[cfg(test)]
mod tests {
    use super::{PartOne, PartTwo};
    use aoclib::Solvable;
    use test_case::test_case;

    #[test_case("turn on 0,0 through 999,999", 1000000)]
    #[test_case("toggle 0,0 through 999,0", 1000)]
    #[test_case("turn on 0,0 through 999,999\nturn off 499,499 through 500,500", 1000000-4)]
    fn aoc_2015_06_part_one_samples(input: &str, result: usize) {
        assert_eq!(PartOne::solve(input).unwrap(), result);
    }

    #[test_case("turn on 0,0 through 0,0", 1)]
    #[test_case("toggle 0,0 through 999,999", 2000000)]
    fn aoc_2015_06_part_two_samples(input: &str, result: i32) {
        assert_eq!(PartTwo::solve(input).unwrap(), result);
    }
}
