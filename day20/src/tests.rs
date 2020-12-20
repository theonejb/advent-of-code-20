use crate::*;
use std::env::var;

#[test]
fn test_matrix_apply_transform() {
    let p = Point::new(3, 2);

    assert_eq!(
        Matrix::X_FLIP.apply(&p),
        Point::new(-3, 2)
    );

    assert_eq!(
        Matrix::Y_FLIP.apply(&p),
        Point::new(3, -2)
    );

    assert_eq!(
        Matrix::XY_FLIP.apply(&p),
        Point::new(-3, -2)
    );
}

#[test]
fn test_matrix_apply_rotate() {
    assert_eq!(
        Matrix::CW_ROTATE.apply(&Point::new(3, 2)),
        Point::new(-2, 3)
    );
    assert_eq!(
        Matrix::CW_ROTATE.apply(&Point::new(1, 0)),
        Point::new(0, 1)
    );
    assert_eq!(
        Matrix::CW_ROTATE.apply(&Point::new(1, 1)),
        Point::new(-1, 1)
    );
    assert_eq!(
        Matrix::CW_ROTATE.apply(&Point::new(5, 7)),
        Point::new(-7, 5)
    );

    assert_eq!(
        Matrix::CCW_ROTATE.apply(&Point::new(-2, 3)),
        Point::new(3, 2)
    );
    assert_eq!(
        Matrix::CCW_ROTATE.apply(&Point::new(0, 1)),
        Point::new(1, 0)
    );
    assert_eq!(
        Matrix::CCW_ROTATE.apply(&Point::new(-1, 1)),
        Point::new(1, 1)
    );

    assert_eq!(
        Matrix::CCW_ROTATE.apply(&Point::new(5, 7)),
        Point::new(7, -5)
    );
    assert_eq!(
        Matrix::CCW_ROTATE.apply(&Point::new(7, -5)),
        Point::new(-5, -7)
    );
    assert_eq!(
        Matrix::CCW_ROTATE.apply(&Point::new(-5, -7)),
        Point::new(-7, 5)
    );
    assert_eq!(
        Matrix::CCW_ROTATE.apply(&Point::new(-7, 5)),
        Point::new(5, 7)
    );

    assert_eq!(
        Matrix::CW_ROTATE.apply(&Point::new(-3, 7)),
        Point::new(-7, -3)
    );
    assert_eq!(
        Matrix::CW_ROTATE.apply(&Point::new(-7, -3)),
        Point::new(3, -7)
    );
    assert_eq!(
        Matrix::CW_ROTATE.apply(&Point::new(3, -7)),
        Point::new(7, 3)
    );
    assert_eq!(
        Matrix::CW_ROTATE.apply(&Point::new(7, 3)),
        Point::new(-3, 7)
    );
}


fn get_tile_2311() -> Tile {
    let input = vec![
        String::from("..##.#..#."),
        String::from("##..#....."),
        String::from("#...##..#."),
        String::from("####.#...#"),
        String::from("##.##.###."),
        String::from("##...#.###"),
        String::from(".#.#.#..##"),
        String::from("..#....#.."),
        String::from("###...#.#."),
        String::from("..###..###"),
    ];
    Tile::from_input(2311, &input)
}

fn get_tile_1427() -> Tile {
    let input = vec![
        String::from("###.##.#.."),
        String::from(".#..#.##.."),
        String::from(".#.##.#..#"),
        String::from("#.#.#.##.#"),
        String::from("....#...##"),
        String::from("...##..##."),
        String::from("...#.#####"),
        String::from(".#.####.#."),
        String::from("..#..###.#"),
        String::from("..##.#..#."),
    ];
    Tile::from_input(1427, &input)
}

#[test]
fn test_tile_from_input() {
    let tile = get_tile_2311();

    assert_eq!(tile.get_pixel_value(&Point::new(0, 0)), Pixel::Black);
    assert_eq!(tile.get_pixel_value(&Point::new(0, 1)), Pixel::White);
    assert_eq!(tile.get_pixel_value(&Point::new(9, 9)), Pixel::White);
    assert_eq!(tile.get_pixel_value(&Point::new(9, 8)), Pixel::Black);
    assert_eq!(tile.width, 10);
    assert_eq!(tile.height, 10);
}

#[test]
fn test_tile_variant_from_tile_with_transformations() {
    let tile = get_tile_2311();

    let transformations = vec![];
    let variant = TileVariant::from_tile_with_transformations(&tile, &transformations);
    assert_eq!(variant.get_pixel_value(&Point::new(0, 0)), Pixel::Black);
    assert_eq!(variant.get_pixel_value(&Point::new(1, 0)), Pixel::Black);
    assert_eq!(variant.get_pixel_value(&Point::new(9, 8)), Pixel::Black);
    assert_eq!(variant.get_pixel_value(&Point::new(9, 9)), Pixel::White);
    assert_eq!(variant.width, 10);
    assert_eq!(variant.height, 10);

    let transformations = vec![Matrix::CW_ROTATE];
    let variant = TileVariant::from_tile_with_transformations(&tile, &transformations);
    assert_eq!(variant.get_pixel_value(&Point::new(0, 0)), Pixel::Black);
    assert_eq!(variant.get_pixel_value(&Point::new(1, 0)), Pixel::White);
    assert_eq!(variant.get_pixel_value(&Point::new(9, 8)), Pixel::White);
    assert_eq!(variant.get_pixel_value(&Point::new(9, 9)), Pixel::Black);
    assert_eq!(variant.width, 10);
    assert_eq!(variant.height, 10);
}

#[test]
fn test_tile_get_all_variants() {
    let tile = get_tile_2311();
    for (i, variant) in tile.variants.iter().enumerate() {
        variant.write(&format!("test_images/tile_{}_variant_{}.bmp", tile.id, variant.variant_id));
    }

    let tile = get_tile_1427();

    for (i, variant) in tile.variants.iter().enumerate() {
        variant.write(&format!("test_images/tile_{}_variant_{}.bmp", tile.id, variant.variant_id));
    }
}

#[test]
fn test_tile_variant_get_border() {
    let tile = get_tile_2311();
    let variant = &tile.variants[0];

    let border = variant.get_border(Side::Top);
    assert_eq!(border.pixels, vec![Pixel::Black, Pixel::Black, Pixel::White, Pixel::White, Pixel::Black, Pixel::White, Pixel::Black, Pixel::Black, Pixel::White, Pixel::Black]);

    let border = variant.get_border(Side::Right);
    assert_eq!(border.pixels, vec![
        Pixel::Black,
        Pixel::Black,
        Pixel::Black,
        Pixel::White,
        Pixel::Black,
        Pixel::White,
        Pixel::White,
        Pixel::Black,
        Pixel::Black,
        Pixel::White,
    ]);

    let border = variant.get_border(Side::Bottom);
    assert_eq!(border.pixels, vec![
        Pixel::Black, Pixel::Black, Pixel::White, Pixel::White, Pixel::White, Pixel::Black, Pixel::Black, Pixel::White, Pixel::White, Pixel::White
    ]);

    let border = variant.get_border(Side::Left);
    assert_eq!(border.pixels, vec![
        Pixel::Black,
        Pixel::White,
        Pixel::White,
        Pixel::White,
        Pixel::White,
        Pixel::White,
        Pixel::Black,
        Pixel::Black,
        Pixel::White,
        Pixel::Black,
    ]);
}

#[test]
fn test_arrangement_get_neighbours() {
    let mut arrangement = Arrangement::new(3, 3);
    assert!(arrangement.get_neighbours(&Point::new(0, 0)).is_empty());

    arrangement.insert(&Point::new(0, 0), 1951, 0);
    arrangement.insert(&Point::new(1, 0), 2311, 0);
    arrangement.insert(&Point::new(2, 0), 3079, 0);
    arrangement.insert(&Point::new(0, 1), 2729, 0);
    arrangement.insert(&Point::new(1, 1), 1427, 0);
    arrangement.insert(&Point::new(2, 1), 2473, 0);
    arrangement.insert(&Point::new(0, 2), 2971, 0);
    arrangement.insert(&Point::new(1, 2), 1489, 0);
    arrangement.insert(&Point::new(2, 2), 1171, 0);

    assert_eq!(arrangement.get_neighbours(&Point::new(0, 0)), vec![
        Neighbour { tile_id: 2311, sides_to_match: (Side::Left, Side::Right), variant_id: 0 },
        Neighbour { tile_id: 2729, sides_to_match: (Side::Top, Side::Bottom), variant_id: 0 }
    ]);

    assert_eq!(arrangement.get_neighbours(&Point::new(1, 0)), vec![
        Neighbour { tile_id: 1951, sides_to_match: (Side::Right, Side::Left), variant_id: 0 },
        Neighbour { tile_id: 3079, sides_to_match: (Side::Left, Side::Right), variant_id: 0 },
        Neighbour { tile_id: 1427, sides_to_match: (Side::Top, Side::Bottom), variant_id: 0 }
    ]);

    assert_eq!(arrangement.get_neighbours(&Point::new(1, 1)), vec![
        Neighbour { tile_id: 2729, sides_to_match: (Side::Right, Side::Left), variant_id: 0 },
        Neighbour { tile_id: 2311, sides_to_match: (Side::Bottom, Side::Top), variant_id: 0 },
        Neighbour { tile_id: 2473, sides_to_match: (Side::Left, Side::Right), variant_id: 0 },
        Neighbour { tile_id: 1489, sides_to_match: (Side::Top, Side::Bottom), variant_id: 0 }
    ]);

    assert_eq!(arrangement.get_neighbours(&Point::new(2, 2)), vec![
        Neighbour { tile_id: 1489, sides_to_match: (Side::Right, Side::Left), variant_id: 0 },
        Neighbour { tile_id: 2473, sides_to_match: (Side::Bottom, Side::Top), variant_id: 0 }
    ]);
}

#[test]
fn test_arrangement_can_fit_tile() {
    let mut tiles_stack = BunchOfTiles::new();
    tiles_stack.insert(get_tile_2311());
    tiles_stack.insert(get_tile_1427());

    let tile_2311 = tiles_stack.get(2311);
    let tile_1427 = tiles_stack.get(1427);

    let mut arrangment = Arrangement::new(3, 3);

    assert!(arrangment.can_fit_tile(tile_2311.id, tile_2311.variants[0].variant_id, &Point::new(0, 0), &tiles_stack));

    arrangment.insert(&Point::new(0, 0), tile_2311.id, 7);

    assert!(!arrangment.can_fit_tile(tile_2311.id, tile_2311.variants[0].variant_id, &Point::new(0, 0), &tiles_stack));
    assert!(arrangment.can_fit_tile(tile_2311.id, tile_2311.variants[0].variant_id, &Point::new(0, 1), &tiles_stack));

    assert!(arrangment.can_fit_tile(tile_1427.id, 23, &Point::new(0, 1), &tiles_stack));
}

#[test]
fn test_arrangement_fill_yourself() {
    let mut tiles = BunchOfTiles::new();
    let input_tiles = read_input("test_input.txt");
    for t in input_tiles {
        tiles.insert(t);
    }

    if let Some(arrangement) = Arrangement::find(3, &tiles) {
        println!("{}", arrangement);
    }
}