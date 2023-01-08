pub use anyhow::{anyhow, bail, Context, Error, Result};
use std::path::{Path, PathBuf};
use std::{env, fs};

pub trait Solvable<T, R> {
    fn solve(input: T) -> Result<R>;
}

/**
 * Reads the input file for the day.
 */
pub fn reader(year: &'static str, day: &'static str, file_name: &'static str) -> Result<String> {
    fs::read_to_string(path_resolve(year, day, file_name)?.as_path()).map_err(Error::msg)
}

fn find_nearest_directory_named(directory_name: &str) -> Result<String> {
    let target_directory = Path::new(directory_name);
    let exe_path = env::current_exe()?;
    let mut path = exe_path.as_path();
    while !path.join(target_directory).exists() {
        path = path.parent().context("Parent Not found")?;
    }
    return Ok(path
        .join(target_directory)
        .to_str()
        .context("Cannot join path")?
        .to_string());
}

fn path_resolve(year: &'static str, day: &'static str, file_name: &'static str) -> Result<PathBuf> {
    let input_dir_path = find_nearest_directory_named("input")?;

    Ok(PathBuf::from(format!(
        "{input_dir_path}/{year}/day_{day}/{file_name}",
        input_dir_path = input_dir_path,
        year = year,
        day = day,
        file_name = file_name
    )))
}
