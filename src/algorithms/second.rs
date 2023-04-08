use super::structs::{point::Point, rect::Rect};
use itertools::Itertools;

pub fn count_rect_for_point(p: Point, rects: Vec<Rect>) -> u32 {
    let (xs, ys): (Vec<i32>, Vec<i32>) = compress_points(&rects);

    0
}

fn compress_points(r: &Vec<Rect>) -> (Vec<i32>, Vec<i32>) {
    let (mut c_x, mut c_y): (Vec<i32>, Vec<i32>) = get_coords_from_rect(&r);
    c_x.sort();
    c_y.sort();
    c_x = c_x.into_iter().unique().collect();
    c_y = c_y.into_iter().unique().collect();
    (c_x.to_owned(), c_y.to_owned())
}

fn create_compressed_map() -> Vec<Vec<u32>> {
    // let mut c_map: Vec<Vec<i32>> = Vec::with_capacity(compressed_x.capacity());
    // for _i in 0..compressed_x.len() {
    //     c_map.push(Vec::with_capacity(compressed_y.capacity()));
    // }

    unimplemented!()
}

fn get_coords_from_rect(r: &Vec<Rect>) -> (Vec<i32>, Vec<i32>) {
    let mut c_x = Vec::new();
    let mut c_y = Vec::new();
    r.iter().for_each(|r| {
        c_x.push(r.lower_l.x);
        c_y.push(r.lower_l.y);
        c_x.push(r.upper_r.x);
        c_y.push(r.upper_r.y);
    });
    (c_x.to_owned(), c_y.to_owned())
}

