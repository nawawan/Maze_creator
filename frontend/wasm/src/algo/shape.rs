use std::ops::{Add, Mul, Sub};

use web_sys::CanvasRenderingContext2d;

#[derive(Hash, Eq, PartialEq)]
pub struct Point<T> {
    pub x: T,
    pub y: T,
}

pub struct Line<T> {
    pub from: Point<T>,
    pub to: Point<T>,
}

impl<T> Point<T>
where
    T: Mul<Output = T> + Add<Output = T> + Sub<Output = T> + Copy,
{
    pub fn new(x: T, y: T) -> Self {
        Point { x: x, y: y }
    }

    pub fn flatten(&self, width: T) -> T {
        self.x * width + self.y
    }
}

impl Point<usize> {
    pub fn from_1d_index(idx: usize, width: usize) -> Self {
        Point {
            x: idx / width,
            y: idx % width,
        }
    }
    pub fn shift_horizontal(&mut self) {
        self.y += 1;
    }
    pub fn shift_vertical(&mut self) {
        self.x += 1;
    }
}

impl From<Point<usize>> for Point<f64> {
    fn from(point: Point<usize>) -> Self {
        Point {
            x: point.x as f64,
            y: point.y as f64,
        }
    }
}

impl<T> Line<T> {
    pub fn new(from: Point<T>, to: Point<T>) -> Self {
        Line { from: from, to: to }
    }
}

impl Line<f64> {
    pub fn draw(&self, ctx: &CanvasRenderingContext2d) {
        ctx.move_to(self.from.x, self.from.y);
        ctx.line_to(self.to.x, self.to.y);
    }
}
