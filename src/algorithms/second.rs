use super::structs::{
    compressed::{index::CompressedIndex, map::CompressedMap},
    point::Point,
    rect::Rect,
};
use super::traits::lab::LabSolution;

#[derive(Debug)]
pub struct AlgorithmOnMap;

impl LabSolution<&Vec<Rect>, CompressedMap> for AlgorithmOnMap {
    fn count_rect_for_point(points: &Vec<Point>, rects: &Vec<Rect>) -> Vec<i32> {
        let res;
        if rects.is_empty() {
            res = vec![0; points.len()];
        } else {
            let prep_data = Self::prepare_data(rects).unwrap();
            res = Self::run_with_prepared(prep_data, points);
        }
        res
    }

    fn run_with_prepared(prepared_data: CompressedMap, points: &Vec<Point>) -> Vec<i32> {
        let mut res: Vec<i32> = Vec::with_capacity(points.len());
        let c_map = prepared_data;
        for p in points {
            res.push(Self::find_point_in_map(&c_map, &p) as i32);
        }
        res
    }

    fn prepare_data(rects: &Vec<Rect>) -> Option<CompressedMap> {
        let (mut c_idx, mut c_idy): (CompressedIndex, CompressedIndex) =
            CompressedIndex::from_rects(&rects);
        c_idx.compress();
        c_idy.compress();
        let mut c_map = CompressedMap::from((&c_idx, &c_idy));
        c_map.fill_with(&rects);
        Some(c_map)
    }
}

impl AlgorithmOnMap {
    fn find_point_in_map(map: &CompressedMap, p: &Point) -> u32 {
        if let Some(ans) = map.get_value(p) {
            ans
        } else {
            0
        }
    }
}
