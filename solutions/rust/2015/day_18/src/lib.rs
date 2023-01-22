pub struct PartOne;
pub struct PartTwo;

impl aoclib::Solvable<&str, usize> for PartOne {
    fn solve(input: &str) -> aoclib::Result<usize> {
        Ok(part_one_general(input, 100))
    }
}

impl aoclib::Solvable<&str, usize> for PartTwo {
    fn solve(input: &str) -> aoclib::Result<usize> {
        Ok(part_two_general(input, 100))
    }
}

fn part_one_general(input: &str, steps: usize) -> usize {
    let (mut state, mut last_state) = parse_input(input);
    for _ in 0..steps {
        animate_step(&mut state, &mut last_state);
    }
    state.debug_println();
    state.count_on()
}

fn part_two_general(input: &str, steps: usize) -> usize {
    let (mut state, mut last_state) = parse_input(input);
    for _ in 0..steps {
        state.turn_on_corners();
        animate_step(&mut state, &mut last_state);
    }
    state.debug_println();
    state.turn_on_corners();
    state.count_on()
}

type Pos = (usize, usize);
struct BinLights {
    grid_width: usize,
    grid: Vec<bool>,
    unpadded_width: usize,
}

impl std::ops::Index<Pos> for BinLights {
    type Output = bool;
    fn index(&self, (x, y): Pos) -> &Self::Output {
        &self.grid[x + y * self.grid_width]
    }
}

impl std::ops::IndexMut<Pos> for BinLights {
    fn index_mut(&mut self, (x, y): Pos) -> &mut bool {        
        &mut self.grid[x + y * self.grid_width]
    }
}

impl BinLights {
    fn backup(&mut self, other: &Self) {
        self.grid.copy_from_slice(&other.grid);
    }

    fn new(unpadded_width: usize) -> Self {
        let grid_width = unpadded_width + 2;
        BinLights { grid_width, 
        grid: vec![false; grid_width * grid_width], 
        unpadded_width,
        }
    }

    fn count_on(&self) -> usize {
        self.grid.iter().filter(|b| **b).count()
    }

    fn rows(&mut self) -> impl Iterator<Item = &mut [bool]> {
        // return all rows except first and last, as grid is padded.
        self.grid.chunks_mut(self.grid_width).skip(1).take(self.unpadded_width)
    }

    fn count_neighbours(&self, row: usize, col: usize) -> usize {
        let mut count = 0;
        for i in row - 1..=row + 1 {
            for j in col - 1..=col + 1 {
                count += usize::from(self[(i, j)]);
            }
        }
        count - usize::from(self[(row, col)])
    }

    fn debug_println(&self) {
        self.grid.chunks(self.grid_width)
            .map(|row| {
                row.iter()
                    .map(|ch| if *ch { '#' } else { '.' })
                    .collect::<String>()
            })
            .for_each(|st| println!("{st}"));
    }

    fn turn_on_corners(&mut self) {
        let unpadded_width = self.unpadded_width;
        self[(1, 1)] = true;
        self[(1, unpadded_width)] = true;
        self[(unpadded_width, 1)] = true;
        self[(unpadded_width, unpadded_width)] = true;
    }
}


fn parse_input(input: &str) -> (BinLights, BinLights) {
    let unpadded_width = input.lines().count();
    let mut state = BinLights::new(unpadded_width);
    let last_state = BinLights::new(unpadded_width);
    for (line, row) in input
        .lines()
        .zip(state.rows())
    {
        for (ch, light) in line
            .chars()
            .zip(row.iter_mut().skip(1).take(unpadded_width))
                // taking all bits except first and last as grid is padded.
        {
            match ch {
                '.' => (),
                '#' => *light = true,
                _ => unreachable!("Only '.' and '#' should be in the input"),
            }
        }
    }
    
    (state, last_state)
}

fn animate_step(state: &mut BinLights, last_state: &mut BinLights) {
    last_state.backup(state);
    for i in 1..=state.unpadded_width {
        for j in 1..=state.unpadded_width {
            let count = last_state.count_neighbours(i, j);
            if last_state[(i, j)] && !(count == 2 || count == 3) {
                state[(i, j)] = false;
            }
            if count == 3 {
                state[(i, j)] = true;
            }
        }
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    static SAMPLE_INPUT: &str = ".#.#.#\n\
        ...##.\n\
        #....#\n\
        ..#...\n\
        #.#..#\n\
        ####..";

    static SAMPLE_INPUT_2: &str = "##.#.#\n\
        ...##.\n\
        #....#\n\
        ..#...\n\
        #.#..#\n\
        ####.#";

    #[test]
    fn aoc_2015_18_part_one_samples() {
        assert_eq!(part_one_general(SAMPLE_INPUT, 4), 4);
    }

    #[test]
    fn aoc_2015_18_part_two_samples() {
        assert_eq!(part_two_general(SAMPLE_INPUT_2, 5), 17);
    }
}
