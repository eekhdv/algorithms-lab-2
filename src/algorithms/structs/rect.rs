use super::point::Point;

pub struct Rect {
    pub lower_l: Point,
    pub upper_r: Point,
}

impl Rect {
    fn new(lower_l: Point, upper_r: Point) -> Self {
        Self {
            lower_l: (lower_l),
            upper_r: (upper_r),
        }
    }

    pub fn from_vec(rect_coords: Vec<(Point, Point)>) -> Vec<Rect> {
        let mut res_vec: Vec<Self> = Vec::with_capacity(rect_coords.capacity());
        for rect in rect_coords {
            res_vec.push(Self::new(rect.0, rect.1));
        }
        res_vec
    }
}

impl From<(Point, Point)> for Rect {
    fn from(value: (Point, Point)) -> Self {
        Self {
            lower_l: (value.0),
            upper_r: (value.1),
        }
    }
}
