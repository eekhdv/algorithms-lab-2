use super::structs::{point::Point, rect::Rect};
use super::traits::lab::SecondLabSolution;

#[derive(Debug)]
pub struct AlgorithmBruteForce;

impl SecondLabSolution for AlgorithmBruteForce {
    fn count_rect_for_point(p: &Point, rects: &Vec<Rect>) -> u32 {
        let mut res = 0;
        rects.iter().for_each(|x| {
            if x.is_in(&p) {
                res += 1;
            }
        });
        res
    }
}

impl Rect {
    fn is_in(&self, p: &Point) -> bool {
        (self.lower_l.x <= p.x && p.x <= self.upper_r.x)
            && (self.lower_l.x <= p.y && p.y <= self.upper_r.y)
    }
}
