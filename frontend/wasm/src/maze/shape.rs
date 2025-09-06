pub struct Point {
    pub x: f64,
    pub y: f64,
}

pub struct Line {
    pub from: Point,
    pub to: Point,
}

impl Point {
    pub fn new(x: f64, y: f64) -> Self {
        Point {
            x: x,
            y: y
        }
    }
}

impl Line {
    pub fn new(from: Point, to: Point) -> Self {
        Line {
            from: from,
            to: to,
        }
    }
}