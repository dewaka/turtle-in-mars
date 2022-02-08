use std::collections::HashSet;

#[derive(Debug, Eq, PartialEq, Hash, Copy, Clone)]
pub struct Pos {
    pub x: i32,
    pub y: i32,
}

impl Pos {
    fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }
}

#[derive(Debug, Eq, PartialEq)]
pub enum Orientation {
    North,
    South,
    East,
    West,
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

#[derive(Debug)]
pub struct Turtle {
    pos: Pos,
    orientation: Orientation,
    lost: bool,
}

impl Turtle {
    fn new(pos: Pos, orientation: Orientation) -> Self {
        Self {
            pos,
            orientation,
            lost: false,
        }
    }

    fn make_move(&mut self, mars: &mut Mars, direction: Direction) {
        match direction {
            Direction::Left => self.orientation = self.orientation.with_direction(direction),
            Direction::Right => self.orientation = self.orientation.with_direction(direction),
            Direction::Forward => {
                let new_pos = self.advanced_pos();
                if mars.in_bounds(new_pos) {
                    self.pos = new_pos;
                } else if !mars.scent_seen(new_pos) {
                    self.pos = new_pos;
                    self.lost = true;
                    mars.add_scent(new_pos);
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

#[derive(Debug)]
pub struct Mars {
    grid: Grid,
    scents: HashSet<Pos>,
}

impl Mars {
    fn in_bounds(&self, pos: Pos) -> bool {
        let lower = self.grid.lower_left;
        let upper = self.grid.upper_right;

        (pos.x >= lower.x && pos.y >= lower.y) && (pos.x <= upper.x && pos.y <= upper.y)
    }

    fn add_scent(&mut self, pos: Pos) {
        self.scents.insert(pos);
    }

    fn scent_seen(&self, pos: Pos) -> bool {
        self.scents.contains(&pos)
    }
}

#[derive(Debug)]
pub struct TurtleMover {
    mars: Mars,
    turtles: Vec<Turtle>,
}

impl TurtleMover {
    pub fn new(pos: Pos) -> Self {
        Self {
            mars: Mars {
                grid: Grid {
                    lower_left: Pos::new(0, 0),
                    upper_right: pos,
                },
                scents: HashSet::new(),
            },
            turtles: vec![],
        }
    }

    pub fn move_turtle(
        &mut self,
        start_pos: Pos,
        orientation: Orientation,
        direction: &[Direction],
    ) {
        let mut turtle = Turtle::new(start_pos, orientation);

        for &dir in direction {
            turtle.make_move(&mut self.mars, dir);
            if turtle.lost {
                break;
            }
        }

        self.turtles.push(turtle);
    }
}

#[cfg(test)]
mod test {
    use super::Direction::*;
    use super::Orientation::*;
    use super::{Direction, Pos, TurtleMover};

    #[test]
    fn test_moving_one_turtle() {
        let mut mover = TurtleMover::new(Pos::new(5, 3));
        let dir = vec![
            Right, Forward, Right, Forward, Right, Forward, Right, Forward,
        ];
        mover.move_turtle(Pos::new(1, 1), East, &dir);

        assert_eq!(1, mover.turtles.len());
        let turtle = mover.turtles.get(0).unwrap();
        assert!(!turtle.lost);
        assert_eq!(turtle.pos, Pos::new(1, 1));
        assert_eq!(turtle.orientation, East);
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
}
