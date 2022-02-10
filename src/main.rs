use crate::lib::{Direction, Mars, Pos, Turtle};
use std::io;
use std::io::{BufRead, StdinLock};

mod lib;

fn read_pos(mut stdin: StdinLock) -> Result<Pos, ()> {
    let mut line = String::new();
    stdin.read_line(&mut line).map_err(|_| ())?;
    line.parse::<Pos>().map_err(|_| ())
}

fn read_turtle(mut stdin: StdinLock) -> Result<Turtle, ()> {
    let mut line = String::new();
    stdin.read_line(&mut line).map_err(|_| ())?;
    line.parse::<Turtle>()
}

fn read_directions(mut stdin: StdinLock) -> Result<Vec<Direction>, ()> {
    let mut res = String::new();
    stdin.read_line(&mut res).map_err(|_| ())?;
    if res.is_empty() {
        Err(())
    } else {
        Ok(Direction::from_string(&res))
    }
}

fn move_turtle(mars: &mut Mars) -> Result<(), ()> {
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
