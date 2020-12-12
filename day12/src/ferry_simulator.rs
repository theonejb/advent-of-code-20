use std::collections::HashMap;

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

#[derive(Debug,PartialEq,Hash)]
pub enum Direction { North, South, East, West }

#[derive(Debug,PartialEq,Hash)]
pub enum RotationDirection { Left, Right }

#[derive(Debug)]
pub struct Ship {
    pub direction: Direction,
    pub north: i32,
    pub east: i32,
}

impl Ship {
    pub fn new() -> Ship {
        Ship { direction: Direction::East, north: 0, east: 0 }
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

    pub fn move_ship(&mut self, movement: &MovementInstruction) {
        match movement {
            MovementInstruction::North(by) => {
                self.north += *by as i32;
            }
            MovementInstruction::South(by) => {
                self.north -= *by as i32;
            }
            MovementInstruction::East(by) => {
                self.east += *by as i32;
            }
            MovementInstruction::West(by) => {
                self.east -= *by as i32;
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
                        self.north += *by as i32;
                    }
                    Direction::South => {
                        self.north -= *by as i32;
                    }
                    Direction::East => {
                        self.east += *by as i32;
                    }
                    Direction::West => {
                        self.east -= *by as i32;
                    }
                }
            }
        }
    }

    pub fn get_manhattan_distance(&self) -> i32 {
        self.north.abs() + self.east.abs()
    }
}