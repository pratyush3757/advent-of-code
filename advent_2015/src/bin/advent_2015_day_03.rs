use advent_2015::file_input;
use std::collections::HashSet;

fn part_1(input_str: &str) -> usize {
    let mut pos_x = 0;
    let mut pos_y = 0;
    let mut visited_houses = HashSet::new();

    for direction in input_str.chars() {
        match direction {
            '^' => pos_y += 1,
            'v' => pos_y -= 1,
            '>' => pos_x += 1,
            '<' => pos_x -= 1,
            _ => (),
        };

        visited_houses.insert((pos_x, pos_y));
    }

    visited_houses.len()
}

fn part_2(input_str: &str) -> usize {
    let mut pos_santa_x = 0;
    let mut pos_santa_y = 0;
    let mut pos_robo_x = 0;
    let mut pos_robo_y = 0;
    let mut visited_houses = HashSet::new();

    for (turn, direction) in input_str.chars().enumerate() {
        let mut increment_y = 0;
        let mut increment_x = 0;
        match direction {
            '^' => increment_y += 1,
            'v' => increment_y -= 1,
            '>' => increment_x += 1,
            '<' => increment_x -= 1,
            _ => (),
        };

        if turn % 2 == 0 {
            pos_santa_x += increment_x;
            pos_santa_y += increment_y;
            visited_houses.insert((pos_santa_x, pos_santa_y));
        } else {
            pos_robo_x += increment_x;
            pos_robo_y += increment_y;
            visited_houses.insert((pos_robo_x, pos_robo_y));
        }
    }

    visited_houses.len()
}

fn main() {
    let lines = file_input::read_input_file();
    let input_str: &str = &lines[0];
    println!("{}", part_1(input_str));
    println!("{}", part_2(input_str));
}
