use advent_2015::file_input;

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

fn part_1(lines: &Vec<String>) {
    let mut lights = Blights {
        grid: vec![vec![false; 1000]; 1000],
    };
    for line in lines {
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

    println!(
        "{}",
        lights
            .grid
            .iter()
            .flat_map(|x: &Vec<_>| x.iter())
            .filter(|b| **b)
            .count()
    );
}

fn part_2(lines: &Vec<String>) {
    let mut lights = Nlights {
        grid: vec![vec![0; 1000]; 1000],
    };
    for line in lines {
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

    println!(
        "{}",
        lights
            .grid
            .iter()
            .flat_map(|x: &Vec<_>| x.iter())
            .sum::<i32>()
    );
}

fn main() {
    let lines = file_input::read_input_file();
    part_1(&lines);
    part_2(&lines);
}
