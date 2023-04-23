use super::structs::compressed::index::CompressedIndex;
use super::structs::{point::Point, rect::Rect};
use super::traits::lab::LabSolution;

#[derive(Debug)]
pub struct AlgorithmOnPersistenTree;
impl LabSolution for AlgorithmOnPersistenTree {
    fn count_rect_for_point(points: &Vec<Point>, rects: &Vec<Rect>) {
        if rects.is_empty() {
            print!("Here is no rectangles");
        } else {
            let (mut c_idx, mut c_idy): (CompressedIndex, CompressedIndex) =
                CompressedIndex::from_rects(&rects);
            c_idx.compress();
            c_idy.compress();

            let (seg_tree, c_idr) = PersistentTree::build_with(&c_idx, &c_idy, rects);

            for p in points {
                print!(
                    "{} ",
                    PersistentTree::query(&seg_tree, p, &c_idr, &c_idx, &c_idy)
                );
            }
        }
        println!();
    }
}

#[derive(Debug, Clone)]
struct Node {
    val: i32,
    left: Option<Box<Node>>,
    right: Option<Box<Node>>,
}

impl Node {
    fn new(val: i32) -> Self {
        Node {
            val,
            left: None,
            right: None,
        }
    }

    fn insert(&self, l: i32, r: i32, val: i32, lb: i32, rb: i32) -> Box<Node> {
        let mut new_node = self.clone();
        if l >= rb || r <= lb {
            return Box::new(new_node);
        }
        if l <= lb && rb <= r {
            new_node.val += val;
            return Box::new(new_node);
        }
        let mid = (lb + rb) / 2;

        if self.left.is_none() {
            new_node.left = Some(Box::new(Node::new(0)));
        }
        if let Some(left_child) = &new_node.left {
            new_node.left = Some(left_child.insert(l, r, val, lb, mid));
        }
        if self.right.is_none() {
            new_node.right = Some(Box::new(Node::new(0)));
        }
        if let Some(right_child) = &new_node.right {
            new_node.right = Some(right_child.insert(l, r, val, mid, rb));
        }
        Box::new(new_node)
    }

    fn sum(&self, _l: i32, _r: i32, c_y: i32) -> i32 {
        let l = _l;
        let r = _r;
        let cur_node = Box::new(self.clone());
        if r - l == 1 {
            return cur_node.val;
        }

        let mid = (l + r) / 2;
        if c_y < mid {
            if let Some(l_child) = cur_node.left {
                return cur_node.val + l_child.sum(l, mid, c_y);
            } else {
                return cur_node.val;
            }
        } else {
            if let Some(r_child) = cur_node.right {
                return cur_node.val + r_child.sum(mid, r, c_y);
            } else {
                return cur_node.val;
            }
        }
    }
}

#[derive(Debug, Clone)]
struct PersistentTree {
    root: Option<Box<Node>>,
}

impl PersistentTree {
    fn new() -> Self {
        PersistentTree { root: None }
    }

    fn insert(&self, l: i32, r: i32, val: i32, lb: i32, rb: i32) -> Self {
        let mut new_root = self.root.clone();
        if self.root.is_none() {
            new_root = Some(Box::new(Node::new(0)));
        }
        if let Some(new_r) = new_root {
            new_root = Some(new_r.insert(l, r, val, lb, rb));
        };
        PersistentTree { root: new_root }
    }

    fn query(
        seg_tree: &Vec<Self>,
        p: &Point,
        c_idr: &CompressedIndex,
        c_idx: &CompressedIndex,
        c_idy: &CompressedIndex,
    ) -> i32 {
        let idx = c_idx.get_index_of(&p.x);
        let idy = c_idy.get_index_of(&p.y);
        let idr = c_idr.get_index_of(&(idx as i32));
        if idr >= 0 {
            seg_tree[idr as usize].sum(0, c_idy.len() as i32, idy as i32)
        } else {
            0
        }
    }

    fn sum(&self, _l: i32, _r: i32, c_y: i32) -> i32 {
        self.root.as_ref().unwrap().sum(_l, _r, c_y)
    }

    fn build_with(
        c_idx: &CompressedIndex,
        c_idy: &CompressedIndex,
        rects: &Vec<Rect>,
    ) -> (Vec<Self>, CompressedIndex) {
        #[derive(Debug)]
        enum PointPos {
            UP,
            DOWN,
        }
        #[derive(Debug)]
        struct PointToAdd {
            c_x: i32,
            c_yd: i32,
            c_yu: i32,
            pos: PointPos,
        }

        let mut points_to_add: Vec<PointToAdd> = Vec::with_capacity(rects.capacity());

        for rect in rects {
            let y_up = c_idy.get_index_of(&(rect.upper_r.y + 1));
            let y_down = c_idy.get_index_of(&rect.lower_l.y);
            points_to_add.push(PointToAdd {
                c_x: (c_idx.get_index_of(&rect.lower_l.x)),
                c_yd: (y_down),
                c_yu: (y_up),
                pos: (PointPos::DOWN),
            });
            points_to_add.push(PointToAdd {
                c_x: (c_idx.get_index_of(&(rect.upper_r.x + 1))),
                c_yd: (y_down),
                c_yu: (y_up),
                pos: (PointPos::UP),
            });
        }
        points_to_add.sort_by_key(|k| k.c_x);

        let mut roots: Vec<Self> = Vec::with_capacity(c_idx.capacity());
        let mut root = PersistentTree::new();
        let mut prev_point = points_to_add.first().unwrap().c_x;
        let mut c_rx: Vec<i32> = Vec::with_capacity(c_idx.capacity());

        for ref point_to_add in points_to_add {
            if point_to_add.c_x != prev_point {
                roots.push(root.clone());
                c_rx.push(prev_point as i32);
                prev_point = point_to_add.c_x;
            }
            root = root.insert(
                point_to_add.c_yd as i32,
                point_to_add.c_yu as i32,
                match point_to_add.pos {
                    PointPos::UP => -1,
                    PointPos::DOWN => 1,
                },
                0,
                c_idy.len() as i32,
            );
        }
        c_rx.push(prev_point as i32);
        roots.push(root.clone());
        let c_rx = CompressedIndex::new(c_rx);
        (roots, c_rx)
    }
}
