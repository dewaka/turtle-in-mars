use crate::lib::{AppError, Direction, Mars};
use std::io;
use std::str::FromStr;

mod lib;

fn read<T, E>() -> Result<T, E>
where
    T: FromStr<Err = E>,
    E: From<AppError>,
{
    let stdin = io::stdin();
    let mut line = String::new();
    stdin.read_line(&mut line).map_err(|_| AppError::IOError)?;
    line.parse::<T>()
}

fn read_directions() -> Result<Vec<Direction>, AppError> {
    let stdin = io::stdin();
    let mut res = String::new();
    stdin.read_line(&mut res).map_err(|_| AppError::IOError)?;
    if res.is_empty() {
        Err(AppError::InvalidDirection(format!(
            "error reading direction"
        )))
    } else {
        Ok(Direction::from_string(&res))
    }
}

fn move_turtle(mars: &mut Mars) -> Result<(), AppError> {
    let turtle = read()?;
    let dirs: Vec<Direction> = read_directions()?;
    Ok(mars.move_turtle(turtle, &dirs))
}

fn main() {
    let upper_bound = read();
    if upper_bound.is_err() {
        return;
    }

    let mut mars = Mars::new(upper_bound.unwrap());
    loop {
        let ok = move_turtle(&mut mars);
        if ok.is_err() {
            break;
        }
    }
    mars.report();
}
