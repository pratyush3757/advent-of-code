use std::io::{self, BufRead};
use std::path::PathBuf;
use std::{env, fs};

fn get_input_path() -> Option<PathBuf> {
    let path = env::current_exe().expect("Cannot get executable path");
    let input_file_name = path.file_name()?.to_str()?.to_owned();
    let mut ancestors = path.ancestors().nth(3)?.to_path_buf();
    ancestors.push(format!("advent_2015/src/input/{input_file_name}.txt"));
    Some(ancestors)
}

#[must_use]
pub fn read_input_file() -> Vec<String> {
    let input_file_path = get_input_path().expect("Error getting the input file path");
    println!("Input File: {}", input_file_path.display());

    let file = fs::File::open(input_file_path).expect("Cannot Open File");
    let line_reader = io::BufReader::new(file).lines();
    let lines: Vec<String> = line_reader
        .collect::<Result<_, std::io::Error>>()
        .expect("Error during reading the file");
    lines
}
