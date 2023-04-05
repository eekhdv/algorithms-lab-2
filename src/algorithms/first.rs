use super::structs::{point::Point, rect::Rect};

pub fn count_rect_for_point(x: i32, y: i32, rects_p: Vec<((i32, i32), (i32, i32))>) -> u32 {
    let rects = Rect::from_vec(rects_p);
    let p = Point::new(x, y);

    let mut res = 0;
    rects.iter().for_each(|x| {
        if x.is_in(&p) {
            res += 1;
        }
    });
    res
}

impl Rect {
    fn is_in(&self, p: &Point) -> bool {
        (self.lower_l.x <= p.x && p.x <= self.upper_r.x)
            && (self.lower_l.x <= p.y && p.y <= self.upper_r.y)
    }
}
