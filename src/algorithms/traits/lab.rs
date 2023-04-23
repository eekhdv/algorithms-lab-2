use crate::algorithms::structs::{point::Point, rect::Rect};

pub trait LabSolution {
    fn count_rect_for_point(p: &Vec<Point>, rects: &Vec<Rect>) -> Vec<i32>;
}
