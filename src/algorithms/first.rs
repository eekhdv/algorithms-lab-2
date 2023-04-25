use super::structs::{point::Point, rect::Rect};
use super::traits::lab::LabSolution;

#[derive(Debug)]
pub struct AlgorithmBruteForce;

impl LabSolution<&Vec<Rect>, Vec<Rect>> for AlgorithmBruteForce {
    fn count_rect_for_point(points: &Vec<Point>, rects: &Vec<Rect>) -> Vec<i32> {
        let mut res: Vec<i32> = Vec::with_capacity(points.len());
        for p in points {
            let mut r = 0;
            rects.iter().for_each(|x| {
                if x.is_in(&p) {
                    r += 1;
                }
            });
            res.push(r);
        }
        res
    }
    fn run_with_prepared(rects: &Vec<Rect>, p: &Vec<Point>) -> Vec<i32> {
        Self::count_rect_for_point(p, rects)
    }

    fn find_single_point(rects: &Vec<Rect>, p: &Point) -> i32 {
        let mut r = 0;
        rects.iter().for_each(|x| {
            if x.is_in(&p) {
                r += 1;
            }
        });
        r
    }
}

impl Rect {
    fn is_in(&self, p: &Point) -> bool {
        (self.lower_l.x <= p.x && p.x <= self.upper_r.x)
            && (self.lower_l.y <= p.y && p.y <= self.upper_r.y)
    }
}
