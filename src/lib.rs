use std::collections::HashSet;
use std::fmt;
use std::fmt::Formatter;
use std::num::ParseIntError;
use std::str::FromStr;

#[derive(Debug, Eq, PartialEq, Hash, Copy, Clone)]
pub struct Pos {
    pub x: i32,
    pub y: i32,
}

impl Pos {
    pub fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }
}

impl FromStr for Pos {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let coords: Vec<&str> = s.trim().split(' ').collect();
        let x = coords[0].parse::<i32>()?;
        let y = coords[1].parse::<i32>()?;
        Ok(Pos::new(x, y))
    }
}

#[derive(Debug, Eq, PartialEq)]
pub enum Orientation {
    North,
    South,
    East,
    West,
}

impl FromStr for Orientation {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.trim() {
            "N" => Ok(Orientation::North),
            "S" => Ok(Orientation::South),
            "E" => Ok(Orientation::East),
            "W" => Ok(Orientation::West),
            _ => Err(()),
        }
    }
}

impl fmt::Display for Orientation {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let abbr = match self {
            Orientation::North => "N",
            Orientation::South => "S",
            Orientation::East => "E",
            Orientation::West => "W",
        };

        write!(f, "{}", abbr)
    }
}

impl Orientation {
    fn with_direction(&self, direction: Direction) -> Self {
        match self {
            Orientation::North => {
                if direction == Direction::Left {
                    Orientation::West
                } else if direction == Direction::Right {
                    Orientation::East
                } else {
                    Orientation::North
                }
            }
            Orientation::South => {
                if direction == Direction::Left {
                    Orientation::East
                } else if direction == Direction::Right {
                    Orientation::West
                } else {
                    Orientation::South
                }
            }
            Orientation::East => {
                if direction == Direction::Left {
                    Orientation::North
                } else if direction == Direction::Right {
                    Orientation::South
                } else {
                    Orientation::East
                }
            }
            Orientation::West => {
                if direction == Direction::Left {
                    Orientation::South
                } else if direction == Direction::Right {
                    Orientation::North
                } else {
                    Orientation::West
                }
            }
        }
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum Direction {
    Left,
    Right,
    Forward,
}

impl Direction {
    pub fn from_char(c: char) -> Option<Direction> {
        match c {
            'L' => Some(Direction::Left),
            'R' => Some(Direction::Right),
            'F' => Some(Direction::Forward),
            _ => None,
        }
    }

    pub fn from_string(s: &str) -> Vec<Direction> {
        s.chars().flat_map(|c| Direction::from_char(c)).collect()
    }
}

#[derive(Debug, Eq, PartialEq)]
pub struct Turtle {
    pos: Pos,
    orientation: Orientation,
    lost: bool,
}

impl fmt::Display for Turtle {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{} {} {}", self.pos.x, self.pos.y, self.orientation)
    }
}

impl FromStr for Turtle {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let s: Vec<&str> = s.trim().split(' ').collect();
        if s.len() == 3 {
            let x = s[0].parse::<i32>().map_err(|_| ())?;
            let y = s[1].parse::<i32>().map_err(|_| ())?;
            let o = s[2].parse::<Orientation>().map_err(|_| ())?;
            Ok(Turtle::new(Pos::new(x, y), o))
        } else {
            Err(())
        }
    }
}

impl Turtle {
    pub fn new(pos: Pos, orientation: Orientation) -> Self {
        Self {
            pos,
            orientation,
            lost: false,
        }
    }

    fn make_move(&mut self, mars: &mut Mars, direction: Direction) {
        if !self.lost && mars.in_bounds(self.pos) {
            match direction {
                Direction::Left => self.orientation = self.orientation.with_direction(direction),
                Direction::Right => self.orientation = self.orientation.with_direction(direction),
                Direction::Forward => {
                    let new_pos = self.advanced_pos();
                    if mars.in_bounds(new_pos) {
                        self.pos = new_pos;
                    } else if !mars.scent_seen(new_pos) {
                        // this turtle is lost
                        self.lost = true;
                        // add lost pos to scents to help future turtles
                        mars.add_scent(new_pos);
                    }
                }
            }
        }
    }

    fn advanced_pos(&self) -> Pos {
        match self.orientation {
            Orientation::North => Pos::new(self.pos.x, self.pos.y + 1),
            Orientation::South => Pos::new(self.pos.x, self.pos.y - 1),
            Orientation::East => Pos::new(self.pos.x + 1, self.pos.y),
            Orientation::West => Pos::new(self.pos.x - 1, self.pos.y),
        }
    }
}

#[derive(Debug)]
pub struct Grid {
    lower_left: Pos,
    upper_right: Pos,
}

impl Grid {
    fn in_bounds(&self, pos: Pos) -> bool {
        let lower = self.lower_left;
        let upper = self.upper_right;

        (pos.x >= lower.x && pos.y >= lower.y) && (pos.x <= upper.x && pos.y <= upper.y)
    }
}

#[derive(Debug)]
pub struct Mars {
    grid: Grid,           // Two dimensional grid of Mars
    turtles: Vec<Turtle>, // Turtles who walked on Mars
    scents: HashSet<Pos>, // Scents left by turtles
}

impl Mars {
    pub fn new(pos: Pos) -> Self {
        Self {
            grid: Grid {
                lower_left: Pos::new(0, 0),
                upper_right: pos,
            },
            turtles: vec![],
            scents: HashSet::new(),
        }
    }

    pub fn move_turtle(&mut self, mut turtle: Turtle, direction: &[Direction]) {
        if !self.in_bounds(turtle.pos) {
            return;
        }

        for &dir in direction {
            turtle.make_move(self, dir);
            if turtle.lost {
                break;
            }
        }

        self.turtles.push(turtle);
    }

    pub fn report(&self) {
        for turtle in &self.turtles {
            Mars::print_turtle(turtle);
        }
    }

    fn print_turtle(turtle: &Turtle) {
        if turtle.lost {
            println!(
                "{} {} {} LOST",
                turtle.pos.x, turtle.pos.y, turtle.orientation
            )
        } else {
            println!("{} {} {}", turtle.pos.x, turtle.pos.y, turtle.orientation)
        }
    }

    fn in_bounds(&self, pos: Pos) -> bool {
        self.grid.in_bounds(pos)
    }

    fn add_scent(&mut self, pos: Pos) {
        self.scents.insert(pos);
    }

    fn scent_seen(&self, pos: Pos) -> bool {
        self.scents.contains(&pos)
    }
}

#[cfg(test)]
mod test {
    use super::Direction::*;
    use super::Orientation::*;
    use super::{Direction, Mars, Pos, Turtle};

    #[test]
    fn test_moving_one_turtle() {
        let mut mars = Mars::new(Pos::new(5, 3));
        mars.move_turtle(
            Turtle::new(Pos::new(1, 1), East),
            &Direction::from_string("RFRFRFRF"),
        );

        assert_eq!(1, mars.turtles.len());
        assert_eq!(
            &Turtle {
                pos: Pos::new(1, 1),
                orientation: East,
                lost: false
            },
            mars.turtles.get(0).unwrap()
        );
    }

    #[test]
    fn test_not_moving_out_of_bounds_turtles() {
        let mut mars = Mars::new(Pos::new(5, 3));
        mars.move_turtle(Turtle::new(Pos::new(10, 20), East), &[Direction::Forward]);
        // We shouldn't have moved out of bounds turtles
        assert!(mars.turtles.is_empty());
        assert!(mars.scents.is_empty());
    }

    #[test]
    fn test_moving_three_turtles() {
        let mut mars = Mars::new(Pos::new(5, 3));
        mars.move_turtle(
            Turtle::new(Pos::new(1, 1), East),
            &Direction::from_string("RFRFRFRF"),
        );
        mars.move_turtle(
            Turtle::new(Pos::new(3, 2), North),
            &Direction::from_string("FRRFLLFFRRFLL"),
        );
        mars.move_turtle(
            Turtle::new(Pos::new(0, 3), West),
            &Direction::from_string("LLFFFLFLFL"),
        );

        assert_eq!(3, mars.turtles.len());

        assert_eq!(
            &Turtle {
                pos: Pos::new(1, 1),
                orientation: East,
                lost: false
            },
            mars.turtles.get(0).unwrap()
        );
        assert_eq!(
            &Turtle {
                pos: Pos::new(3, 3),
                orientation: North,
                lost: true
            },
            mars.turtles.get(1).unwrap()
        );
        assert_eq!(
            &Turtle {
                pos: Pos::new(2, 3),
                orientation: South,
                lost: false
            },
            mars.turtles.get(2).unwrap()
        );
    }

    #[test]
    fn test_parsing_directions() {
        assert_eq!(Direction::from_string(""), vec![]);

        assert_eq!(Direction::from_string("XYZ"), vec![]);

        assert_eq!(Direction::from_string("RL"), vec![Right, Left]);

        assert_eq!(
            Direction::from_string("RFRFRFRF"),
            vec![Right, Forward, Right, Forward, Right, Forward, Right, Forward,]
        );
    }

    #[test]
    fn test_turtle_to_string() {
        assert_eq!(format!("{}", Turtle::new(Pos::new(1, 1), East)), "1 1 E");
        assert_eq!(format!("{}", Turtle::new(Pos::new(3, 5), West)), "3 5 W");
        assert_eq!(format!("{}", Turtle::new(Pos::new(8, 10), North)), "8 10 N");
    }

    #[test]
    fn test_reading_turtle() {
        assert_eq!(Turtle::new(Pos::new(3, 4), East), "3 4 E".parse().unwrap());
        assert_eq!(Turtle::new(Pos::new(5, 8), West), "5 8 W".parse().unwrap());
    }
}
