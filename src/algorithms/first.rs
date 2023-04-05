use super::structs::{rect::Rect, point::Point};

pub fn count_rect_for_point(x: i32, y: i32, rects_p: Vec<((i32, i32), (i32, i32))>) -> u32 {
    let rects = Rect::from_vec(rects_p);
    let p = Point::new(x, y);

    0
}
