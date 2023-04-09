use super::point::Point;

pub struct Rect {
    pub lower_l: Point,
    pub upper_r: Point,
}

impl Rect {
    pub fn new(lower_l: Point, upper_r: Point) -> Self {
        Self {
            lower_l: (lower_l),
            upper_r: (upper_r),
        }
    }
}
