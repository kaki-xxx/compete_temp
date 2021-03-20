use cargo_snippet::snippet;

use seg_tree::{SegTree, Monoid};

#[snippet("_segtree", prefix = "use seg_tree::{SegTree, Monoid};")]
mod seg_tree {
    use std::ops::Bound::{Included, Excluded, Unbounded};
    use std::ops::RangeBounds;
    use std::ops::Index;

    pub struct SegTree<M: Monoid> {
        len: usize,
        node: Vec<M>,
    }

    pub trait Monoid: Clone + Copy {
        const E: Self;
        fn op(self, other: Self) -> Self;
    }

    impl<M: Monoid> SegTree<M> {
        pub fn build(v: &Vec<M>) -> Self {
            let len = v.len();
            let mut node = vec![M::E; 2*len];
            for i in 0..len {
                node[len+i] = v[i];
            }
            for i in (1..len).rev() {
                node[i] = node[i << 1].op(node[i << 1 | 1]);
            }
            SegTree { len, node }
        }

        pub fn set(&mut self, mut i: usize, v: M) {
            i += self.len;
            self.node[i] = v;
            while i > 0 {
                i >>= 1;
                self.node[i] = self.node[i << 1].op(self.node[i << 1 | 1]);
            }
        }

        pub fn fold<R: RangeBounds<usize>>(&mut self, range: R) -> M {
            let mut l = match range.start_bound() {
                Included(&s) => s,
                Excluded(&s) => s + 1,
                Unbounded => 0,
            };
            let mut r = match range.end_bound() {
                Included(&s) => s + 1,
                Excluded(&s) => s,
                Unbounded => self.len,
            };
            l += self.len; r += self.len;
            let mut ret_l = M::E;
            let mut ret_r = M::E;
            while l < r {
                if l & 1 != 0 {
                    ret_l = ret_l.op(self.node[l]);
                    l += 1;
                }
                if r & 1 != 0 {
                    r -= 1;
                    ret_r = self.node[r].op(ret_r);
                }
                l >>= 1; r >>= 1;
            }
            ret_l.op(ret_r)
        }
    }

    impl<M: Monoid + Clone + Copy> Index<usize> for SegTree<M> {
        type Output = M;
        fn index(&self, i: usize) -> &Self::Output {
            &self.node[self.len + i]
        }
    }
}
