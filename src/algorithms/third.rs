use super::structs::compressed::index::CompressedIndex;
use super::structs::tree::PersistentTree;
use super::structs::{point::Point, rect::Rect};
use super::traits::lab::LabSolution;

#[derive(Debug)]
pub struct AlgorithmOnPersistenTree;

impl
    LabSolution<
        &Vec<Rect>,
        (
            (Vec<PersistentTree>, CompressedIndex),
            CompressedIndex,
            CompressedIndex,
        ),
    > for AlgorithmOnPersistenTree
{
    fn count_rect_for_point(points: &Vec<Point>, rects: &Vec<Rect>) -> Vec<i32> {
        let res;
        if rects.is_empty() {
            res = vec![0; points.len()];
        } else {
            res = Self::run_with_prepared(Self::prepare_data(rects).unwrap(), points);
        }
        res
    }

    fn run_with_prepared(
        prepared_data: (
            (Vec<PersistentTree>, CompressedIndex),
            CompressedIndex,
            CompressedIndex,
        ),
        points: &Vec<Point>,
    ) -> Vec<i32> {
        let mut res: Vec<i32> = Vec::with_capacity(points.len());
        let ((seg_tree, c_idr), c_idx, c_idy) = prepared_data;
        for p in points {
            res.push(PersistentTree::query(&seg_tree, p, &c_idr, &c_idx, &c_idy));
        }
        res
    }

    fn prepare_data(
        rects: &Vec<Rect>,
    ) -> Option<(
        (Vec<PersistentTree>, CompressedIndex),
        CompressedIndex,
        CompressedIndex,
    )> {
        let (mut c_idx, mut c_idy): (CompressedIndex, CompressedIndex) =
            CompressedIndex::from_rects(&rects);
        c_idx.compress();
        c_idy.compress();

        Some((
            PersistentTree::build_with(&c_idx, &c_idy, rects),
            c_idx,
            c_idy,
        ))
    }
}
