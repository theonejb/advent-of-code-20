use std::collections::HashMap;
use std::env::var;
use std::fmt::{Display, Formatter, Result};
use std::fs::File;
use std::i32::MAX;

use image::{ColorType, GrayImage, Luma};
use image::bmp::BmpEncoder;
use image::imageops::{FilterType, resize};
use std::path::Path;
use std::io::{BufReader, BufRead};


mod tests;

//<editor-fold desc="Geometric Entities">
#[derive(Debug, PartialEq, Copy, Clone, Eq, Hash)]
struct Point {
    x: i32,
    y: i32,
}

impl Display for Point {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

impl Point {
    pub fn new(x: i32, y: i32) -> Point {
        Point { x, y }
    }

    pub fn translate(&self, xt: i32, yt: i32) -> Point {
        Point {
            x: self.x + xt,
            y: self.y + yt,
        }
    }
}

#[derive(Debug, Copy, Clone)]
struct Matrix(
    (i32, i32),
    (i32, i32),
);

impl Matrix {
    pub const X_FLIP: Matrix = Matrix((-1, 0), (0, 1));
    pub const Y_FLIP: Matrix = Matrix((1, 0), (0, -1));
    pub const XY_FLIP: Matrix = Matrix((-1, 0), (0, -1));

    /*
    These matrices are the opposite of what you would expect (the CW_ROTATE matrix is usually
    presented as a CCW rotation transform matrix) because we use a left-handed Cartesian
    coordinate system, where x increases towards the left and y increases downwards.
     */
    pub const CW_ROTATE: Matrix = Matrix((0, -1), (1, 0));
    pub const CCW_ROTATE: Matrix = Matrix((0, 1), (-1, 0));

    pub fn apply(&self, p: &Point) -> Point {
        Point {
            x: p.x * self.0.0 + p.y * self.0.1,
            y: p.x * self.1.0 + p.y * self.1.1,
        }
    }
}

impl Display for Matrix {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "[({} {}) ({} {})]", self.0.0, self.0.1, self.1.0, self.1.1)
    }
}

fn apply_transformations(p: &Point, transformations: &Vec<Matrix>) -> Point {
    let mut p = p.clone();
    for t in transformations.iter() {
        p = t.apply(&p);
    }

    p
}
//</editor-fold>

#[derive(Debug, Copy, Clone, PartialEq)]
enum Pixel { Black, White }

impl Pixel {
    pub fn to_luma(&self) -> Luma<u8> {
        match self {
            Pixel::Black => Luma::from([0x00]),
            Pixel::White => Luma::from([0xff]),
        }
    }
}

type Pixels = HashMap<Point, Pixel>;

type TileId = u32;

#[derive(Debug)]
struct Tile {
    id: TileId,
    pixels: Pixels,
    width: usize,
    height: usize,
    variants: Vec<TileVariant>,
}

impl Tile {
    pub fn set_pixel_value(&mut self, at: &Point, to: &Pixel) {
        self.pixels.insert(at.clone(), to.clone());
    }

    pub fn get_pixel_value(&self, at: &Point) -> Pixel {
        self.pixels.get(at).unwrap().clone()
    }

    pub fn from_input(id: TileId, input: &Vec<String>) -> Tile {
        let mut _self = Tile {
            id,
            pixels: Pixels::new(),
            width: 0,
            height: 0,
            variants: vec![],
        };

        for (y, row) in input.iter().enumerate() {
            for (x, c) in row.chars().enumerate() {
                _self.pixels.insert(
                    Point::new(x as i32, y as i32),
                    match c {
                        '.' => Pixel::Black,
                        '#' => Pixel::White,
                        _ => panic!("Invalid pixel input value")
                    },
                );
            }
        }

        _self.width = input[0].len();
        _self.height = input.len();
        _self.variants = _self.get_all_variants();

        _self
    }

    pub fn get_all_variants(&self) -> Vec<TileVariant> {
        let mut variants = vec![];

        let all_transformations = vec![
            vec![],
            vec![Matrix::CW_ROTATE],
            vec![Matrix::CW_ROTATE, Matrix::CW_ROTATE],
            vec![Matrix::CW_ROTATE, Matrix::CW_ROTATE, Matrix::CW_ROTATE],
            vec![Matrix::X_FLIP],
            vec![Matrix::X_FLIP, Matrix::CW_ROTATE],
            vec![Matrix::X_FLIP, Matrix::CW_ROTATE, Matrix::CW_ROTATE],
            vec![Matrix::X_FLIP, Matrix::CW_ROTATE, Matrix::CW_ROTATE, Matrix::CW_ROTATE],
            // vec![Matrix::Y_FLIP],
            // vec![Matrix::Y_FLIP, Matrix::CW_ROTATE],
            // vec![Matrix::Y_FLIP, Matrix::CW_ROTATE, Matrix::CW_ROTATE],
            // vec![Matrix::Y_FLIP, Matrix::CW_ROTATE, Matrix::CW_ROTATE, Matrix::CW_ROTATE],
            // vec![Matrix::XY_FLIP],
            // vec![Matrix::XY_FLIP, Matrix::CW_ROTATE],
            // vec![Matrix::XY_FLIP, Matrix::CW_ROTATE, Matrix::CW_ROTATE],
            // vec![Matrix::XY_FLIP, Matrix::CW_ROTATE, Matrix::CW_ROTATE, Matrix::CW_ROTATE],
        ];

        for transformations in all_transformations.iter() {
            variants.push(
                TileVariant::from_tile_with_transformations(&self, transformations)
            );
        }

        variants
    }

    pub fn get_variant_with_id(&self, id: VariantId) -> Option<&TileVariant> {
        for v in self.variants.iter() {
            if v.variant_id == id {
                return Some(v);
            }
        }

        None
    }
}

impl Display for Tile {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        writeln!(f, "Tile {}", self.id);

        for y in 0..self.height {
            for x in 0..self.width {
                let p = Point::new(x as i32, y as i32);
                let value = self.get_pixel_value(&p);
                write!(f, "{}", match value {
                    Pixel::Black => ".",
                    Pixel::White => "#"
                });
            }
            writeln!(f);
        }

        Result::Ok(())
    }
}

#[derive(Debug, Copy, Clone, PartialEq)]
enum Side { Left, Top, Right, Bottom }

struct Border {
    pixels: Vec<Pixel>
}

impl Border {
    pub fn new() -> Border {
        Border {
            pixels: vec![]
        }
    }
}

impl PartialEq for Border {
    fn eq(&self, other: &Self) -> bool {
        self.pixels == other.pixels
    }
}

type VariantId = u32;

static mut VARIANT_COUNTER: u32 = 0;

unsafe fn get_next_variant_id() -> VariantId {
    VARIANT_COUNTER += 1;
    VARIANT_COUNTER
}

#[derive(Debug)]
struct TileVariant {
    tile_id: TileId,
    variant_id: VariantId,
    transformations: Vec<Matrix>,
    pixels: Pixels,
    width: usize,
    height: usize,
}

impl TileVariant {
    fn set_pixel_value(&mut self, at: &Point, to: &Pixel) {
        self.pixels.insert(at.clone(), to.clone());
    }

    fn get_pixel_value(&self, at: &Point) -> Pixel {
        self.pixels.get(at).unwrap().clone()
    }

    pub fn from_tile_with_transformations(tile: &Tile, transformations: &Vec<Matrix>) -> TileVariant {
        let variant_id;
        unsafe {
            variant_id = get_next_variant_id();
        }

        let mut _self = TileVariant {
            tile_id: tile.id,
            variant_id,
            transformations: transformations.clone(),
            pixels: Pixels::new(),
            width: tile.width,
            height: tile.height,
        };

        let (mut min_x, mut min_y) = (MAX, MAX);
        let mut temp_pixels = Pixels::new();
        for y in 0.._self.height {
            for x in 0.._self.width {
                let this_point = Point::new(x as i32, y as i32);
                let new_point = apply_transformations(&this_point, transformations);

                if new_point.x < min_x {
                    min_x = new_point.x;
                }
                if new_point.y < min_y {
                    min_y = new_point.y;
                }

                let value = tile.get_pixel_value(&this_point);

                temp_pixels.insert(new_point, value);
            }
        }

        let x_translation = -min_x;
        let y_translation = -min_y;
        for (p, v) in temp_pixels.iter() {
            let p = p.translate(x_translation, y_translation);
            _self.set_pixel_value(&p, v);
        }

        _self
    }

    pub fn write(&self, filename: &str) {
        let mut img = GrayImage::new(self.width as u32, self.height as u32);
        for x in 0..self.width {
            for y in 0..self.height {
                let pixel = self.get_pixel_value(&Point::new(x as i32, y as i32)).to_luma();
                img.put_pixel(x as u32, y as u32, pixel);
            }
        }

        let mut larger_img = resize(&img, 512, 512, FilterType::Nearest);
        larger_img.save(filename);
    }

    pub fn get_border(&self, side: Side) -> Border {
        let (mut x, mut y, dx, dy) = match side {
            Side::Left => (0, 0, 0, 1),
            Side::Top => (0, 0, 1, 0),
            Side::Right => (self.width - 1, 0, 0, 1),
            Side::Bottom => (0, self.height - 1, 1, 0)
        };

        let mut border = Border::new();
        while x < self.width && y < self.height {
            border.pixels.push(self.get_pixel_value(&Point::new(x as i32, y as i32)));
            x += dx;
            y += dy;
        }

        border
    }
}

impl Display for TileVariant {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        writeln!(f, "Tile {} with transformations:", self.tile_id);

        for t in self.transformations.iter() {
            write!(f, "{}", t);
        }

        writeln!(f);

        for y in 0..self.height {
            for x in 0..self.width {
                let p = Point::new(x as i32, y as i32);
                let value = self.get_pixel_value(&p);
                write!(f, "{}", match value {
                    Pixel::Black => ".",
                    Pixel::White => "#"
                });
            }
            writeln!(f);
        }

        Result::Ok(())
    }
}

struct BunchOfTiles {
    tiles: HashMap<TileId, Tile>
}

impl BunchOfTiles {
    pub fn new() -> BunchOfTiles {
        BunchOfTiles { tiles: HashMap::new() }
    }

    pub fn insert(&mut self, tile: Tile) {
        self.tiles.insert(tile.id, tile);
    }

    pub fn get(&self, tile_id: TileId) -> &Tile {
        self.tiles.get(&tile_id).unwrap()
    }
}

#[derive(Clone)]
struct Arrangement {
    width: usize,
    height: usize,
    tiles: HashMap<Point, (TileId, VariantId)>,
}

/*
The sides to match are:
    0: The border side of the neighbour which touches us
    1: Our border side which touches this neighbour
 */
#[derive(Debug, PartialEq)]
struct Neighbour {
    tile_id: TileId,
    variant_id: VariantId,
    sides_to_match: (Side, Side),
}

impl Arrangement {
    pub fn new(width: usize, height: usize) -> Arrangement {
        Arrangement {
            width,
            height,
            tiles: HashMap::new(),
        }
    }

    pub fn get_neighbours(&self, of: &Point) -> Vec<Neighbour> {
        let mut neighbours = vec![];

        for (dx, dy, sides) in [
            (-1, 0, (Side::Right, Side::Left)),
            (0, -1, (Side::Bottom, Side::Top)),
            (1, 0, (Side::Left, Side::Right)),
            (0, 1, (Side::Top, Side::Bottom))
        ].iter() {
            let x = of.x + dx;
            let y = of.y + dy;

            if !(x >= 0 && x < self.width as i32) {
                continue;
            }

            if !(y >= 0 && y < self.height as i32) {
                continue;
            }

            let neighbour_coord = Point::new(x, y);
            if let Some(neighbour_id) = self.tiles.get(&neighbour_coord) {
                neighbours.push(Neighbour {
                    tile_id: neighbour_id.0,
                    variant_id: neighbour_id.1,
                    sides_to_match: sides.clone(),
                });
            }
        }

        neighbours
    }

    pub fn insert(&mut self, at: &Point, tile_id: TileId, variant_id: VariantId) {
        self.tiles.insert(at.clone(), (tile_id, variant_id));
    }

    pub fn is_full(&self) -> bool {
        (self.width * self.height) == self.tiles.len()
    }

    fn has_tile(&self, at: &Point) -> bool {
        self.tiles.get(at).is_some()
    }

    fn first_empty_point(&self) -> Option<Point> {
        for x in 0..self.width {
            for y in 0..self.height {
                let p = Point::new(x as i32, y as i32);
                if !self.has_tile(&p) {
                    return Some(p);
                }
            }
        }

        None
    }

    fn can_fit_tile(&self, tile_id: TileId, variant_id: VariantId, at: &Point, tiles_stack: &BunchOfTiles) -> bool {
        if self.tiles.get(at).is_some() {
            return false;
        }

        let us = tiles_stack.tiles.get(&tile_id).unwrap().get_variant_with_id(variant_id).unwrap();

        let neighbours = self.get_neighbours(at);
        for neighbour in neighbours {
            if let Some(tile) = tiles_stack.tiles.get(&neighbour.tile_id) {
                if let Some(variant) = tile.get_variant_with_id(neighbour.variant_id) {
                    let their_border = variant.get_border(neighbour.sides_to_match.0);
                    let our_border = us.get_border(neighbour.sides_to_match.1);

                    if their_border != our_border {
                        return false;
                    }
                }
            }
        }

        true
    }

    fn fill(arrangement: Arrangement, tiles: &BunchOfTiles, tiles_used: Vec<TileId>, tiles_remaining: Vec<TileId>) -> Option<Arrangement> {
        for _ in 0..arrangement.tiles.len() {
            print!(".");
        }
        println!();

        if tiles_remaining.is_empty() {
            return Some(arrangement);
        }

        let point_to_fill = arrangement.first_empty_point();
        let point_to_fill = point_to_fill.unwrap();

        for tile_id in tiles_remaining.iter() {
            let tile_id = *tile_id;
            let tile = tiles.get(tile_id);

            for variant in tile.variants.iter() {
                if arrangement.can_fit_tile(tile_id, variant.variant_id, &point_to_fill, tiles) {
                    let mut new_arrangement = arrangement.clone();
                    new_arrangement.insert(&point_to_fill, tile_id, variant.variant_id);

                    let mut tiles_used = tiles_used.clone();
                    tiles_used.push(tile_id);

                    let mut new_tiles_remaining: Vec<TileId> = vec![];
                    for tid in tiles_remaining.iter() {
                        if *tid != tile_id {
                            new_tiles_remaining.push(*tid);
                        }
                    }

                    if let Some(arrangement) = Arrangement::fill(new_arrangement, tiles, tiles_used, new_tiles_remaining) {
                        return Some(arrangement);
                    }
                }
            }
        }

        None
    }

    pub fn find(of_size: usize, from: &BunchOfTiles) -> Option<Arrangement> {
        let mut tiles_remaining: Vec<TileId> = vec![];
        for tile_id in from.tiles.keys() {
            tiles_remaining.push(*tile_id);
        }
        Arrangement::fill(
            Arrangement::new(of_size, of_size),
            from,
            vec![],
            tiles_remaining,
        )
    }

    pub fn new_from_solution(of_size: usize, tiles: &BunchOfTiles, solution: Vec<Vec<(TileId, VariantId)>>) -> Arrangement {
        let mut arrangement = Arrangement::new(of_size, of_size);

        for (y, row) in solution.iter().enumerate() {
            for (x, (tile_id, variant_id)) in row.iter().enumerate() {
                let p = Point::new(x as i32, y as i32);
                let tile = tiles.get(*tile_id).get_variant_with_id(*variant_id).unwrap();

                arrangement.insert(&p, tile.tile_id, tile.variant_id);
            }
        }

        arrangement
    }
}

impl Display for Arrangement {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        for y in 0..self.height {
            for x in 0..self.width {
                if let Some(tile) = self.tiles.get(&Point::new(x as i32, y as i32)) {
                    write!(f, "{:5}|{:0>2}\t", tile.0, tile.1);
                } else {
                    write!(f, "-----|--\t");
                }
            }
            writeln!(f);
        }

        writeln!(f)
    }
}

fn read_input(filename: &str) -> Vec<Tile> {
    let mut tiles = vec![];

    let f = File::open(Path::new(filename)).unwrap();
    let mut current_tile_input = vec![];
    let mut current_tile_id = 0;

    for line in BufReader::new(f).lines() {
        let line = line.unwrap();

        if line.starts_with("Tile ") {
            let line_parts: Vec<&str> = line.split(" ").collect();
            let id_part = line_parts[1];
            let id = &id_part[..id_part.len() - 1];
            let id = id.parse::<u32>().unwrap();
            current_tile_id = id;
        } else if line.is_empty() {
            tiles.push(
                Tile::from_input(current_tile_id, &current_tile_input)
            );

            current_tile_id = 0;
            current_tile_input = vec![];
        } else {
            current_tile_input.push(line);
        }
    }

    if current_tile_id != 0 {
        tiles.push(
            Tile::from_input(current_tile_id, &current_tile_input)
        );

        current_tile_id = 0;
        current_tile_input = vec![];
    }

    tiles
}

fn get_part1_solution() -> Vec<Vec<(TileId, VariantId)>> {
    vec![
        vec![(2753, 939), (2539, 802), (3343, 624), (1583, 482), (2309, 349), (2113, 539), (3319, 545), (2711, 1133), (2273, 247), (1663, 1029), (3907, 276), (2593, 891), ],
        vec![(3889, 901), (2017, 32), (1433, 421), (3701, 520), (2381, 1081), (2683, 392), (3989, 586), (1213, 185), (3413, 226), (2297, 24), (2389, 787), (2357, 64), ],
        vec![(2011, 796), (2459, 853), (1483, 637), (1997, 201), (1543, 999), (3433, 81), (2999, 456), (1637, 1128), (2633, 506), (3299, 442), (2609, 928), (1279, 956), ],
        vec![(1171, 42), (3457, 317), (1453, 663), (2393, 645), (3533, 213), (2467, 615), (3167, 933), (3797, 356), (1619, 195), (1667, 108), (2027, 1071), (1721, 325), ],
        vec![(2213, 666), (3389, 217), (3499, 595), (1783, 393), (1399, 167), (1597, 331), (2953, 857), (1627, 98), (3137, 149), (1801, 691), (2731, 409), (2129, 626), ],
        vec![(3761, 812), (2267, 525), (1021, 757), (1873, 133), (1303, 554), (3329, 701), (2549, 821), (1907, 1149), (2687, 532), (2791, 490), (3203, 1051), (1097, 407), ],
        vec![(1499, 457), (1201, 880), (2749, 653), (1559, 1047), (1319, 971), (1609, 885), (1787, 125), (3709, 144), (3527, 949), (1753, 07), (1069, 1036), (1741, 76), ],
        vec![(1709, 564), (1049, 601), (1699, 10), (3547, 570), (1423, 153), (2971, 1114), (2671, 681), (2707, 721), (2819, 1057), (3719, 1100), (2237, 465), (1823, 283), ],
        vec![(1291, 963), (3323, 180), (2179, 705), (1009, 978), (3877, 257), (3181, 842), (1381, 1096), (2087, 344), (3041, 905), (3109, 268), (2659, 985), (1307, 769), ],
        vec![(1747, 291), (1091, 1077), (3947, 91), (3833, 240), (3671, 299), (2003, 713), (3169, 435), (1163, 383), (2797, 1021), (1867, 746), (1289, 744), (3541, 307), ],
        vec![(1019, 1003), (1427, 762), (1367, 913), (2099, 784), (1879, 504), (1487, 872), (1283, 34), (2251, 53), (3853, 429), (1987, 251), (2411, 172), (2287, 117), ],
        vec![(3881, 1138), (2039, 829), (2663, 364), (3593, 1107), (2239, 1016), (3049, 475), (2833, 677), (2063, 833), (3847, 729), (2503, 71), (3779, 371), (2383, 578), ],
    ]
}

fn main() {
    let mut tiles = BunchOfTiles::new();
    let input_tiles = read_input("input.txt");
    for t in input_tiles {
        tiles.insert(t);
    }

    if let Some(arrangement) = Arrangement::find(12, &tiles) {
        println!("{}", arrangement);
    } else {
        println!("No arrangement found");
    }

    let mut arrangement = Arrangement::new_from_solution(12, &tiles, get_part1_solution());
    println!("{}", arrangement);
}
