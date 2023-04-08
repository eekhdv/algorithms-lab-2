use crate::algorithms::structs::{rect::Rect, point::Point};

pub trait SecondLabSolution {
    fn count_rect_for_point(p: Point, rects: Vec<Rect>) -> u32;
}
