use crate::algorithms::structs::{point::Point, rect::Rect};

pub trait LabSolution<T, U> {
    fn count_rect_for_point(p: &Vec<Point>, rects: &Vec<Rect>) -> Vec<i32>;
    fn prepare_data(_: T) -> Option<U> { None }
}
