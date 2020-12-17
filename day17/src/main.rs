mod tests;

use std::collections::HashMap;

use itertools::iproduct;
use std::i32::{MIN, MAX};
use std::ops::Range;

#[derive(Debug, Eq, Hash, Copy, Clone)]
struct Point(i32, i32, i32);

impl PartialEq for Point {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0 && self.1 == other.1 && self.2 == other.2
    }
}

impl Point {
    pub fn neighbours(&self) -> Vec<Point> {
        let x_range = self.0 - 1..self.0 + 2;
        let y_range = self.1 - 1..self.1 + 2;
        let z_range = self.2 - 1..self.2 + 2;

        let mut neighbours = vec![];
        for (x, y, z) in iproduct!(x_range, y_range, z_range) {
            if x == self.0 && y == self.1 && z == self.2 {
                continue;
            }

            neighbours.push(
                Point(x, y, z)
            );
        }

        neighbours
    }
}

#[derive(Debug, PartialEq, Copy, Clone)]
enum CubeStatus {
    Active,
    Inactive,
}

#[derive(Debug)]
struct Grid {
    cubes: HashMap<Point, CubeStatus>
}

impl Grid {
    pub fn new() -> Grid {
        Grid {
            cubes: HashMap::new()
        }
    }

    pub fn new_from_input(input: &Vec<String>) -> Grid {
        let mut grid = Grid::new();

        const Z: i32 = 0;
        for (y, row) in input.iter().enumerate() {
            for (x, col) in row.chars().enumerate() {
                if col == '#' {
                    grid.update_cube_at_point(&Point(x as i32, y as i32, Z), CubeStatus::Active);
                }
            }
        }

        grid
    }

    pub fn point_to_cube_status(&self, p: &Point) -> CubeStatus {
        match self.cubes.get(p) {
            None => CubeStatus::Inactive,
            Some(cube_status) => cube_status.clone()
        }
    }

    pub fn update_cube_at_point(&mut self, p: &Point, cube_status: CubeStatus) {
        let p = p.clone();
        self.cubes.insert(p, cube_status);
    }

    pub fn next_state_for_point(&self, p: &Point) -> CubeStatus {
        let neighbours = p.neighbours();
        let mut active_neighbours = 0;
        let mut inactive_neighbours = 0;

        for neighbour in neighbours {
            let state = self.point_to_cube_status(&neighbour);
            match state {
                CubeStatus::Active => {
                    active_neighbours += 1;
                }
                CubeStatus::Inactive => {
                    inactive_neighbours += 1;
                }
            }
        }

        match self.point_to_cube_status(p) {
            CubeStatus::Active => {
                if active_neighbours == 2 || active_neighbours == 3 {
                    CubeStatus::Active
                } else {
                    CubeStatus::Inactive
                }
            }
            CubeStatus::Inactive => {
                if active_neighbours == 3 {
                    CubeStatus::Active
                } else {
                    CubeStatus::Inactive
                }
            }
        }
    }

    pub fn get_extents_of_grid_to_consider(&self) -> ((i32, i32), (i32, i32), (i32, i32)) {
        let (mut min_x, mut min_y, mut min_z) = (MAX, MAX, MAX);
        let (mut max_x, mut max_y, mut max_z) = (MIN, MIN, MIN);

        for (p, state) in self.cubes.iter() {
            if *state == CubeStatus::Active {
                if p.0 < min_x {
                    min_x = p.0;
                }
                if p.1 < min_y {
                    min_y = p.1;
                }
                if p.2 < min_z {
                    min_z = p.2;
                }

                if p.0 > max_x {
                    max_x = p.0;
                }
                if p.1 > max_y {
                    max_y = p.1;
                }
                if p.2 > max_z {
                    max_z = p.2;
                }
            }
        }

        ((min_x - 1, max_x + 1), (min_y - 1, max_y + 1), (min_z - 1, max_z + 1))
    }

    pub fn tick(&mut self) {
        let (
            (min_x, max_x),
            (min_y, max_y),
            (min_z, max_z)
        ) = self.get_extents_of_grid_to_consider();

        let mut new_cubes = HashMap::new();
        for (x, y, z) in iproduct!(min_x .. max_x + 1, min_y .. max_y + 1, min_z .. max_y + 1) {
            let p = Point(x, y, z);
            new_cubes.insert(p, self.next_state_for_point(&p));
        }

        self.cubes = new_cubes;
    }

    pub fn number_of_active_cubes(&self) -> usize {
        let mut n_active = 0usize;

        for (_, state) in self.cubes.iter() {
            if *state == CubeStatus::Active {
                n_active += 1;
            }
        }

        n_active
    }

    pub fn print(&self, x_range: Range<i32>, y_range: Range<i32>, z_range: Range<i32>) {
        let x_range: Vec<i32> = x_range.collect();
        let y_range: Vec<i32> = y_range.collect();
        let z_range: Vec<i32> = z_range.collect();

        for z in &z_range {
            println!("z = {}", z);
            for y in &y_range {
                for x in &x_range {
                    print!("{}", match self.point_to_cube_status(&Point(*x, *y, *z)) {
                        CubeStatus::Active => "#",
                        CubeStatus::Inactive => "."
                    });
                }

                println!();
            }
            println!()
        }
    }

    pub fn print_extents(&self) {
        let (
            (min_x, max_x),
            (min_y, max_y),
            (min_z, max_z)
        ) = self.get_extents_of_grid_to_consider();

        self.print(
            min_x..max_x + 1,
            min_y..max_y + 1,
            min_z..max_z + 1,
        );
    }
}

fn main() {
    let input = vec![
        String::from("######.#"),
        String::from("#.###.#."),
        String::from("###....."),
        String::from("#.####.."),
        String::from("##.#.###"),
        String::from(".######."),
        String::from("###.####"),
        String::from("######.#"),
    ];
    let mut grid = Grid::new_from_input(&input);
    for _ in 1..=6 {
        grid.tick();
    }
    println!("Part 1: {}", grid.number_of_active_cubes());
}
