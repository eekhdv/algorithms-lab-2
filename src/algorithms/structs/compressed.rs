pub mod index {
    use crate::algorithms::structs::rect::Rect;
    use itertools::Itertools;

    #[derive(Debug, Clone)]
    pub struct CompressedIndex {
        c_index: Vec<i32>,
    }

    impl CompressedIndex {
        pub fn new(v: Vec<i32>) -> Self {
            Self { c_index: v }
        }
        pub fn get_index_of(&self, val: &i32) -> i32 {
            crate::utils::lower_bound(&self.c_index, val.clone())
        }
        pub fn len(&self) -> usize {
            self.c_index.len()
        }
        pub fn capacity(&self) -> usize {
            self.c_index.capacity()
        }
        pub fn from_rects(r: &Vec<Rect>) -> (Self, Self) {
            let mut c_x = Vec::with_capacity(r.len() * 3);
            let mut c_y = Vec::with_capacity(r.len() * 3);
            r.iter().for_each(|r| {
                c_x.push(r.lower_l.x);
                c_y.push(r.lower_l.y);
                c_x.push(r.upper_r.x);
                c_x.push(r.upper_r.x + 1);
                c_y.push(r.upper_r.y);
                c_y.push(r.upper_r.y + 1);
            });
            (Self::new(c_x), Self::new(c_y))
        }

        pub fn compress(&mut self) {
            self.c_index = self.c_index.clone().into_iter().sorted().dedup().collect();
            self.c_index.shrink_to_fit();
        }
    }
}

pub mod map {
    use std::ops::Deref;

    use super::index::CompressedIndex;
    use crate::algorithms::structs::{point::Point, rect::Rect};
    #[derive(Debug)]
    pub struct CompressedMap {
        c_map: Vec<Vec<u32>>,
        c_idx: CompressedIndex,
        c_idy: CompressedIndex,
    }

    impl Deref for CompressedMap {
        type Target = Vec<Vec<u32>>;
        fn deref(&self) -> &Self::Target {
            &self.c_map
        }
    }

    impl CompressedMap {
        pub fn fill_with(&mut self, rects: &Vec<Rect>) {
            for r in rects {
                let ll = &r.lower_l;
                let ur = &r.upper_r;
                let ll = Point::new(
                    self.c_idx.get_index_of(&ll.x),
                    self.c_idy.get_index_of(&ll.y),
                );
                let ur = Point::new(
                    self.c_idx.get_index_of(&ur.x),
                    self.c_idy.get_index_of(&ur.y),
                );
                for x in ll.x..=ur.x {
                    for y in ll.y..=ur.y {
                        self.c_map[x as usize][y as usize] += 1;
                    }
                }
            }
        }

        pub fn get_value(&self, p: &Point) -> Option<u32> {
            let id_x = self.c_idx.get_index_of(&p.x);
            let id_y = self.c_idy.get_index_of(&p.y);
            if id_x == -1 || id_y == -1 {
                None
            } else {
                Some(self.c_map[id_x as usize][id_y as usize])
            }
        }
    }

    impl From<(&CompressedIndex, &CompressedIndex)> for CompressedMap {
        fn from(value: (&CompressedIndex, &CompressedIndex)) -> Self {
            let (c_x, c_y) = value;
            let c_map = vec![0; c_y.len() * c_x.len()];
            let c_map: Vec<Vec<u32>> = c_map
                .chunks(c_y.len())
                .map(|chunk| chunk.to_vec())
                .collect::<Vec<_>>();
            Self {
                c_map: (c_map.to_owned()),
                c_idx: c_x.clone(),
                c_idy: c_y.clone(),
            }
        }
    }
}
