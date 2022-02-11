use crate::lib::{AppError, Direction, Mars, Pos, Turtle};
use std::io;
use std::io::{BufRead, StdinLock};

mod lib;

fn read_pos(mut stdin: StdinLock) -> Result<Pos, AppError> {
    let mut line = String::new();
    stdin.read_line(&mut line).map_err(|_| AppError::IOError)?;
    line.parse::<Pos>()
}

fn read_turtle(mut stdin: StdinLock) -> Result<Turtle, AppError> {
    let mut line = String::new();
    stdin.read_line(&mut line).map_err(|_| AppError::IOError)?;
    line.parse::<Turtle>()
}

fn read_directions(mut stdin: StdinLock) -> Result<Vec<Direction>, AppError> {
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
    let stdin = io::stdin();
    let turtle = read_turtle(stdin.lock())?;
    let dirs = read_directions(stdin.lock())?;
    Ok(mars.move_turtle(turtle, &dirs))
}

fn main() {
    let stdin = io::stdin();

    let upper_bound = read_pos(stdin.lock());
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
