use super::point::Point;

pub struct Rect {
    pub lower_l: Point,
    pub upper_r: Point,
}

impl Rect {
    fn new(lower_l: (i32, i32), upper_r: (i32, i32)) -> Self {
        Self {
            lower_l: (Point::new(lower_l.0, lower_l.1)),
            upper_r: (Point::new(upper_r.0, upper_r.1)),
        }
    }

    pub fn from_vec(rect_coords: Vec<((i32, i32), (i32, i32))>) -> Vec<Rect> {
        let mut res_vec: Vec<Self> = Vec::with_capacity(rect_coords.capacity());
        for rect in rect_coords {
            res_vec.push(Self::new(rect.0, rect.1));
        }
        res_vec
    }
}
