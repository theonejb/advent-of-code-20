use std::collections::HashMap;
use std::fmt::{Display, Formatter, Result};
use std::fs::File;
use std::i32::MAX;
use image::{GrayImage, Luma, ColorType};
use image::bmp::BmpEncoder;
use image::imageops::{resize, FilterType};

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
            y: self.y + yt
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
    variants: Vec<TileVariant>
}

impl Tile {
    pub fn set_pixel_value(&mut self, at: &Point, to: &Pixel) {
        self.pixels.insert(at.clone(), to.clone());
    }

    pub fn get_pixel_value(&self, at: &Point) -> Pixel {
        self.pixels.get(at).unwrap().clone()
    }

    pub fn from_input(id: TileId, input: &Vec<String>) -> Tile {
        let mut _self = Tile{
            id,
            pixels: Pixels::new(),
            width: 0,
            height: 0,
            variants: vec![]
        };

        for (y, row) in input.iter().enumerate() {
            for (x, c) in row.chars().enumerate() {
                _self.pixels.insert(
                    Point::new(x as i32, y as i32),
                    match c {
                        '.' => Pixel::Black,
                        '#' => Pixel::White,
                        _ => panic!("Invalid pixel input value")
                    }
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

            vec![Matrix::Y_FLIP],
            vec![Matrix::Y_FLIP, Matrix::CW_ROTATE],
            vec![Matrix::Y_FLIP, Matrix::CW_ROTATE, Matrix::CW_ROTATE],
            vec![Matrix::Y_FLIP, Matrix::CW_ROTATE, Matrix::CW_ROTATE, Matrix::CW_ROTATE],

            vec![Matrix::XY_FLIP],
            vec![Matrix::XY_FLIP, Matrix::CW_ROTATE],
            vec![Matrix::XY_FLIP, Matrix::CW_ROTATE, Matrix::CW_ROTATE],
            vec![Matrix::XY_FLIP, Matrix::CW_ROTATE, Matrix::CW_ROTATE, Matrix::CW_ROTATE],
        ];

        for transformations in all_transformations.iter() {
            variants.push(
                TileVariant::from_tile_with_transformations(&self, transformations)
            );
        }

        variants
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

#[derive(Debug)]
struct TileVariant {
    tile_id: TileId,
    transformations: Vec<Matrix>,
    pixels: Pixels,
    width: usize,
    height: usize
}

impl TileVariant {
    fn set_pixel_value(&mut self, at: &Point, to: &Pixel) {
        self.pixels.insert(at.clone(), to.clone());
    }

    fn get_pixel_value(&self, at: &Point) -> Pixel {
        self.pixels.get(at).unwrap().clone()
    }

    pub fn from_tile_with_transformations(tile: &Tile, transformations: &Vec<Matrix>) -> TileVariant {
        let mut _self = TileVariant{
            tile_id: tile.id,
            transformations: transformations.clone(),
            pixels: Pixels::new(),
            width: tile.width,
            height: tile.height
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

fn main() {
    println!("Hello, world!");
}
