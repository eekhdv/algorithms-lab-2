use super::structs::{point::Point, rect::Rect};
use super::traits::lab::LabSolution;

#[derive(Debug)]
pub struct AlgorithmBruteForce;

impl LabSolution for AlgorithmBruteForce {
    fn count_rect_for_point(points: &Vec<Point>, rects: &Vec<Rect>) {
        for p in points {
            let mut res = 0;
            rects.iter().for_each(|x| {
                if x.is_in(&p) {
                    res += 1;
                }
            });
            print!("{res} ");
        }
        println!();
    }
}

impl Rect {
    fn is_in(&self, p: &Point) -> bool {
        (self.lower_l.x <= p.x && p.x <= self.upper_r.x)
            && (self.lower_l.y <= p.y && p.y <= self.upper_r.y)
    }
}
