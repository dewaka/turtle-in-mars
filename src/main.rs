use crate::Direction::{Forward, Right};
use crate::Orientation::{East, North, South, West};
use std::collections::HashSet;

#[derive(Debug, Eq, PartialEq, Hash, Copy, Clone)]
struct Pos {
    x: i32,
    y: i32,
}

impl Pos {
    fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }
}

#[derive(Debug, Eq, PartialEq)]
enum Orientation {
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
                    West
                } else if direction == Right {
                    East
                } else {
                    North
                }
            }
            Orientation::South => {
                if direction == Direction::Left {
                    East
                } else if direction == Right {
                    West
                } else {
                    South
                }
            }
            Orientation::East => {
                if direction == Direction::Left {
                    North
                } else if direction == Right {
                    South
                } else {
                    East
                }
            }
            Orientation::West => {
                if direction == Direction::Left {
                    South
                } else if direction == Right {
                    North
                } else {
                    West
                }
            }
        }
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
enum Direction {
    Left,
    Right,
    Forward,
}

#[derive(Debug)]
struct Turtle {
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
            North => Pos::new(self.pos.x, self.pos.y + 1),
            South => Pos::new(self.pos.x, self.pos.y - 1),
            East => Pos::new(self.pos.x + 1, self.pos.y),
            West => Pos::new(self.pos.x - 1, self.pos.y),
        }
    }
}

#[derive(Debug)]
struct Grid {
    lower_left: Pos,
    upper_right: Pos,
}

#[derive(Debug)]
struct Mars {
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
struct TurtleMover {
    mars: Mars,
    turtles: Vec<Turtle>,
}

impl TurtleMover {
    fn new(pos: Pos) -> Self {
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

    fn move_turtle(&mut self, start_pos: Pos, orientation: Orientation, direction: &[Direction]) {
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

fn main() {
    println!("Hello, world!");
}

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
