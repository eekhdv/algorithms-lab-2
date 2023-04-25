use crate::algorithms::structs::{point::Point, rect::Rect};

pub trait LabSolution<T, U> {
    fn count_rect_for_point(p: &Vec<Point>, rects: &Vec<Rect>) -> Vec<i32>;
    fn prepare_data(_idata: T) -> Option<U> {
        None
    }
    fn find_single_point(prepared_data: &U, p: &Point) -> i32;
    fn run_with_prepared(prepared_data: &U, p: &Vec<Point>) -> Vec<i32>;
}
