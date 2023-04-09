pub struct Point {
    pub x: i32,
    pub y: i32,
}

impl Point {
    pub fn new(x: i32, y: i32) -> Self {
        Self { x: (x), y: (y) }
    }

    pub fn from_tup(p: (i32, i32)) -> Self {
        Self::new(p.0, p.1)
    }
}
