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

#[test]
fn test_tile_from_input() {
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

    let tile = Tile::from_input(2311, &input);

    assert_eq!(tile.get_pixel_value(&Point::new(0, 0)), Pixel::Black);
    assert_eq!(tile.get_pixel_value(&Point::new(0, 1)), Pixel::White);
    assert_eq!(tile.get_pixel_value(&Point::new(9, 9)), Pixel::White);
    assert_eq!(tile.get_pixel_value(&Point::new(9, 8)), Pixel::Black);
    assert_eq!(tile.width, 10);
    assert_eq!(tile.height, 10);
}

#[test]
fn test_tile_variant_from_tile_with_transformations() {
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

    let tile = Tile::from_input(2311, &input);

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

    let tile = Tile::from_input(2311, &input);

    for (i, variant) in tile.variants.iter().enumerate() {
        variant.write(&format!("test_images/tile_{}_variant_{}.bmp", tile.id, i));
    }
}