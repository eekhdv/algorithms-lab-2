use crate::algorithms::structs::{point::Point, rect::Rect};

pub trait SecondLabSolution {
    fn count_rect_for_point(p: Point, rects: Vec<Rect>) -> u32;
}
