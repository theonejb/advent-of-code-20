use crate::game_of_waiting_area_seats::PositionState::{EmptySeat, OccupiedSeat, Floor};
use std::fmt::{Display, Formatter};
use core::fmt;
use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum PositionState {
    EmptySeat,
    OccupiedSeat,
    Floor,
}

impl PositionState {
    fn from_input_char(c: char) -> PositionState {
        match c {
            '.' => Floor,
            'L' => EmptySeat,
            '#' => OccupiedSeat,
            _ => panic!("Unknown position state char.")
        }
    }

    fn to_char(&self) -> char {
        match self {
            EmptySeat => 'L',
            OccupiedSeat => '#',
            Floor => '.'
        }
    }
}

type WaitingAreaMap = Vec<Vec<PositionState>>;

#[derive(Debug)]
pub struct WaitingArea {
    pub map: WaitingAreaMap,
    pub width: usize,
    pub height: usize,
}

impl Display for WaitingArea {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        for row in self.map.iter() {
            for col in row {
                write!(f, "{}", col.to_char());
            }
            writeln!(f, "");
        }

        fmt::Result::Ok(())
    }
}

impl WaitingArea {
    pub fn new(input: &Vec<String>) -> WaitingArea {
        let height = input.len();
        let width = input[0].len();

        let mut waiting_area = WaitingArea {
            map: vec![],
            width,
            height,
        };

        for y in 0..height {
            let mut row: Vec<PositionState> = vec![];
            let row_chars = input[y].as_bytes();

            for x in 0..width {
                let position_state = PositionState::from_input_char(
                    row_chars[x] as char
                );
                row.push(position_state);
            }

            waiting_area.map.push(row);
        }

        waiting_area
    }

    pub fn get_number_of_occupied_seats(&self) -> usize {
        let mut occupied_seats = 0;
        for row in self.map.iter() {
            for position_state in row.iter() {
                match *position_state {
                    EmptySeat => {}
                    OccupiedSeat => { occupied_seats += 1; }
                    Floor => {}
                }
            }
        }

        occupied_seats
    }

    pub fn get_next_state(&self, x: usize, y: usize) -> (PositionState, bool) {
        let neighbours = self.get_neighbours(x, y);

        let mut state_counter = HashMap::new();
        state_counter.insert(&Floor, 0);
        state_counter.insert(&EmptySeat, 0);
        state_counter.insert(&OccupiedSeat, 0);

        for neighbour in neighbours.iter() {
            let neighbour_states_count = match state_counter.get(neighbour) {
                Some(count) => count + 1,
                None => 0
            };
            state_counter.insert(neighbour, neighbour_states_count);
        }

        match self.map[y][x] {
            EmptySeat => {
                if *state_counter.get(&OccupiedSeat).unwrap() == 0 {
                    (OccupiedSeat, true)
                } else {
                    (EmptySeat, false)
                }
            }
            OccupiedSeat => {
                if *state_counter.get(&OccupiedSeat).unwrap() >= 4 {
                    (EmptySeat, true)
                } else {
                    (OccupiedSeat, false)
                }
            }
            Floor => (Floor, false)
        }
    }

    pub fn get_next_state2(&self, x: usize, y: usize) -> (PositionState, bool) {
        let neighbours = self.get_visible_neighbours(x, y);

        let mut state_counter = HashMap::new();
        state_counter.insert(&Floor, 0);
        state_counter.insert(&EmptySeat, 0);
        state_counter.insert(&OccupiedSeat, 0);

        for neighbour in neighbours.iter() {
            let neighbour_states_count = match state_counter.get(neighbour) {
                Some(count) => count + 1,
                None => 0
            };
            state_counter.insert(neighbour, neighbour_states_count);
        }

        match self.map[y][x] {
            EmptySeat => {
                if *state_counter.get(&OccupiedSeat).unwrap() == 0 {
                    (OccupiedSeat, true)
                } else {
                    (EmptySeat, false)
                }
            }
            OccupiedSeat => {
                if *state_counter.get(&OccupiedSeat).unwrap() >= 5 {
                    (EmptySeat, true)
                } else {
                    (OccupiedSeat, false)
                }
            }
            Floor => (Floor, false)
        }
    }


    pub fn get_first_visible_neighbour_from_with_slope(&self, x: usize, y: usize, slope: (i32, i32)) -> PositionState {
        let mut x = x as i32;
        let mut y = y as i32;

        let (dx, dy) = slope;

        loop {
            x += dx;
            y += dy;

            if x < 0 || x >= self.width as i32 {
                break;
            }
            if y < 0 || y >= self.height as i32 {
                break;
            }

            match &self.map[y as usize][x as usize] {
                Floor => {
                    continue;
                },
                x => { return x.clone(); }
            }
        }

        Floor
    }

    pub fn get_visible_neighbours(&self, x: usize, y: usize) -> Vec<PositionState> {
        let slopes = [
            (-1, -1),
            (0, -1),
            (1, -1),
            (-1, 0),
            (1, 0),
            (-1, 1),
            (0, 1),
            (1, 1)
        ];

        slopes.iter().map(|slope| {
            self.get_first_visible_neighbour_from_with_slope(x, y, slope.clone())
        }).collect()
    }

    pub fn get_neighbours(&self, x: usize, y: usize) -> Vec<PositionState> {
        let x = x as i32;
        let y = y as i32;
        let all_neighbours = vec![
            (x - 1, y - 1),
            (x, y - 1),
            (x + 1, y - 1),
            (x - 1, y),
            (x + 1, y),
            (x - 1, y + 1),
            (x, y + 1),
            (x + 1, y + 1)
        ];
        let valid_neighbours = all_neighbours.iter().filter(|n| {
            let (x, y) = n;
            let x = *x;
            let y = *y;

            if x < 0 || x >= self.width as i32 {
                return false;
            }

            if y < 0 || y >= self.height as i32 {
                return false;
            }

            true
        });

        let mut neighbours: Vec<PositionState> = vec![];
        for (x, y) in valid_neighbours {
            let x = *x as usize;
            let y = *y as usize;

            let position_state = &self.map[y][x];
            neighbours.push(
                position_state.clone()
            );
        }
        neighbours
    }

    pub fn tick(&mut self) -> bool {
        let mut next_map = vec![];
        let mut did_map_change = false;

        for y in 0..self.height {
            let mut row = vec![];

            for x in 0..self.width {
                let (next_state, did_change) = self.get_next_state(x, y);
                row.push(
                    next_state
                );

                if did_change {
                    did_map_change = true;
                }
            }

            next_map.push(row);
        }

        self.map = next_map;
        did_map_change
    }

    pub fn tick2(&mut self) -> bool {
        let mut next_map = vec![];
        let mut did_map_change = false;

        for y in 0..self.height {
            let mut row = vec![];

            for x in 0..self.width {
                let (next_state, did_change) = self.get_next_state2(x, y);
                row.push(
                    next_state
                );

                if did_change {
                    did_map_change = true;
                }
            }

            next_map.push(row);
        }

        self.map = next_map;
        did_map_change
    }
}