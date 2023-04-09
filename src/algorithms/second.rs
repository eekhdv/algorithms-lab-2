use super::structs::{
    compressed::{CompressedIndex, CompressedMap},
    point::Point,
    rect::Rect,
};
use super::traits::lab::SecondLabSolution;

#[derive(Debug)]
struct AlgorithmOnMap;

impl SecondLabSolution for AlgorithmOnMap {
    fn count_rect_for_point(p: Point, rects: Vec<Rect>) -> u32 {
        let (mut c_idx, mut c_idy): (CompressedIndex, CompressedIndex) =
            CompressedIndex::from_rects(&rects);
        c_idx.compress();
        c_idy.compress();
        let mut c_map = CompressedMap::from((&c_idx, &c_idy));
        c_map.fill_with(&rects);
        let ans = Self::find_point_in_map(&c_map, &p);
        ans
    }
}

impl AlgorithmOnMap {
    fn find_point_in_map(map: &CompressedMap, p: &Point) -> u32 {
        map.get_value(p)
    }
}
