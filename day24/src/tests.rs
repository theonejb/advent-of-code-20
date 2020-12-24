use crate::*;
use itertools::assert_equal;

#[test]
fn test_parse_tile_directions() {
    let directions = "esenee";
    let directions = parse_tile_directions(directions);

    assert_equal(directions, vec![
        Direction::East,
        Direction::SouthEast,
        Direction::NorthEast,
        Direction::East,
    ]);

    assert_equal(parse_tile_directions("seswneswswsenwwnwse"), vec![
        Direction::SouthEast,
        Direction::SouthWest,
        Direction::NorthEast,
        Direction::SouthWest,
        Direction::SouthWest,
        Direction::SouthEast,
        Direction::NorthWest,
        Direction::West,
        Direction::NorthWest,
        Direction::SouthEast
    ]);
}

#[test]
fn test_get_neighbour_in_direction() {
    let coord = HexCoord::new(0, 0);
    assert_eq!(
        get_neighbour_in_direction(&coord, &Direction::NorthWest),
        HexCoord::new(-1, -1)
    );
    assert_eq!(
        get_neighbour_in_direction(&coord, &Direction::NorthEast),
        HexCoord::new(1, -1)
    );
    assert_eq!(
        get_neighbour_in_direction(&coord, &Direction::East),
        HexCoord::new(2, 0)
    );
    assert_eq!(
        get_neighbour_in_direction(&coord, &Direction::SouthEast),
        HexCoord::new(1, 1)
    );
    assert_eq!(
        get_neighbour_in_direction(&coord, &Direction::SouthWest),
        HexCoord::new(-1, 1)
    );
    assert_eq!(
        get_neighbour_in_direction(&coord, &Direction::West),
        HexCoord::new(-2, 0)
    );
}

#[test]
fn test_solve1() {
    let inputs = vec![
        String::from("sesenwnenenewseeswwswswwnenewsewsw"),
        String::from("neeenesenwnwwswnenewnwwsewnenwseswesw"),
        String::from("seswneswswsenwwnwse"),
        String::from("nwnwneseeswswnenewneswwnewseswneseene"),
        String::from("swweswneswnenwsewnwneneseenw"),
        String::from("eesenwseswswnenwswnwnwsewwnwsene"),
        String::from("sewnenenenesenwsewnenwwwse"),
        String::from("wenwwweseeeweswwwnwwe"),
        String::from("wsweesenenewnwwnwsenewsenwwsesesenwne"),
        String::from("neeswseenwwswnwswswnw"),
        String::from("nenwswwsewswnenenewsenwsenwnesesenew"),
        String::from("enewnwewneswsewnwswenweswnenwsenwsw"),
        String::from("sweneswneswneneenwnewenewwneswswnese"),
        String::from("swwesenesewenwneswnwwneseswwne"),
        String::from("enesenwswwswneneswsenwnewswseenwsese"),
        String::from("wnwnesenesenenwwnenwsewesewsesesew"),
        String::from("nenewswnwewswnenesenwnesewesw"),
        String::from("eneswnwswnwsenenwnwnwwseeswneewsenese"),
        String::from("neswnwewnwnwseenwseesewsenwsweewe"),
        String::from("wseweeenwnesenwwwswnew"),
    ];
    assert_eq!(solve1(&inputs), 10);
}