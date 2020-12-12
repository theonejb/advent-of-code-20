use std::collections::HashMap;
use std::convert::TryInto;

#[derive(Debug)]
pub enum MovementInstruction {
    North(usize),
    South(usize),
    East(usize),
    West(usize),
    Left(usize),
    Right(usize),
    Forward(usize),
}

impl MovementInstruction {
    pub fn compile(input: &str) -> MovementInstruction {
        let instruction_char = input.chars().nth(0).unwrap();
        let operand = &input[1..];
        let operand = operand.parse::<usize>().unwrap();

        match instruction_char {
            'N' => MovementInstruction::North(operand),
            'S' => MovementInstruction::South(operand),
            'E' => MovementInstruction::East(operand),
            'W' => MovementInstruction::West(operand),
            'L' => MovementInstruction::Left(operand),
            'R' => MovementInstruction::Right(operand),
            'F' => MovementInstruction::Forward(operand),
            _ => {
                panic!("Unknown movement instruction")
            }
        }
    }
}

#[derive(Debug, PartialEq, Hash)]
pub enum Direction { North, South, East, West }

#[derive(Debug, PartialEq, Hash)]
pub enum RotationDirection { Left, Right }

#[derive(Debug)]
pub struct Waypoint {
    pub north: f32,
    pub east: f32,
}

#[derive(Debug)]
pub struct Ship {
    pub direction: Direction,
    pub north: f32,
    pub east: f32,
    pub waypoint: Waypoint,
}

impl Ship {
    pub fn new() -> Ship {
        Ship { direction: Direction::East, north: 0.0, east: 0.0, waypoint: Waypoint { east: 10.0, north: 1.0 } }
    }

    pub fn rotate(facing: &Direction, rotation_direction: RotationDirection, by: usize) -> Direction {
        // These are the only valid values in my input
        assert!(by == 90 || by == 180 || by == 270);

        if rotation_direction == RotationDirection::Left {
            match facing {
                Direction::North => {
                    if by == 90 {
                        return Direction::West;
                    } else if by == 180 {
                        return Direction::South;
                    } else if by == 270 {
                        return Direction::East;
                    }
                }
                Direction::South => {
                    if by == 90 {
                        return Direction::East;
                    } else if by == 180 {
                        return Direction::North;
                    } else if by == 270 {
                        return Direction::West;
                    }
                }
                Direction::East => {
                    if by == 90 {
                        return Direction::North;
                    } else if by == 180 {
                        return Direction::West;
                    } else if by == 270 {
                        return Direction::South;
                    }
                }
                Direction::West => {
                    if by == 90 {
                        return Direction::South;
                    } else if by == 180 {
                        return Direction::East;
                    } else if by == 270 {
                        return Direction::North;
                    }
                }
            }
        } else if rotation_direction == RotationDirection::Right {
            match facing {
                Direction::North => {
                    if by == 90 {
                        return Direction::East;
                    } else if by == 180 {
                        return Direction::South;
                    } else if by == 270 {
                        return Direction::West;
                    }
                }
                Direction::South => {
                    if by == 90 {
                        return Direction::West;
                    } else if by == 180 {
                        return Direction::North;
                    } else if by == 270 {
                        return Direction::East;
                    }
                }
                Direction::East => {
                    if by == 90 {
                        return Direction::South;
                    } else if by == 180 {
                        return Direction::West;
                    } else if by == 270 {
                        return Direction::North;
                    }
                }
                Direction::West => {
                    if by == 90 {
                        return Direction::North;
                    } else if by == 180 {
                        return Direction::East;
                    } else if by == 270 {
                        return Direction::South;
                    }
                }
            }
        }

        panic!("Unable to compute new direction");
    }

    pub fn rotate_waypoint(waypoint: &Waypoint, rotation_direction: RotationDirection, by: usize) -> Waypoint {
        let mut by = by as f32;

        // Angles are measured anti-clockwise as a standard. So a 90 degree Right rotation is a -90 degree anti-clockwise rotation
        if rotation_direction == RotationDirection::Right {
            by = 360.0 - by;
        }
        by = by.to_radians();

        let sin_t = (by as f32).sin();
        let cos_t = (by as f32).cos();
        let east = waypoint.east as f32;
        let north = waypoint.north as f32;

        let new_east = cos_t * east - sin_t * north;
        let new_north = sin_t * east + cos_t * north;

        Waypoint {
            east: new_east,
            north: new_north
        }
    }

    pub fn move_waypoint(waypoint: &Waypoint, direction: Direction, by: usize) -> Waypoint {
        match direction {
            Direction::North => Waypoint {north: waypoint.north + by as f32, ..*waypoint},
            Direction::South => Waypoint {north: waypoint.north - by as f32, ..*waypoint},
            Direction::East => Waypoint {east: waypoint.east + by as f32, ..*waypoint},
            Direction::West => Waypoint {east: waypoint.east - by as f32, ..*waypoint},
        }
    }

    pub fn move_ship(&mut self, movement: &MovementInstruction) {
        match movement {
            MovementInstruction::North(by) => {
                self.north += *by as f32;
            }
            MovementInstruction::South(by) => {
                self.north -= *by as f32;
            }
            MovementInstruction::East(by) => {
                self.east += *by as f32;
            }
            MovementInstruction::West(by) => {
                self.east -= *by as f32;
            }
            MovementInstruction::Left(by) => {
                self.direction = Ship::rotate(&self.direction, RotationDirection::Left, *by)
            }
            MovementInstruction::Right(by) => {
                self.direction = Ship::rotate(&self.direction, RotationDirection::Right, *by)
            }
            MovementInstruction::Forward(by) => {
                match self.direction {
                    Direction::North => {
                        self.north += *by as f32;
                    }
                    Direction::South => {
                        self.north -= *by as f32;
                    }
                    Direction::East => {
                        self.east += *by as f32;
                    }
                    Direction::West => {
                        self.east -= *by as f32;
                    }
                }
            }
        }
    }

    pub fn follow_instruction(&mut self, instruction: &MovementInstruction) {
        match instruction {
            MovementInstruction::North(by) => {
                self.waypoint = Ship::move_waypoint(&self.waypoint, Direction::North, *by);
            }
            MovementInstruction::South(by) => {
                self.waypoint = Ship::move_waypoint(&self.waypoint, Direction::South, *by);
            }
            MovementInstruction::East(by) => {
                self.waypoint = Ship::move_waypoint(&self.waypoint, Direction::East, *by);
            }
            MovementInstruction::West(by) => {
                self.waypoint = Ship::move_waypoint(&self.waypoint, Direction::West, *by);
            }
            MovementInstruction::Left(by) => {
                self.waypoint = Ship::rotate_waypoint(&self.waypoint, RotationDirection::Left, *by);
            }
            MovementInstruction::Right(by) => {
                self.waypoint = Ship::rotate_waypoint(&self.waypoint, RotationDirection::Right, *by);
            }
            MovementInstruction::Forward(by) => {
                self.move_ship_towards_waypoint(*by);
            }
        }
    }

    pub fn move_ship_towards_waypoint(&mut self, by: usize) {
        self.east += self.waypoint.east * by as f32;
        self.north += self.waypoint.north * by as f32;
    }

    pub fn get_manhattan_distance(&self) -> f32 {
        self.north.round().abs() + self.east.round().abs()
    }
}