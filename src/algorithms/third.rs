use super::structs::compressed::index::CompressedIndex;
use super::structs::tree::PersistentTree;
use super::structs::{point::Point, rect::Rect};
use super::traits::lab::LabSolution;

#[derive(Debug)]
pub struct AlgorithmOnPersistenTree;

impl LabSolution for AlgorithmOnPersistenTree {
    fn count_rect_for_point(points: &Vec<Point>, rects: &Vec<Rect>) -> Vec<i32> {
        let mut res: Vec<i32> = Vec::with_capacity(points.len());
        if rects.is_empty() {
            res = vec![0; points.len()];
        } else {
            let (mut c_idx, mut c_idy): (CompressedIndex, CompressedIndex) =
                CompressedIndex::from_rects(&rects);
            c_idx.compress();
            c_idy.compress();

            let (seg_tree, c_idr) = PersistentTree::build_with(&c_idx, &c_idy, rects);

            for p in points {
                res.push(PersistentTree::query(&seg_tree, p, &c_idr, &c_idx, &c_idy));
            }
        }
        res
    }
}
