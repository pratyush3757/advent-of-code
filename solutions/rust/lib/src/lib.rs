pub mod errors;

pub use errors::{AocError, SolutionError, SolverError};
use std::path::{Path, PathBuf};
use std::{env, fs, io};

pub type Result<T> = std::result::Result<T, AocError>;
pub type Solution<T> = std::result::Result<T, SolutionError>;
pub type SolverResult<T> = std::result::Result<T, SolverError>;

pub trait Solvable<T, R> {
    fn solve(input: T) -> Solution<R>;
}

/**
 * Reads the input file for the day.
 */
pub fn reader(year: u16, day: u16, file_name: &'static str) -> io::Result<String> {
    fs::read_to_string(path_resolve(year, day, file_name)?.as_path())
}

fn find_nearest_directory_named(directory_name: &str) -> io::Result<String> {
    let target_directory = Path::new(directory_name);
    let exe_path = env::current_exe()?;
    let mut path = exe_path.as_path();
    while !path.join(target_directory).exists() {
        path = path.parent().unwrap();
    }
    return Ok(path.join(target_directory).to_str().unwrap().to_string());
}

fn path_resolve(year: u16, day: u16, file_name: &'static str) -> io::Result<PathBuf> {
    let input_dir_path = find_nearest_directory_named("input")?;

    Ok(PathBuf::from(format!(
        "{input_dir_path}/{year}/day_{day:02}/{file_name}",
        input_dir_path = input_dir_path,
        year = year,
        day = day,
        file_name = file_name
    )))
}
