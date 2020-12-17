use crate::*;
use itertools::assert_equal;

#[test]
fn test_point_neighbours() {
    let p = Point(0, 0, 0, 0);
    let neighbours = p.neighbours();
    assert_eq!(neighbours.len(), 80);
}

#[test]
fn test_grid_point_to_cube_status() {
    let mut grid = Grid::new();
    assert_eq!(grid.point_to_cube_status(&Point(0, 0, 0, 0)), CubeStatus::Inactive);
}

#[test]
fn test_grid_update_cube_at_point() {
    let mut grid = Grid::new();
    grid.update_cube_at_point(&Point(1, 1, 1, 0), CubeStatus::Active);

    assert_eq!(grid.point_to_cube_status(&Point(0, 0, 0, 0)), CubeStatus::Inactive);
    assert_eq!(grid.point_to_cube_status(&Point(1, 1, 1, 0)), CubeStatus::Active);
}

#[test]
fn test_grid_next_state_for_point() {
    let mut grid = Grid::new();
    grid.update_cube_at_point(&Point(1, 0, 0, 0), CubeStatus::Active);
    grid.update_cube_at_point(&Point(2, 1, 0, 0), CubeStatus::Active);
    grid.update_cube_at_point(&Point(0, 2, 0, 0), CubeStatus::Active);
    grid.update_cube_at_point(&Point(1, 2, 0, 0), CubeStatus::Active);
    grid.update_cube_at_point(&Point(2, 2, 0, 0), CubeStatus::Active);

    assert_eq!(
        grid.next_state_for_point(&Point(0, 1, 0, 0)),
        CubeStatus::Active
    );
}

#[test]
fn test_grid_get_extents_of_grid_to_consider() {
    let mut grid = Grid::new();
    grid.update_cube_at_point(&Point(1, 0, 0, 0), CubeStatus::Active);
    grid.update_cube_at_point(&Point(2, 1, 0, 0), CubeStatus::Active);
    grid.update_cube_at_point(&Point(0, 2, 0, 0), CubeStatus::Active);
    grid.update_cube_at_point(&Point(1, 2, 0, 0), CubeStatus::Active);
    grid.update_cube_at_point(&Point(2, 2, 0, 0), CubeStatus::Active);

    let (
        (min_x, max_x),
        (min_y, max_y),
        (min_z, max_z),
        (min_w, max_w),
    ) = grid.get_extents_of_grid_to_consider();

    assert_eq!(min_x, -1);
    assert_eq!(max_x, 3);

    assert_eq!(min_y, -1);
    assert_eq!(max_y, 3);

    assert_eq!(min_z, -1);
    assert_eq!(max_z, 1);

    assert_eq!(min_w, -1);
    assert_eq!(max_w, 1);
}

#[test]
fn test_grid_number_of_active_cubes() {
    let mut grid = Grid::new();
    grid.update_cube_at_point(&Point(1, 0, 0, 0), CubeStatus::Active);
    grid.update_cube_at_point(&Point(2, 1, 0, 0), CubeStatus::Active);
    grid.update_cube_at_point(&Point(0, 2, 0, 0), CubeStatus::Active);
    grid.update_cube_at_point(&Point(1, 2, 0, 0), CubeStatus::Active);
    grid.update_cube_at_point(&Point(2, 2, 0, 0), CubeStatus::Active);

    assert_eq!(grid.number_of_active_cubes(), 5);
}

#[test]
fn test_grid_tick() {
    let mut grid = Grid::new();
    grid.update_cube_at_point(&Point(1, 0, 0, 0), CubeStatus::Active);
    grid.update_cube_at_point(&Point(2, 1, 0, 0), CubeStatus::Active);
    grid.update_cube_at_point(&Point(0, 2, 0, 0), CubeStatus::Active);
    grid.update_cube_at_point(&Point(1, 2, 0, 0), CubeStatus::Active);
    grid.update_cube_at_point(&Point(2, 2, 0, 0), CubeStatus::Active);

    for _ in 1..=6 {
        grid.tick();
    }

    assert_eq!(grid.number_of_active_cubes(), 848);
}

#[test]
fn test_grid_new_from_input() {
    let input = vec![
        String::from(".#."),
        String::from("..#"),
        String::from("###"),
    ];
    let mut grid = Grid::new_from_input(&input);

    for _ in 1..=6 {
        grid.tick();
    }

    assert_eq!(grid.number_of_active_cubes(), 848);
}