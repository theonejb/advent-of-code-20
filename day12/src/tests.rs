use crate::ferry_simulator::*;
use crate::ferry_simulator::Direction::{North, West, South, East};
use crate::ferry_simulator::RotationDirection::{Left, Right};

#[test]
fn test_rotation() {
    assert_eq!(Ship::rotate(&North, Left, 90), West, "North to Left 90");
    assert_eq!(Ship::rotate(&North, Left, 180), South, "North to Left 180");
    assert_eq!(Ship::rotate(&North, Left, 270), East, "North to Left 270");

    assert_eq!(Ship::rotate(&North, Right, 90), East, "North to Right 90");
    assert_eq!(Ship::rotate(&North, Right, 180), South, "North to Right 180");
    assert_eq!(Ship::rotate(&North, Right, 270), West, "North to Right 270");

    assert_eq!(Ship::rotate(&East, Left, 90), North, "East to Left 90");
    assert_eq!(Ship::rotate(&East, Left, 180), West, "East to Left 180");
    assert_eq!(Ship::rotate(&East, Left, 270), South, "East to Left 270");

    assert_eq!(Ship::rotate(&East, Right, 90), South, "East to Right 90");
    assert_eq!(Ship::rotate(&East, Right, 180), West, "East to Right 180");
    assert_eq!(Ship::rotate(&East, Right, 270), North, "East to Right 270");

    assert_eq!(Ship::rotate(&South, Left, 90), East, "South to Left 90");
    assert_eq!(Ship::rotate(&South, Left, 180), North, "South to Left 180");
    assert_eq!(Ship::rotate(&South, Left, 270), West, "South to Left 270");

    assert_eq!(Ship::rotate(&South, Right, 90), West, "South to Right 90");
    assert_eq!(Ship::rotate(&South, Right, 180), North, "South to Right 180");
    assert_eq!(Ship::rotate(&South, Right, 270), East, "South to Right 270");

    assert_eq!(Ship::rotate(&West, Left, 90), South, "West to Left 90");
    assert_eq!(Ship::rotate(&West, Left, 180), East, "West to Left 180");
    assert_eq!(Ship::rotate(&West, Left, 270), North, "West to Left 270");

    assert_eq!(Ship::rotate(&West, Right, 90), North, "West to Right 90");
    assert_eq!(Ship::rotate(&West, Right, 180), East, "West to Right 180");
    assert_eq!(Ship::rotate(&West, Right, 270), South, "West to Right 270");
}

#[test]
fn test_move_ship() {
    let mut ship = Ship::new();

    ship.move_ship(&MovementInstruction::compile("F10"));
    assert_eq!(ship.east, 10.0);
    assert_eq!(ship.north, 0.0);

    ship.move_ship(&MovementInstruction::compile("N3"));
    assert_eq!(ship.east, 10.0);
    assert_eq!(ship.north, 3.0);

    ship.move_ship(&MovementInstruction::compile("F7"));
    assert_eq!(ship.east, 17.0);
    assert_eq!(ship.north, 3.0);

    ship.move_ship(&MovementInstruction::compile("R90"));
    assert_eq!(ship.direction, South);
    assert_eq!(ship.east, 17.0);
    assert_eq!(ship.north, 3.0);

    ship.move_ship(&MovementInstruction::compile("F11"));
    assert_eq!(ship.east, 17.0);
    assert_eq!(ship.north, -8.0);
}

#[test]
fn test_get_manhattan_distance() {
    let input = [
        "F10",
        "N3",
        "F7",
        "R90",
        "F11",
    ];

    let mut ship = Ship::new();

    for instruction in input.iter() {
        ship.move_ship(&MovementInstruction::compile(*instruction));
    }

    assert_eq!(25.0, ship.get_manhattan_distance());
}

#[test]
fn test_rotate_waypoint() {
    let waypoint = Ship::rotate_waypoint(
        &Waypoint { east: 10.0, north: 4.0 },
        Right,
        90,
    );

    assert_eq!(waypoint.east.round(), 4.0);
    assert_eq!(waypoint.north.round(), -10.0);

    let waypoint = Ship::rotate_waypoint(
        &waypoint,
        Left,
        90
    );
    assert_eq!(waypoint.east.round(), 10.0);
    assert_eq!(waypoint.north.round(), 4.0);
}

#[test]
fn test_move_waypoint() {
    let waypoint = Waypoint { east:10.0, north: 1.0};
    let waypoint = Ship::move_waypoint(&waypoint, North, 3);
    assert_eq!(waypoint.east, 10.0);
    assert_eq!(waypoint.north, 4.0);
}

#[test]
fn test_move_ship_towards_waypoint() {
    let mut ship = Ship::new();
    ship.move_ship_towards_waypoint(10);
    assert_eq!(ship.east, 100.0);
    assert_eq!(ship.north, 10.0);
}

#[test]
fn test_follow_instruction() {
    let input = [
        "F10",
        "N3",
        "F7",
        "R90",
        "F11",
    ];

    let mut ship = Ship::new();

    for instruction in input.iter() {
        ship.follow_instruction(&MovementInstruction::compile(*instruction));
    }

    assert_eq!(ship.east, 214.0);
    assert_eq!(ship.north, -72.0);

    assert_eq!(ship.waypoint.east, 4.0);
    assert_eq!(ship.waypoint.north, -10.0);

    assert_eq!(ship.get_manhattan_distance(), 286.0);
}